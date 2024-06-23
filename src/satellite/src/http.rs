use ic_cdk::api::management_canister::http_request::{
    http_request as http_request_outcall, CanisterHttpRequestArgument, HttpMethod,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DogApiResponse {
    message: String,
    status: String,
}

pub async fn query() -> Result<String, String> {
    let url = "https://dog.ceo/api/breeds/image/random".to_string();

    let request_headers = vec![];

    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    match http_request_outcall(request, 2_000_000_000).await {
        Ok((response,)) => {
            let str_body = String::from_utf8(response.body)
                .expect("Transformed response is not UTF-8 encoded.");

            let dog_response: DogApiResponse =
                serde_json::from_str(&str_body).map_err(|e| e.to_string())?;

            Ok(dog_response.message)
        }
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            Err(message)
        }
    }
}
