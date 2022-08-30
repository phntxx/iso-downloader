use std::env;
use std::str;
use std::fs::{read_to_string, File};
use std::io::Write;
use curl::easy::Easy;

pub fn download_file(url: &str, filename: &str) {
  let mut curl = Easy::new();
  let directory = env::current_dir().unwrap();

  curl.url(url).unwrap();
  curl.follow_location(true).unwrap();
  curl.progress(true).unwrap();

  let mut file = File::create(directory.join(filename)).unwrap();

  curl.write_function(move |data| {
    file.write_all(data).unwrap();
    Ok(data.len())
  }).unwrap();

  curl.perform().unwrap();
}

pub fn delete_file(filename: &str) {
  todo!();
}

pub fn download_str(url: &str) -> String {
  /*
   * As of now, this function is needlessly complex.
   * What it does is it writes the response data into a temporary file,
   * then parses that temporary file as a string, which it then returns.
   * I have no idea how closures work, so this'll have to do.
   */
  let mut curl = Easy::new();

  let file_path = env::temp_dir().join("downloader-temp");

  curl.url(url).unwrap();
  curl.follow_location(true).unwrap();

  let mut file = File::create(file_path.clone()).unwrap();

  curl.write_function(move |data| {
    file.write_all(data).unwrap();
    Ok(data.len())
  }).unwrap();

  curl.perform().unwrap();

  let result = read_to_string(file_path.clone())
    .expect("Should have been able to read the file");

  return result;
}
