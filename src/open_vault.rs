use std::path::Path;
use crate::Vault;

// !!! Dependencies !!!
use sled;
use serde::{Deserialize, Serialize};



#[derive(Debug)]
pub struct OpenVault {
    vault: Vault,

    vault_contents: sled::Db,
//    inner_config: InnerConfig,
//    entries: Vec<Entry>,
}
impl OpenVault {
    pub fn new(vault: Vault) -> Self {
        //#todo read from file
        let vault_contents = OpenVault::create_init_db();
       
        OpenVault {
            vault,
            vault_contents,
//            inner_config,
//            entries,
        }
    }
    pub fn create_init_db() -> sled::Db {
        // create vault in file system as sled db
        let inner_config = InnerConfig::new();
        let vault_content_path = Path::new("./reeepassdata/open-vault");
        let vault_contents = sled::open(vault_content_path).unwrap();
        vault_contents.insert(b"inner_config", toml::to_string(&inner_config).unwrap().as_bytes()).unwrap();
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




