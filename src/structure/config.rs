use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
  pub name: String,
  pub url: String,
  pub sha256: Option<String>,
  pub asc: Option<String>,
  pub asc_base: Option<String>,
  pub signature: Option<String>,
  pub patches: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
  pub images: Vec<Image>
}