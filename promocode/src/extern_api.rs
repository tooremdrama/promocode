#[cfg(not(debug_assertions))]
const OPEN_WEATHER_MAP_KEY : &str = "PUT_YOUR_KEY_HERE";

const METEOS : [&str; 16]= ["Thunderstorm", "Drizzle", "Rain", "Snow", "Mist", "Smoke", "Haze", "Dust","Fog", "Sand", "Dust", "Ash", "Squall", "Tornado", "Clear", "Clouds"];

#[cfg(debug_assertions)]
pub async fn get_meteo(_town: &str) -> (String, f32) {
    ("Rain".to_string(), 28.)
}

/// Use <https://openweathermap.org/current> API to get meteo and temperature
#[cfg(not(debug_assertions))]
pub async fn get_meteo(town: &str) -> (String, f32) {
    let res = reqwest::get(format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", town, OPEN_WEATHER_MAP_KEY)).await;

    let default_res = (String::from(""), 0.0);  // Default value if problem with extern API
    if let Ok(r) = res {
        let body = r.text();

        if let Ok(json_str) = body.await {
            let is = get_pattern_data_in_meteo(&json_str, "main\":\"", false);
            if "" == is {
                return default_res;
            }
            if let Ok(temp) = get_pattern_data_in_meteo(&json_str, "temp\":", true).parse() {
                println!("GET METEO JSON {:?}", json_str);
                return (is, temp)
            }
        }
    }
    default_res
}

/// Return true if specified meteo is correct.
/// TODO BONUS get directly from API
pub fn is_valid_condition_meteo (meteo: &str) -> bool {
    for m in METEOS.iter() {
        if *m == meteo {
            return true;
        }
    }
    false
}

/// Search a pattern attribute in json_str and extract value.
#[cfg(not(debug_assertions))]
fn get_pattern_data_in_meteo(json_str: &str, pattern: &str, is_number_after: bool) -> String {
    let mut offset = 1;
    if is_number_after {
        offset = 0;
    }
    let index: Option<usize> = json_str.find(pattern);
    if let Some(index_beg) = index {
        let index_is_end = json_str[index_beg..].find(",");
        if let Some(index_end) = index_is_end {
            return json_str[(index_beg + pattern.len()) ..(index_beg + index_end-offset)].to_string().clone();
        }
    }
    return "".to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    /// This test lets to know current meteo in Lyon used at the present time.
    async fn test_api_meteo_lyon() {
        let (is, temp) = get_meteo("lyon").await;
        eprintln!("METEO : get_meteo(lyon) -> ({}, {})", is, temp);
    }

    #[tokio::test]
    /// This test lets to know current meteo in Paris used at the present time.
    async fn test_api_meteo_paris() {
        let (is, temp) = get_meteo("paris").await;
        eprintln!("METEO : get_meteo(paris) -> ({}, {})", is, temp);
    }

    #[cfg(not(debug_assertions))]
    #[tokio::test]
    /// This test lets to know current meteo used in a fictive town at the present time.
    async fn test_api_meteo_town_does_not_exist() {
        let (is, temp) = get_meteo("sdhqdhqdmoihfoidsmhfq").await;
        eprintln!("METEO : get_meteo(sdhqdhqdmoihfoidsmhfq) -> ({}, {})", is, temp);
        assert!((is, temp) == (String::from(""), 0.0));
    }

    #[test]
    /// Test invalid meteo
    fn test_api_is_valid_condition_meteo_invalid() {
        assert!(is_valid_condition_meteo("dfhdshfo") == false);
    }

    #[test]
    /// Test valid meteo
    fn test_api_is_valid_condition_meteo() {
        assert!(is_valid_condition_meteo("Snow") == true);
    }
}
