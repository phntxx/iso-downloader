use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Release {
  bytes: i64,
  date: String,
  date_modified: String,
  file_type: String,
  filename: String,
  pub md5sum: String,
  mime_type: String,
  release_notes_url: Option<String>,
  sf_download_label: Option<String>,
  sf_platform: Vec<String>,
  sf_platform_default: Vec<String>,
  sf_release_id: Option<String>,
  sf_release_notes_file: Option<String>,
  sf_type: Option<String>,
  pub url: String,
  vscan: String,
  vscan_when: String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlatformReleases {
  mac: Release,
  windows: Release,
  linux: Release
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SourceForge {
  pub release: Release,
  platform_releases: PlatformReleases
}