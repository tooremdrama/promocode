mod commun;
use commun::*;

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    // This const must come from specifications.
    // DO NOT USE promocode MSG* one.
    // If someone change the text in promocode crate, we can check that we take the one in
    // specifications here.
    // Expected response for request are in the following functions rather than in json file that we load.
    // This way, if a message change is done in specifications, we do not need to change all the json file.
    const MSG_PROMOCODE_NOT_EXISTS : &'static str = "This promo code doesn't exist.";
    const MSG_RESTRICTION_DATE_RANGE_INVALID : &str = "Invalid date restriction. Must be : after date <= today date <= before date.";
    const MSG_RESTRICTION_AGE_RANGE_INVALID : &str = "Invalid age restriction. Must be : lt <= age <= gt.";
    const MSG_RESTRICTION_AGE_INVALID : &str = "Invalid age restriction. Must be equal to eq in promocode.";
    const MSG_RESTRICTION_TOWN_INVALID: &str = "Invalid town.";

    #[tokio::test]
    async fn test_is_valid_promocode_no_promocode_related() {
        let name = "test_is_valid_promocode_no_promocode_related";
        send_request_expect(
            "is-valid-promocode",
            get_is_valid_promocode_content(name, 25, "Lyon"),
            StatusCode::BAD_REQUEST, 
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_PROMOCODE_NOT_EXISTS)
            ).await;
    }

    #[cfg(not(debug_assertions))]
    #[tokio::test]
    async fn test_is_valid_promocode_town_invalid() {
        if let Some((is, temp)) = check_current_meteo("Lyon").await {
            let name = "test_is_valid_promocode_town_invalid";
            add_promocode_setup(name, 18, is.as_str(), temp, StatusCode::OK).await ;

            send_request_expect(
                "is-valid-promocode",
                get_is_valid_promocode_content(name, 0, "t"),
                StatusCode::BAD_REQUEST,
                &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
                    name,
                    MSG_RESTRICTION_TOWN_INVALID))
                .await; 
        }
    }

    #[tokio::test]
    async fn test_is_valid_promocode_age_invalid() {
        let name = "test_is_valid_promocode_age_invalid";
        if let Some((_is, _temp)) = check_current_meteo("Lyon").await {
            let json_promocode = get_content_json(&format!("{}.json", name));
            send_request("add-promocode", json_promocode, StatusCode::OK).await;     

            send_request_expect("is-valid-promocode",
                get_is_valid_promocode_content(name, 3, "Lyon"),
                StatusCode::BAD_REQUEST,
                &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
                name,
                MSG_RESTRICTION_AGE_INVALID)
                ).await;
        }
    }

    #[tokio::test]
    async fn test_is_valid_promocode_valid_depth_complex_level_2() {
        let name = "test_is_valid_promocode_valid_depth_complex_level_2";
        let json_promocode = get_content_json(&format!("{}.json", name));
        send_request("add-promocode",
            json_promocode,
            StatusCode::OK)
            .await;

        send_request_expect("is-valid-promocode",
            get_is_valid_promocode_content(name, 4, "lyon"),
            StatusCode::BAD_REQUEST,
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_RESTRICTION_AGE_INVALID,
            MSG_RESTRICTION_METEO_INVALID,
            MSG_RESTRICTION_AGE_INVALID,
            MSG_RESTRICTION_AGE_INVALID,
            MSG_RESTRICTION_AGE_INVALID,
            MSG_RESTRICTION_AGE_INVALID,
            MSG_RESTRICTION_AGE_INVALID,
            MSG_RESTRICTION_AGE_INVALID,
            MSG_RESTRICTION_AGE_INVALID,
            MSG_RESTRICTION_AGE_INVALID))
            .await;
    }

    #[tokio::test]
    async fn test_is_valid_promocode_meteo_return_no_percentage() {
        let name = "test_is_valid_promocode_meteo_return_no_percentage";
        // a fake meteo to be sure this weather not works
        add_promocode_setup(name, 28, "Snow", 1000.0, StatusCode::OK).await ;

        send_request_expect(
            "is-valid-promocode",
            get_is_valid_promocode_content(name, 20, "lyon"),
            StatusCode::BAD_REQUEST,
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_RESTRICTION_METEO_INVALID))
            .await;
    }

    #[tokio::test]
    async fn test_is_valid_promocode_date_range() {
        let name = "test_is_valid_promocode_date_range";
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
    async fn test_is_valid_promocode_date_range_invalid() {
        let name = "test_is_valid_promocode_date_range_invalid";
        let json_promocode = get_content_json(&format!("{}.json", name));
        send_request("add-promocode", json_promocode, StatusCode::OK).await;

        send_request_expect("is-valid-promocode", 
            get_is_valid_promocode_content(name, 25, "Lyon"),
            StatusCode::BAD_REQUEST,
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_RESTRICTION_DATE_RANGE_INVALID))
            .await;
    }

    #[tokio::test]
    async fn test_is_valid_promocode_age_range_too_young_invalid() {
        let name = "test_is_valid_promocode_age_range_too_young_invalid";
        let json_promocode = get_content_json(&format!("{}.json", name));
        send_request("add-promocode", json_promocode, StatusCode::OK).await;

        send_request_expect("is-valid-promocode", 
            get_is_valid_promocode_content(name, 5, "Lyon"),
            StatusCode::BAD_REQUEST,
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_RESTRICTION_AGE_RANGE_INVALID))
            .await;
    }

    #[tokio::test]
    async fn test_is_valid_promocode_age_range_too_old_invalid() {
        let name = "test_is_valid_promocode_age_range_too_old_invalid";
        let json_promocode = get_content_json(&format!("{}.json", name));
        send_request("add-promocode", json_promocode, StatusCode::OK).await;

        send_request_expect("is-valid-promocode", 
            get_is_valid_promocode_content(name, 25, "Lyon"),
            StatusCode::BAD_REQUEST,
            &format!("{{\"promocode_name\":\"{}\",\"reasons\":[\"{}\"],\"status\":\"denied\"}}",
            name,
            MSG_RESTRICTION_AGE_RANGE_INVALID))
            .await;
    }
}
