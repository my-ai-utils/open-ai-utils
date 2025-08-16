use std::time::Duration;

use flurl::{FlUrl, body::FlUrlBody};
use rust_extensions::{base64::IntoBase64, date_time::DateTimeAsMicroseconds};

use crate::{OpenAiRequestBodyBuilder, OtherRequestData, my_auto_gen::*};

pub async fn execute_fl_url_request(
    settings: &HttpRequestSettingsModel,
    rb: &OpenAiRequestBodyBuilder,
    other_request_data: &OtherRequestData,
) -> Result<(OpenAiRespModel, String), String> {
    let mut fl_url = FlUrl::new(settings.url.as_str()).set_timeout(Duration::from_secs(60));

    if let Some(api_key) = settings.api_key.as_ref() {
        fl_url = fl_url.with_header("Authorization", format!("Bearer {}", api_key));
    };

    if settings.do_not_reuse_connection.unwrap_or(false) {
        fl_url = fl_url.do_not_reuse_connection();
    }

    let model = rb.get_model(other_request_data).await;

    rb.write_tech_log(TechRequestLogItem::new_data_as_str(
        DateTimeAsMicroseconds::now(),
        TechLogItemType::Request,
        serde_json::to_string(&model).unwrap(),
    ))
    .await;

    let response = fl_url
        .post(FlUrlBody::as_json(&model))
        .await
        .map_err(|itm| itm.to_string())?;

    let status_code = response.get_status_code();

    if status_code != 200 {
        let body = response.receive_body().await.unwrap();
        println!("OpenAI status code: {}", status_code);
        println!("{:?}", std::str::from_utf8(body.as_slice()));
        return Err(format!("Status code: {}", status_code));
    }

    let body = response
        .receive_body()
        .await
        .map_err(|itm| itm.to_string())?;

    let model: Result<OpenAiRespModel, _> = serde_json::from_slice(body.as_slice());

    match model {
        Ok(model) => {
            let body = match std::str::from_utf8(body.as_slice()) {
                Ok(body_as_str) => body_as_str.to_string(),
                Err(_) => body.as_slice().into_base64(),
            };

            return Ok((model, body));
        }
        Err(err) => {
            println!("Can not deserialize JsonModel. Err: `{}`", err);
            panic!("Can not deserialize JsonModel. Err: `{}`", err);
        }
    }
}
