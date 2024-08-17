use crate::http_request::get_request;
use crate::http_response::map_response_image_response;
use ic_cdk::api::management_canister::http_request::{
    http_request as http_request_outcall, HttpHeader, HttpResponse as HttpResponseCdk,
    TransformArgs as TransformArgsCdk,
};
use ic_cdk::print;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DogApiResponse {
    message: String,
    status: String,
}

pub async fn query(key: &str, prompt: &str) -> Result<String, String> {
    let request = get_request(key, prompt)?;

    print(format!("🔫 ---------> Starting the request. {}", prompt));

    match http_request_outcall(request, 25_000_000_000).await {
        Ok((response,)) => {
            print("✅ ---------> Request processed.".to_string());

            map_response_image_response(response)
        },
        Err((r, m)) => {
            let message =
                format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}");

            print(format!("‼️ ---------> {}.", message));

            Err(message)
        }
    }
}

// Strips all data that is not needed from the original response.
pub fn transform_response(raw: TransformArgsCdk) -> HttpResponseCdk {
    let headers = vec![
        HttpHeader {
            name: "Content-Security-Policy".to_string(),
            value: "default-src 'self'".to_string(),
        },
        HttpHeader {
            name: "Referrer-Policy".to_string(),
            value: "strict-origin".to_string(),
        },
        HttpHeader {
            name: "Permissions-Policy".to_string(),
            value: "geolocation=(self)".to_string(),
        },
        HttpHeader {
            name: "Strict-Transport-Security".to_string(),
            value: "max-age=63072000".to_string(),
        },
        HttpHeader {
            name: "X-Frame-Options".to_string(),
            value: "DENY".to_string(),
        },
        HttpHeader {
            name: "X-Content-Type-Options".to_string(),
            value: "nosniff".to_string(),
        },
    ];

    let mut res = HttpResponseCdk {
        status: raw.response.status.clone(),
        body: raw.response.body.clone(),
        headers,
    };

    if i32::try_from(res.status.clone().0).unwrap() == 200 {
        res.body = raw.response.body;
    } else {
        ic_cdk::api::print(format!("Received an error from proxy: err = {:?}", raw));
    }

    res
}
