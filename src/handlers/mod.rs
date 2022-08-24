pub mod wrappers;

use flair_alexa_sdk::{request::Request as AlexaRequest, response::Response as AlexaResponse};
use flair_alexa_wrapper::models::ServiceMap;
use flair_general_utils::file_fetch::{get_data, post_data};
use rocket::log::private::{error, info, debug};
use serde_json::{json, Value as JsonValue};
use std::{collections::HashMap, env};

use crate::guards::signature::Signature;

pub async fn alexa_handler(
    wrapper: String,
    bussiness: String,
    skill_type: String,
    skill_name: String,
    body: AlexaRequest,
    signature: Signature,
) -> JsonValue {
    info!("\nalexa_handler invoked");
    let mut _resp = AlexaResponse::default_session_close();
    match env::var("FILES_BASE_PATH") {
        Ok(base_path) => {
            let path = format!("{}/general/service_map.json", base_path);
            debug!("\nconfig file path: {}", path);
            let mut wrapper_path: String = String::from("");
            let mut bussiness_path: String = String::from("");
            let mut skill_info: HashMap<String, String> = HashMap::new();
            match get_data::<ServiceMap>(path).await {
                Ok(service_map) => {
                    debug!("{:?}", service_map);
                    for i in service_map.alexa_wrappers {
                        if i.name == wrapper && i.is_active == true {
                            wrapper_path = i.url.clone();
                        }
                    }
                    for i in service_map.bussiness {
                        if i.name == bussiness && i.is_active == true {
                            bussiness_path = i.url.clone();
                            skill_info.insert("content_token".to_string(), i.content_token.clone());
                            skill_info.insert("skill_type".to_string(), skill_type.clone());
                            skill_info.insert("skill_name".to_string(), skill_name.clone());
                        }
                    }
                    if wrapper_path.len() > 0 && bussiness_path.len() > 0 {
                        let _req_body = json!({
                          "skill_info": skill_info,
                          "request_body": body,
                          "signature":signature,
                          "bussiness_path":bussiness_path
                        });
                        match post_data::<AlexaResponse>(wrapper_path, _req_body, HashMap::new())
                            .await
                        {
                            Ok(r) => {
                                _resp = r;
                            }
                            Err(e) => {
                                error!("\nwrapper: {}\n", e);
                            }
                        }
                    } else {
                        error!(
                            "\nSomething wrong with path: {} {}\n",
                            wrapper_path, bussiness_path
                        );
                    }
                }
                Err(e) => {
                    error!("\nError fetching service map file: {}\n", e);
                }
            }
        }
        Err(err) => {
            error!("\nError while fetching config file: {}\n", err);
        }
    }
    debug!("\noutput: {:?}", _resp);
    match serde_json::to_value(_resp) {
        Ok(resp) => resp,
        Err(e) => {
            error!("\nerror converting alexa response to json: {}", e);
            json!({
              "response": {
                "shouldEndSession": false
              },
              "version": "1.0"
            })
        }
    }
}
