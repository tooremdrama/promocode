mod promocode;
mod promocode_api;
mod promocode_protocol;
mod database;
mod extern_api;

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{OpenApiService};
use promocode_api::AddPromoCodeAPI;
use promocode_api::IsValidPromoCodeAPI;
use promocode_api::IndexAPI;

#[tokio::main]
async fn main() {
    let endpoints: (IndexAPI, IsValidPromoCodeAPI, AddPromoCodeAPI) = (IndexAPI, IsValidPromoCodeAPI, AddPromoCodeAPI);
    let api_service = OpenApiService::new(endpoints, "Promocode", "1.0").server("http://localhost:8080");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs-api", ui); // route for API swagger

    let _ = Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await;
}

