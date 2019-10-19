use crate::error;
use base64;
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Deserialize, Clone, Serialize)]
pub struct EncodedMessage {
  data: String,
}

pub trait FromPubSubMessage
where
  Self: std::marker::Sized,
{
  fn from(message: EncodedMessage) -> Result<Self, error::Error>;
}

impl EncodedMessage {
  pub fn decode(&self) -> Result<Vec<u8>, base64::DecodeError> {
    base64::decode(&self.data)
  }

  pub fn new<T: serde::Serialize>(data: &T) -> Self {
    let json = serde_json::to_string(data).unwrap();
    let data = base64::encode(&json);
    EncodedMessage { data }
  }
}
