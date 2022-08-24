use flair_alexa_sdk::{request::Request as AlexaRequest, response::Response as AlexaResponse};
use flair_alexa_wrapper::wrapper::alexa_wrapper;
use rocket::log::private::{info, debug};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use crate::guards::signature::Signature;

pub async fn alexa_wrapper_v1_handler(
    body: JsonValue
) -> AlexaResponse {
    info!("\nalexa_wrapper_v1_handler invoked");
    debug!("\nbody: {:?}", body);
    let mut skill_info:HashMap<String, String> = HashMap::new();
    let mut request_body:AlexaRequest = AlexaRequest::new();
    let mut bussiness_path:String = "".to_string();
    let mut signature:Signature = Signature::new("".to_string(), "".to_string());

    match serde_json::from_value::<HashMap<String, String>>(body["skill_info"].clone()){
        Ok(_skill_info) =>{skill_info = _skill_info},
        Err(e)=>{
            error!("\n{}", e);
        }
    }
    match serde_json::from_value::<AlexaRequest>(body["request_body"].clone()){
        Ok(_request_body) =>{request_body = _request_body},
        Err(e)=>{
            error!("\n{}", e);
        }
    }
    match serde_json::from_value::<Signature>(body["signature"].clone()){
        Ok(_signature) =>{signature = _signature},
        Err(e)=>{
            error!("\n{}", e);
        }
    }
    match serde_json::from_value::<String>(body["bussiness_path"].clone()){
        Ok(_bussiness_path) =>{bussiness_path = _bussiness_path},
        Err(e)=>{
            error!("\n{}", e);
        }
    }
    let _resp = alexa_wrapper(skill_info, request_body, signature.signature, signature.signature_cert_chain_url, bussiness_path).await;
    _resp
}
