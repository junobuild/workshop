use ic_cdk::api::management_canister::http_request::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
struct DallE3Data {
    revised_prompt: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct DallE3Response {
    created: u64,
    data: Vec<DallE3Data>,
}

pub fn map_response_image_response(response: HttpResponse) -> Result<String, String> {
    let str_body =
        String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.");

    let dalle_response: DallE3Response = from_str(&str_body).map_err(|e| e.to_string())?;

    let data = dalle_response
        .data
        .first()
        .ok_or_else(|| "No data available in the DallE3 response.".to_string())?;

    Ok(data.url.clone())
}
