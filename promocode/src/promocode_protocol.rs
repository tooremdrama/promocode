use poem_openapi::{Object, payload::Json};
use crate::promocode::PromoCode;
use crate::promocode::Advantage;

/// Status
const ACCEPTED : &str = "accepted";
const DENIED : &str = "denied";

#[derive(Object)]
pub struct PromoCodeRequest {
    pub promocode_name: String,    
    pub arguments: Arguments,
}

#[derive(Object)]
pub struct Arguments {
    pub age: u32,
    pub town: String,
}


#[derive(Object)]
pub struct PromoCodeValidResponse {
    promocode_name: String,    
    status: String,
    advantage: Advantage, 
}

#[derive(Object)]
pub struct PromoCodeInvalidResponse {
    promocode_name: String,    
    status: String,
    reasons: Vec<String>,
}

impl PromoCodeValidResponse<> {
    pub fn new(promocode_req: &Json<PromoCodeRequest>) -> Self {
        let promo_name = promocode_req.promocode_name.clone();
        PromoCodeValidResponse {
            promocode_name: promo_name.clone(),
            status: ACCEPTED.to_string(),
            advantage: PromoCode::get_advantage(&promo_name),
        }
    }
}

impl PromoCodeInvalidResponse {
    pub fn new(promocode_req: &Json<PromoCodeRequest>, reasons: Vec<String>) -> Self {
        PromoCodeInvalidResponse {
            promocode_name: promocode_req.promocode_name.clone(),
            status: DENIED.to_string(),
            reasons,
        }
    }
}
