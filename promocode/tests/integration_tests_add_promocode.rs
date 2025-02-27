mod commun;
use commun::*;

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[tokio::test]
    async fn test_add_promocode_with_empty_restriction() {
        let json_request = get_content_json("test_add_promocode_with_empty_restriction.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }
    
    #[tokio::test]
    async fn test_add_promocode_with_date_range_valid() {
        let json_request = get_content_json("test_add_promocode_with_date_range_valid.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_date_range_invalid() {
        let json_request = get_content_json("test_add_promocode_with_date_range_invalid.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_date_range_invalid_format() {
        let json_request = get_content_json("test_add_promocode_with_date_range_invalid_format.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_age_eq() {
        let json_request = get_content_json("test_add_promocode_with_age_eq.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_age_eq_invalid() {
        let json_request = get_content_json("test_add_promocode_with_age_eq_invalid.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_percent_signed_invalid() {
        let json_request = get_content_json("test_add_promocode_with_percent_signed_invalid.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_percent_bigger_100_invalid() {
        let json_request = get_content_json("test_add_promocode_with_percent_bigger_100_invalid.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_percent_0_invalid() {
        let json_request = get_content_json("test_add_promocode_with_percent_0_invalid.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_weather() {
        let json_request = get_content_json("test_add_promocode_with_weather.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_weather_town_invalid() {
        let json_request = get_content_json("test_add_promocode_with_weather_town_invalid.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_zero_restriction_in_or() {
        let json_request = get_content_json("test_add_promocode_with_zero_restriction_in_or.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_one_restriction_in_or() {
        let json_request = get_content_json("test_add_promocode_with_one_restriction_in_or.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_two_restriction_in_or() {
        let json_request = get_content_json("test_add_promocode_with_two_restriction_in_or.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_three_restriction_in_or() {
        let json_request = get_content_json("test_add_promocode_with_three_restriction_in_or.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_three_restriction_in_or_invalid() {
        let json_request = get_content_json("test_add_promocode_with_three_restriction_in_or_invalid.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_zero_restriction_in_and() {
        let json_request = get_content_json("test_add_promocode_with_zero_restriction_in_and.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_three_restriction_in_and() {
        let json_request = get_content_json("test_add_promocode_with_three_restriction_in_and.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_with_three_restriction_in_and_invalid() {
        let json_request = get_content_json("test_add_promocode_with_three_restriction_in_and_invalid.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }

    #[tokio::test]
    async fn test_add_promocode_complex() {
        let json_request = get_content_json("test_add_promocode_complex.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_depth_complex() {
        let json_request = get_content_json("test_add_promocode_depth_complex.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_depth_complex_level_2() {
        let json_request = get_content_json("test_add_promocode_depth_complex_level_2.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_example_in_test() {
        let json_request = get_content_json("test_add_promocode_example_in_test.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;
    }

    #[tokio::test]
    async fn test_add_promocode_add_twice_with_same_name() {
        let json_request = get_content_json("test_add_promocode_add_twice_with_same_name.json");
        send_request("add-promocode", json_request, StatusCode::OK).await;

        let json_request = get_content_json("test_add_promocode_add_twice_with_same_name.json");
        send_request("add-promocode", json_request, StatusCode::BAD_REQUEST).await;
    }
}

