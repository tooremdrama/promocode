use poem_openapi::{OpenApi, payload::{Json, PlainText}, ApiResponse};

use crate::promocode::PromoCode;
use crate::promocode_protocol::PromoCodeRequest;
use crate::promocode_protocol::PromoCodeValidResponse;
use crate::promocode_protocol::PromoCodeInvalidResponse;

pub struct IsValidPromoCodeAPI;
pub struct AddPromoCodeAPI;
pub struct IndexAPI;

#[OpenApi]
impl IndexAPI {
    /// IndexAPI to indicate possible route.
    #[oai(path = "/", method = "get")]
    async fn welcome(&self) -> PlainText<&'static str> {
        PlainText("Welcome\nUse http://localhost:8080/docs-api to see the available features.")
    }
}

#[OpenApi]
impl IsValidPromoCodeAPI {
    /// IsValidPromoCodeAPI lets to check validity of promocode.
    #[oai(path = "/is-valid-promocode", method = "post")]
    async fn is_valid_promocode(&self, promocode_req:Json<PromoCodeRequest>) -> PromoCodeIsValidResponse {
        let (valided, reasons) = PromoCode::is_promocode_valid_for_request(&promocode_req).await;
        match valided {
            true  => PromoCodeIsValidResponse::Ok(Json(PromoCodeValidResponse::new(&promocode_req))),
            false => PromoCodeIsValidResponse::Invalid(Json(PromoCodeInvalidResponse::new(&promocode_req, reasons))),
        }
    }
}

#[OpenApi]
impl AddPromoCodeAPI {
    /// AddPromoCodeAPI lets to add a promocode.
    #[oai(path = "/add-promocode", method = "post")]
    async fn add_promocode(&self, promocode:Json<PromoCode>) -> PromoCodeAddedResponse {
        let Json(promo) = promocode;
        if promo.push_if_coherent() {
            return PromoCodeAddedResponse::Ok; 
        }
        PromoCodeAddedResponse::Invalid
    }
}

/// API response for add-promocode route
#[derive(ApiResponse)]
enum PromoCodeAddedResponse {
    #[oai(status = 200)]
    Ok,
    #[oai(status = 400)]
    Invalid,
}

/// API response for is-valid-promocode route
#[derive(ApiResponse)]
enum PromoCodeIsValidResponse {
    #[oai(status = 200)]
    Ok(Json<PromoCodeValidResponse>),
    #[oai(status = 400)]
    Invalid(Json<PromoCodeInvalidResponse>),
}
