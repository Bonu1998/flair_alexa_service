use flair_alexa_sdk::{request::Request as AlexaRequest, response::Response as AlexaResponse};
use flair_general_utils::dao::MySqlDao;
use flair_types::skill::io::{BussinessInput, BussinessOutput};
use mvp::handler::bussiness_handler;
use rocket::{log::private::{error, info}, serde::json::Json};
use serde_json::{json, Value as JsonValue};

use crate::{
    guards::signature::Signature,
    handlers::{alexa_handler, wrappers::alexa_wrapper_v1_handler},
};

#[post(
    "/<request_type>/<wrapper>/<bussiness>/<skill_type>/<skill_name>",
    format = "json",
    data = "<body>"
)]
pub async fn skill_endpoint(
    request_type: String,
    wrapper: String,
    bussiness: String,
    skill_type: String,
    skill_name: String,
    body: Json<AlexaRequest>,
    signature: Signature,
) -> JsonValue {
    info!("\nskill_endpoint route");
    let mut _resp: JsonValue = json!({
        "success": false,
        "message": "Something went wrong",
    });
    match request_type.as_str() {
        "alexa" => {
            _resp = alexa_handler(
                wrapper,
                bussiness,
                skill_type,
                skill_name,
                body.into_inner(),
                signature,
            )
            .await;
        }
        _ => {
            error!("\nIncorrect Request type {}", request_type);
        }
    }
    _resp
}

#[post("/alexa_wrapper_v1", format = "json", data = "<body>")]
pub async fn alexa_wrapper_v1_endpoint(body: Json<JsonValue>) -> Json<AlexaResponse> {
    let _resp = alexa_wrapper_v1_handler(body.into_inner()).await;
    Json(_resp)
}

#[post("/mvp_v1", format = "json", data = "<body>")]
pub async fn bussiness(body: Json<BussinessInput>) -> Json<BussinessOutput> {
    match MySqlDao::init("mysql://root:RAJ.18@balaji@localhost/mvp".to_string()){
        Ok(dao)=>{
            Json(bussiness_handler(body.into_inner(), dao).await)
        }, 
        Err(e)=>{
            error!("\n{}", e);
            Json(BussinessOutput::new())
        }
    }
}
