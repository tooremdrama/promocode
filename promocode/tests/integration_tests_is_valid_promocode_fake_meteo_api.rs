mod commun;
use commun::*;


#[cfg(debug_assertions)]
#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    //////////////////////////////////////////////
    // Fake weather for this test only in dev mode//
    //////////////////////////////////////////////

    #[tokio::test]
    async fn test_is_valid_promocode_fake_weather_is_valid() {
        let name = "test_is_valid_promocode_fake_weather_is_valid";
        let json_promocode = get_content_json(&format!("{}.json", name));
        send_request("add-promocode", json_promocode, StatusCode::OK).await;

        send_request_expect("is-valid-promocode",
            get_is_valid_promocode_content(name, 25, "Lyon"),
            StatusCode::OK,
            &format!("{{\"advantage\":{{\"percent\":20}},\"promocode_name\":\"{}\",\"status\":\"accepted\"}}",
            name))
            .await;
    }

    #[tokio::test]
    async fn test_is_valid_promocode_fake_weather_is_invalid_temp() {
        let name = "test_is_valid_promocode_fake_weather_is_invalid_temp";
        let json_promocode = get_content_json(&format!("{}.json", name));
        send_request("add-promocode", json_promocode, StatusCode::OK).await;

        send_request_expect("is-valid-promocode",
            get_is_valid_promocode_content(name, 25, "Lyon"),
            StatusCode::BAD_REQUEST,
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_RESTRICTION_METEO_INVALID))
            .await;
    }

    #[tokio::test]
    async fn test_is_valid_promocode_fake_weather_is_invalid_is() {
        let name = "test_is_valid_promocode_fake_weather_is_invalid_is";
        let json_promocode = get_content_json(&format!("{}.json", name));
        send_request("add-promocode", json_promocode, StatusCode::OK).await;

        send_request_expect("is-valid-promocode",
            get_is_valid_promocode_content(name, 25, "Lyon"),
            StatusCode::BAD_REQUEST,
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_RESTRICTION_METEO_INVALID))
            .await;
    }

    #[tokio::test]
    async fn test_is_valid_promocode_fake_weather_is_invalid_all() {
        let name = "test_is_valid_promocode_fake_weather_is_invalid_all";
        let json_promocode = get_content_json(&format!("{}.json", name));
        send_request("add-promocode", json_promocode, StatusCode::OK).await;

        send_request_expect("is-valid-promocode",
            get_is_valid_promocode_content(name, 25, "Lyon"),
            StatusCode::BAD_REQUEST,
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_RESTRICTION_METEO_INVALID))
            .await;
    }
}
