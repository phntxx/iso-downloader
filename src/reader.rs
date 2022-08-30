use serde_yaml;
use std::fs;

use crate::structure::Config;

pub fn read(config: &str) -> Config {

  let file_contents = fs::read_to_string(config)
    .expect("Something went wrong reading the configuration file.");

  let data = serde_yaml::from_str::<Config>(&file_contents);

  assert!(data.is_ok());

  return data.unwrap();
}