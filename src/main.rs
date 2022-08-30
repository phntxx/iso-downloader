mod reader;
mod structure;
mod patches;
mod downloader;
mod validator;

use crate::structure::Config;

fn main() {
    let config: Config = reader::read("./data/config.yml");
    

    for image in config.images {

        if !image.patches.is_none() {

            // let the patch handler ride this one out
            match image.patches.as_ref().unwrap().as_str() {
                "sourceforge" => patches::sourceforge(image),
                &_ => todo!(),
            };
            
        } else {
            
            // downloader::download_file(&image.url, &image.name);

            if !image.sha256.is_none() {
                validator::validate_sha256(&image.name, &image.sha256.unwrap());
            }
        }
    }

}
