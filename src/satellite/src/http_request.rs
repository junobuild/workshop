use ic_cdk::api::management_canister::http_request::{
    CanisterHttpRequestArgument, HttpHeader, HttpMethod, TransformContext as TransformContextCdk,
};
use serde_json::{json, to_string, Value};

pub fn get_request(key: &str, prompt: &str) -> Result<CanisterHttpRequestArgument, String> {
    let body: Value = gpt_body_image_generation(prompt);

    let body_json = to_string(&body).map_err(|e| e.to_string())?;

    let request_headers = vec![
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "application/json".to_string(),
        },
        HttpHeader {
            name: "idempotency-key".to_string(),
            value: key.to_owned(),
        },
    ];

    let request = CanisterHttpRequestArgument {
        url: "https://us-central1-openai-proxy-ipv6.cloudfunctions.net/openai/images/generations"
            .to_string(),
        method: HttpMethod::POST,
        body: Some(body_json.into_bytes()),
        max_response_bytes: None,
        transform: Some(TransformContextCdk::from_name(
            "transform".to_string(),
            serde_json::to_vec(&Vec::<u8>::new()).unwrap(),
        )),
        headers: request_headers,
    };

    Ok(request)
}

fn gpt_body_image_generation(prompt: &str) -> Value {
    json!({
      "model": "dall-e-3",
      "prompt": prompt.to_owned(),
      "n": 1,
      "size": "1024x1024"
    })
}
