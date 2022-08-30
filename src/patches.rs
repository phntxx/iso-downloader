
use crate::validator;
use crate::downloader;
use crate::structure::config::Image;
use crate::structure::sourceforge::SourceForge;

pub fn sourceforge(image: Image) {

  let raw_json = downloader::download_str(&image.url);
  let data: SourceForge = serde_json::from_str(&raw_json).unwrap();

  // downloader::download_file(&data.release.url, &image.name);

  validator::validate_md5(&image.name, &data.release.md5sum);
}