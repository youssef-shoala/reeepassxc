use std::path::PathBuf;
use std::path::Path;
use std::io::Read;
use std::io::Write;

use crate::OpenVault;

// !!! Dependencies !!!
use serde::{Deserialize, Serialize};
use zip::write::SimpleFileOptions;
use zip::CompressionMethod;
use zip::AesMode;
use walkdir::WalkDir;




#[derive(Debug, Clone)]
pub struct Vault {
    path: PathBuf, 
    name: String,
    group: Option<VaultGroup>,
}
impl Vault {
    pub fn new(
        path: PathBuf, 
        name: String, 
        group_path: Option<PathBuf>, 
        group_name: Option<String>, 
    ) -> Self {        
        let group: Option<VaultGroup> = match group_path {
            None => None,
            Some(group_path) => {
                match group_name {
                    None => None,
                    Some(group_name) => Some(VaultGroup::new(group_path, group_name)),
                }
            },
        };
        Vault {
            path, 
            name, 
            group,
        }
    }



    // create vault in file system from vault struct instance
    pub fn create(&self) -> Result<(), std::io::Error> {
        // check if vault name exists
        if self.path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Vault already exists"));
        }

        // create init db
        let open_vault_init_db: sled::Db = OpenVault::create_init_db();
        // compress it
        // encrypt it  
        // write it to file
        /*        
        let compression = CompressionMethod::Deflated;
        let cipher_id = AesMode::Aes256;
        let encryption_iv = "".to_string();
        let kdf_params = KDFParameters {
            kdf: "argon2".to_string(),
            rounds: 10,
            memory: 1024,
            parallelism: 1,
            salt: "".to_string(),
            seed: "".to_string(),
        }; 
        */
        let compression = CompressionMethod::Deflated;
//        let cipher_id = AesMode::Aes256;

//        let dst_str = format!("./reeepassdata/{}_vault.zip", self.name);
        let dst_path = Path::new(self.path.as_os_str());
        let mut zip = zip::ZipWriter::new(std::fs::File::create(dst_path).unwrap());
        let options = SimpleFileOptions::default()
            .compression_method(compression);
        for entry in WalkDir::new("./reeepassdata/open-vault") {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let path_str = path.to_str().unwrap();
                println!("Adding file {:?} as {:?}...", path, path_str);
                zip.start_file(path_str, options).unwrap();
                let mut f = std::fs::File::open(path).unwrap();
                let mut buffer = Vec::new();
                f.read_to_end(&mut buffer).unwrap();
                zip.write_all(&buffer).unwrap();
                buffer.clear();
            } else if path.is_dir() {
                let path_str = path.to_str().unwrap();
                println!("Adding directory {:?} as {:?}...", path, path_str);
                zip.add_directory(path_str, options).unwrap();
            }
        }
        // destroy open vault
        std::fs::remove_dir_all("./reeepassdata/open-vault").unwrap();
        Ok(())
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
struct VaultGroup {
    path: PathBuf,
    name: String,
}
impl VaultGroup {
    fn new(path: PathBuf, name: String) -> Self {
        VaultGroup {
            path, 
            name, 
        }
    }
}



#[derive(Serialize, Deserialize, Debug, Clone)]
struct KDFParameters {
    kdf: String,
    rounds: u64,
    memory: u64,
    parallelism: u64,
    salt: String,
    seed: String,
}

