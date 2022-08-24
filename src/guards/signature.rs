use rocket::{request::{FromRequest, Outcome},Request, log::private::error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature{
    pub signature:String,
    pub signature_cert_chain_url: String
}

impl Signature {
    pub fn new(signature:String, signature_cert_chain_url:String)-> Signature{
        Signature{signature, signature_cert_chain_url}
    }
}

#[derive(Debug)]
pub enum AlexaSignatureError {
    Missing,
    // InValid
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Signature {
    type Error = AlexaSignatureError;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let mut signature: String = String::from("DEFAULT");
        let mut signature_cert_chain_url: String = String::from("DEFAULT");
        match request.headers().get_one("Signature") {
            Some(v) => {
                signature = String::from(v);
            }
            None => {error!("\nSignature not found")}
        }
        match request.headers().get_one("SignatureCertChainUrl") {
            Some(v) => {
                signature_cert_chain_url = String::from(v);
            }
            None => {error!("\nSignatureCertChainUrl not found")}
        }
        Outcome::Success(Signature::new(signature, signature_cert_chain_url))
    }
}