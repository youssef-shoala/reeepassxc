use std::path::Path;
use std::fs::File;

use crate::Vault;

// !!! Dependencies !!!
use serde::{Deserialize, Serialize};
use sled;



#[derive(Debug)]
pub struct OpenVault {
    vault: Vault,

    vault_contents: sled::Db,
//    inner_config: InnerConfig,
//    entries: Vec<Entry>,
}
impl OpenVault {
    pub fn new(vault: Vault) -> Self {
        // unzip vault file to sled db folder ./reepassdata/open-vault in file system
        // contents to vault_contents

        let vault_path = vault.get_path();
        let file = std::fs::File::open(vault_path).unwrap();

        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);
//            .aes_encryption(Some("password"), AesMode::Aes256)
//            .unix_permissions(0o755);
        let mut archive = zip::ZipArchive::new(file).unwrap();

        match archive.extract("./reeepassdata/open-vault") {
            Ok(_) => println!("Extracted"),
            Err(e) => println!("Error: {:?}", e),
        }

//        for i in 0..archive.len() {
//            let mut file = archive.by_index(i).unwrap();
//            let outpath = match file.enclosed_name() {
//                Some(path) => path.to_owned(),
//                None => continue,
//            };
//            if file.is_dir() {
//                println!("File {} extracted to \"{}\"", i, outpath.display());
//                std::fs::create_dir_all(outpath).unwrap();
//            } else {
//                println!(
//                    "File {} extracted to \"{}\" ({} bytes)",
//                    i,
//                    outpath.display(),
//                    file.size()
//                );
//                if let Some(p) = outpath.parent() {
//                    if !p.exists() {
//                        std::fs::create_dir_all(p).unwrap();
//                    }
//                }
//                let mut outfile = File::create(&outpath).unwrap();
////                println!("file contents: {:?}", file);
//                std::io::copy(&mut file, &mut outfile).unwrap();
//            }
//        }

        let vault_contents = sled::open("./reeepassdata/open-vault").unwrap();
        println!("{:?}", vault_contents.get(b"inner_config").unwrap());
       
        OpenVault {
            vault,
            vault_contents,
        }
    }
    pub fn create_init_db() -> sled::Db {
        // create vault in file system as sled db
        let inner_config = InnerConfig::new();
        let vault_content_path = Path::new("./reeepassdata/open-vault");
        let vault_contents = sled::open(vault_content_path).unwrap();
        vault_contents.insert(b"inner_config", "test".as_bytes()).unwrap();
        println!("{:?}", vault_contents.get(b"inner_config").unwrap());
        println!("{:?}", vault_contents);
        vault_contents
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct InnerConfig {
    protected_field_cipher_id: String,
    protected_field_cipher_key: String,

    totp_cipher_id: Option<String>,
    totp_cipher_key: Option<String>,
}
impl InnerConfig {
    fn new() -> InnerConfig {
        InnerConfig {
            protected_field_cipher_id: "".to_string(),
            protected_field_cipher_key: "".to_string(),
            totp_cipher_id: None,
            totp_cipher_key: None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    id: u64,
    tags: Vec<String>,
    service_name: String,
    username: String,
    password: String,
    url: String, 
    notes: String, 
}




