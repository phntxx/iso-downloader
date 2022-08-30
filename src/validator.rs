use std::env;
use md5;
use sha256;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use crate::downloader::download_str;

fn get_file_path(filename: &str) -> PathBuf {
  let file_path = env::current_dir().unwrap();
  return file_path.join(filename);
}

pub fn validate_asc(filename: &str, asc: &str, asc_base: &str) -> bool {
  todo!();
}

pub fn validate_signature(filename: &str, signature: &str) -> bool {
  todo!();
}

pub fn validate_sha256(filename: &str, sha: &str) -> bool {

  println!("Validating SHA256 of {}...", filename);

  let downloaded_sha = download_str(sha);
  let file_path = get_file_path(filename);
  let file_sha = sha256::digest_file(file_path).unwrap();

  return downloaded_sha.contains(&file_sha);
}

pub fn validate_md5(filename: &str, md5: &str) -> bool {

  println!("Validating MD5 of {}...", filename);

  let file_path = get_file_path(filename);
  let mut file = File::open(&file_path).unwrap();

  let mut file_contents = Vec::<u8>::new();
  file.read_to_end(&mut file_contents).unwrap();

  let file_md5 = md5::compute(file_contents);
  let file_md5_string = format!("{:x}", file_md5);

  return file_md5_string == md5;
}
