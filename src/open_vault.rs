use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::Read;

use crate::Vault;

// !!! Dependencies !!!
use serde::{Deserialize, Serialize};
use sled;



#[derive(Debug)]
pub struct OpenVault {
    vault: Vault,
    vault_contents: File,
}
impl OpenVault {
    pub fn new(vault: Vault) -> Self {
        // unzip vault file to sled db folder ./reepassdata/open-vault in file system
        let vault_path = vault.get_path();
        let file = std::fs::File::open(vault_path).unwrap();

//        let options = zip::write::SimpleFileOptions::default()
//            .compression_method(zip::CompressionMethod::Deflated);
//            .aes_encryption(Some("password"), AesMode::Aes256)
//            .unix_permissions(0o755);
        let mut archive = zip::ZipArchive::new(file).unwrap();

        match archive.extract("./") {
            Ok(_) => println!("Extracted"),
            Err(e) => println!("Error: {:?}", e),
        }

        // get contents from open-vault.kdbx in open-vault folder
        let vault_contents = File::open("./reeepassdata/open-vault/open-vault.kdbx").unwrap();
        println!("{:?}", vault_contents);
        let vault_contents_string = std::fs::read_to_string("./reeepassdata/open-vault/open-vault.kdbx").unwrap();
        println!("{:?}", vault_contents_string);

//        let content = walkdir::WalkDir::new("./reeepassdata/open-vault")
//            .into_iter()
//            .filter_map(|e| e.ok())
//            .filter(|e| e.file_type().is_file())
////            .map(|e| e.path().to_path_buf())
//            .map(|e| std::fs::read_to_string(e.path()).unwrap())
//            .collect::<Vec<String>>();
//        println!("{:?}", content);

       
        OpenVault {
            vault,
            vault_contents,
        }
    }
    pub fn create_init_db() -> File {
        // create vault in file system as .rdbx text file
        let vault_content_path = Path::new("./reeepassdata/open-vault/open-vault.kdbx");
        std::fs::create_dir_all("./reeepassdata/open-vault").unwrap();
        let mut vault_contents_file = File::create(vault_content_path).unwrap();
        let vault_contents = "Test Line 1\nTest Line 2\nTest Line 3\n";
        vault_contents_file.write_all(vault_contents.as_bytes()).unwrap();
        vault_contents_file
    }
//    pub fn create_init_db() -> sled::Db {
//        // create vault in file system as sled db
//        let vault_content_path = Path::new("./reeepassdata/open-vault");
//        let vault_contents = sled::open(vault_content_path).unwrap();
//        vault_contents.insert(b"inner_config", "test".as_bytes()).unwrap();
//        println!("{:?}", vault_contents.get(b"inner_config").unwrap());
//        println!("{:?}", vault_contents);
//        vault_contents
//    }
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




