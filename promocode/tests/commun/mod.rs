use reqwest::{StatusCode};
use promocode::extern_api::get_meteo;
use std::{fs, path::PathBuf};

const JSON_DIR: &str = "tests/promocode-json/";
pub const MSG_RESTRICTION_METEO_INVALID: &str = "Invalid meteo restriction.";

pub async fn send_request(api : &str, json: String, status_expected: StatusCode){
    let client = reqwest::Client::new();
    let res = client.post(format!("http://localhost:8080/{}", api))
            .header("Content-Type", "application/json; charset=utf-8")
            .body(json) // exact body to send
            .send().await;

    eprintln!("response = {:?}", res);
    if let Ok(body) = res {
        let status = body.status().clone();
        if let Ok(text) = body.text().await {
           eprintln!("response body = {:?}", text);
           assert!(status_expected == status);
        }
    }
    else {
        assert!(false);
    }
}

pub async fn send_request_expect(api : &str, json: String, status_expected: StatusCode, expect: &str){
    eprintln!("### send_request_expect: \nAPI: {}\njson: {}", api, json);
    let client = reqwest::Client::new();
    let res = client.post(format!("http://localhost:8080/{}", api))
            .header("Content-Type", "application/json; charset=utf-8")
            .body(json) // exact body to send
            .send().await;

    eprintln!("response: {:?}", res);
    if let Ok(body) = res {
        let status = body.status().clone();
        if let Ok(text) = body.text().await {
            eprintln!("EXPECT:#{}#\nANSWER:#{}#", expect, text);
            assert!(text == expect);
        }
        assert!(status_expected == status);
    }
    else {
        assert!(false);
    }
}

pub async fn add_promocode_setup(
    promocode_name: &str, 
    percentage: i32,
    is: &str,
    temp: f32,
    status_expected: StatusCode) {
    let request = format!( 
"{{
  \"name\": \"{}\",
  \"advantage\": {{ \"percent\": {} }},
  \"restrictions\": [ {{ \"weather\": {{ \"is\": \"{}\", \"temp\": {{ \"gt\": {} }} }} }}]
                }}"
                , promocode_name, percentage, is, temp);

     send_request("add-promocode", request, status_expected).await;
}

pub fn get_is_valid_promocode_content(promocode_name: &str, age: i32, town: &str) -> String {
    format!("
{{
  \"promocode_name\": \"{}\",
  \"arguments\": {{
    \"age\": {},
    \"town\": \"{}\"
  }}
}}",
    promocode_name,
    age,
    town)
}


/// Test with meteo can be not done if weather API is not available.
pub async fn check_current_meteo(town: &str) -> Option<(String, f32)> {
        let (is, temp) = get_meteo(town).await;
        eprintln!("### Current meteo in {} is : {} {}", town, is.clone(), temp);
        if "" == is {
            assert!(true); // Can't test at the moment
            eprintln!("### API Weather problem, can test at the moment.");
            return None;
        }
        Some((is, temp))
}

pub fn get_content_json(file_test: &str) -> String {
    let file_path = PathBuf::from(JSON_DIR).join(file_test);
    match fs::read_to_string(file_path) {
        Ok(s) => { return s },
        _     => { eprintln!("No File {} found to test", file_test);
                   assert!(false);
                   return String::from("")
                 },
    }
}

