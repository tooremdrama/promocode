use poem_openapi::{Object, Union};
use crate::promocode_protocol::PromoCodeRequest;
use crate::{extern_api};//, database::*};
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

enum MsgRestrictionError {
    MsgPromocodeNotExists,
    MsgRestrictionDateRangeInvalid,
    MsgRestrictionAgeRangeInvalid,
    MsgRestrictionAgeInvalid,
    MsgRestrictionMeteoInvalid,
    MsgRestrictionTownInvalid,
}

impl MsgRestrictionError {
    fn as_str(&self) -> &'static str {
        match self {
            MsgRestrictionError::MsgPromocodeNotExists => "This promo code doesn't exist.",
            MsgRestrictionError::MsgRestrictionDateRangeInvalid => "Invalid date restriction. Must be : after date <= today date <= before date.",
            MsgRestrictionError::MsgRestrictionAgeRangeInvalid => "Invalid age restriction. Must be : lt <= age <= gt.",
            MsgRestrictionError::MsgRestrictionAgeInvalid => "Invalid age restriction. Must be equal to eq in promocode.",
            MsgRestrictionError::MsgRestrictionMeteoInvalid => "Invalid meteo restriction.",
            MsgRestrictionError::MsgRestrictionTownInvalid => "Invalid town."
        }
    }
}

#[derive(Object, Debug, Serialize, Deserialize)]
pub struct PromoCode {
    pub name: String,
    advantage: Advantage,
    restrictions: Vec<Restriction>,
}

#[derive(Object, Debug, Serialize, Deserialize)]
pub struct Advantage {
    percent: u32,
}

#[derive(Union, Debug, Serialize, Deserialize)]
enum Restriction {
    Date(DateRestriction),
    Age(AgeRestriction),
    Weather(WeatherRestriction),
    OR(Or),
    AND(And),
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct DateRestriction {
    date: Date,
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct Date {
    after: String,
    before: String,
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct AgeRestriction {
    age: Age,
}

#[derive(Union, Debug, Serialize, Deserialize)]
enum Age {
    Age(AgeRestrictionEqual),
    AgeRange(AgeRestrictionRange),
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct AgeRestrictionEqual {
    eq: u32,
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct AgeRestrictionRange {
    lt: u32,
    gt: u32,
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct WeatherRestriction {
    weather : WeatherTemp,
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct WeatherTemp {
    is: String,
    temp: Temperature,
}

/// In celsius
#[derive(Object, Debug, Serialize, Deserialize)]
struct Temperature {
    gt: f32, 
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct Or {
    or: Vec<Restriction>,
}

#[derive(Object, Debug, Serialize, Deserialize)]
struct And {
    and: Vec<Restriction>,
}

impl PromoCode {

    /// Add only coherent promocode in database
    pub fn push_if_coherent(self) -> bool {
        if !self.is_coherent() { return false }

        PromoCode::database_push(self);
        true
    }

    /// Get the avantage for the code name.
    pub fn get_advantage(promocode_name: &str) -> Advantage {
        if let Some(promocode) = PromoCode::database_get(promocode_name) {
            return promocode.advantage;
        }

        Advantage { percent: 0 } // in case not found one
    }

    /// Return true if promo code is valid.
    /// Return false if promo code is invalid and the reasons why in Vec<String>
    pub async fn is_promocode_valid_for_request(request: &PromoCodeRequest) -> (bool, Vec<String>) {
        match PromoCode::database_get(request.promocode_name.as_str()) {
            None => { (false, vec![MsgRestrictionError::MsgPromocodeNotExists.as_str().to_string()]) },
            Some(promocode) => {
                let mut reasons : Vec<String>= Vec::new();
                (promocode.is_valid(request, &mut reasons).await, reasons)
            }
        }
    }
}

fn is_valid_date(after: &str, before: &str) -> bool {
    let after_date;
    let before_date;
    
    if let Ok(d) = NaiveDate::parse_from_str(after, "%Y-%m-%d") {
        after_date = d;
    } else {
        return false;
    }
    
    if let Ok(d) = NaiveDate::parse_from_str(before, "%Y-%m-%d") {
        before_date = d;
    } else {
        return false;
    }
    
    if after_date > before_date { return false }
    
    let today = chrono::offset::Local::now().date_naive();
    if today < after_date || today > before_date { return false }
    
    true
}

/*
 * This Trait will be more evolutif with more restriction possible.
 */ 
pub trait Validator {
    /// Lets check if resctriction is well formed.
    /// Implement this trait with your own code or return true.
    fn is_coherent(&self) -> bool;

    /// Lets check if resctriction is valided.
    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool;
}

impl Validator for PromoCode {

    fn is_coherent(&self) -> bool {
        // Not accept two promo code with same name.
        if PromoCode::database_promocode_name_already_exists(self.name.as_str()) {
            return false;
        }
        if !self.advantage.is_coherent() {
                return false;
        }
        for r in self.restrictions.iter() {
            if !r.is_coherent() {
                return false;
            }
        }
        true
     }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        match PromoCode::database_get(request.promocode_name.as_str()) {
            None => { 
                reasons.push(MsgRestrictionError::MsgPromocodeNotExists.as_str().to_string());
                false
            },
            Some(_) => {
                let mut res = true;
                if !self.advantage.is_valid(request, reasons).await {
                    res = false;
                }
                for r in self.restrictions.iter() {
                    res &= r.is_valid(request, reasons).await; // We put all the reasons. 
                }
                res
            },
        }
    }
}

impl Validator for Restriction {
    fn is_coherent(&self) -> bool {
        match self {
            Restriction::Date(x)    => { x.is_coherent() },
            Restriction::Age(x)     => { x.is_coherent() },
            Restriction::Weather(x) => { x.is_coherent() },
            Restriction::OR(x)      => { x.is_coherent() },
            Restriction::AND(x)     => { x.is_coherent() },
        }
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        match self {
            Restriction::Date(x)    => { return Box::pin(x.is_valid(request, reasons)).await },
            Restriction::Age(x)     => { return Box::pin(x.is_valid(request, reasons)).await },
            Restriction::Weather(x) => { return Box::pin(x.is_valid(request, reasons)).await },
            Restriction::OR(x)      => { return Box::pin(x.is_valid(request, reasons)).await },
            Restriction::AND(x)     => { return Box::pin(x.is_valid(request, reasons)).await },
        }
    }
}

impl Validator for AgeRestriction {
    fn is_coherent(&self) -> bool {
        self.age.is_coherent()
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        self.age.is_valid(request, reasons).await
    }
}

impl Validator for AgeRestrictionEqual {
    fn is_coherent(&self) -> bool {
        self.eq > 0
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        if request.arguments.age == self.eq {
            return true;
        }
        reasons.push(MsgRestrictionError::MsgRestrictionAgeInvalid.as_str().to_string());
        false
    }

}

impl Validator for AgeRestrictionRange {
    fn is_coherent(&self) -> bool {
        self.gt < self.lt
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        if self.gt <= request.arguments.age && request.arguments.age <= self.lt {
            return true;
        }
        reasons.push(MsgRestrictionError::MsgRestrictionAgeRangeInvalid.as_str().to_string());
        false
    }
}

impl Validator for DateRestriction {
    fn is_coherent(&self) -> bool {
        self.date.is_coherent()
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        self.date.is_valid(request, reasons).await
    }

}

impl Validator for Date {
    fn is_coherent(&self) -> bool {
        let after_date;
        let before_date;

        if let Ok(d) = NaiveDate::parse_from_str(self.after.as_str(), "%Y-%m-%d") {
            after_date = d;
        } else {
            return false;
        }

        if let Ok(d) = NaiveDate::parse_from_str(self.before.as_str(), "%Y-%m-%d") {
            before_date = d;
        } else {
            return false;
        }
        after_date < before_date
    }

    async fn is_valid(&self, _request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        if is_valid_date(self.after.as_str(), self.before.as_str()) {
            return true;
        }
        reasons.push(MsgRestrictionError::MsgRestrictionDateRangeInvalid.as_str().to_string());
        false
    }

}

impl Validator for Advantage {
    fn is_coherent(&self) -> bool {
        self.percent > 0 && self.percent < 101
    }

    async fn is_valid(&self, _request: &PromoCodeRequest, _reasons: &mut Vec<String>) -> bool {
        true
    }
 
}

impl Validator for Age {
    fn is_coherent(&self) -> bool {
        match self {
             Age::Age(x) => { x.is_coherent() },
             Age::AgeRange(x) => { x.is_coherent() },
        }
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        match self {
             Age::Age(x) => { return x.is_valid(request, reasons).await },
             Age::AgeRange(x) => { return  x.is_valid(request, reasons).await },
        }
    }
}

impl Validator for WeatherRestriction {
    fn is_coherent(&self) -> bool {
        self.weather.is_coherent()
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        self.weather.is_valid(request, reasons).await
    }
}

impl Validator for WeatherTemp {
    fn is_coherent(&self) -> bool {
        extern_api::is_valid_condition_meteo(self.is.as_str())
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        // check town exists
        let (i, t) = extern_api::get_meteo(request.arguments.town.as_str()).await;

        if i.is_empty() || request.arguments.town.is_empty() { 
            reasons.push(MsgRestrictionError::MsgRestrictionTownInvalid.as_str().to_string());
            return false; 
        }

        if self.is == i &&  self.temp.gt < t {
            return true;
        }
        reasons.push(MsgRestrictionError::MsgRestrictionMeteoInvalid.as_str().to_string());
        false
    }
}

impl Validator for Or {
    fn is_coherent(&self) -> bool {
        let mut res = true;
        for r in self.or.iter() {
            res &= r.is_coherent();
        }
        res
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        let mut res = false;
        for r in self.or.iter() {
          if r.is_valid(request, reasons).await {
                res = true; // must continue to have all reasons
            }
        }
        res
    }
}

impl Validator for And {
    fn is_coherent(&self) -> bool {
        let mut res = true;
        for r in self.and.iter() {
            res &= r.is_coherent();
        }
        res
    }

    async fn is_valid(&self, request: &PromoCodeRequest, reasons: &mut Vec<String>) -> bool {
        let mut res = true;
        for r in self.and.iter() {
            res &= r.is_valid(request, reasons).await; // must continue to have all reasons
        }
        res
    }
}
