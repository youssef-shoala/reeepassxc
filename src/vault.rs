use std::path::PathBuf;

// !!! Dependencies !!!
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct OpenVault {
    vault: Vault,

    inner_config: InnerConfig,
    entries: Vec<String>,
}
impl OpenVault {
    fn new(vault: Vault) -> Self {
        let inner_config = InnerConfig {
            protected_field_cipher_id: "".to_string(),
            protected_field_cipher_key: "".to_string(),
            totp_cipher_id: None,
            totp_cipher_key: None,
        };
        let entries = Vec::new();
        OpenVault {
            vault,
            inner_config,
            entries,
        }
    }
}
#[derive(Debug)]
struct InnerConfig {
    protected_field_cipher_id: String,
    protected_field_cipher_key: String,

    totp_cipher_id: Option<String>,
    totp_cipher_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Vault {
    path: PathBuf, 
    name: String,
    group: Option<VaultGroup>,
    //tags: Option<Vec<VaultTag>>,
    outer_config: OuterConfig,
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

        let outer_config = OuterConfig::new();

        Vault {
            path,
            name,
            group,
            outer_config,
        }
    }
    // create vault in file system from vault struct instance
    pub fn create(&self) -> Result<(), std::io::Error> {
        // check if vault name exists
        if self.path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Vault already exists"));
        }
        let create_file_result = match &self.group {
            None => {
                // create vault in file system
                match std::fs::File::create(&self.path) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            },
            Some(group) => {
                // create group directory
                match std::fs::create_dir_all(&group.path) {
                    Ok(_) => {
                        // create vault in group directory
                        match std::fs::File::create(&self.path) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e),
                        }
                    },
                    Err(e) => Err(e),
                }
            },
        };
        // write outer config to vault file
        let outer_config_toml = toml::to_string(&self.outer_config).unwrap();
        let write_outer_config_result = match std::fs::write(&self.path, outer_config_toml) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        };

        create_file_result?;
        write_outer_config_result?;
        Ok(())
    }
    pub fn open(&self) -> OpenVault {
        OpenVault::new(self.clone()) 
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
struct OuterConfig {
    compression: String, 
    cipher_id: String,
    encryption_iv: String,
    kdf_params: KDFParameters,
}
impl OuterConfig {
    fn new() -> Self {
        let compression = "none".to_string();
        let cipher_id = "AES256".to_string();
        let encryption_iv = "".to_string();
        let kdf_params = KDFParameters {
            kdf: "argon2".to_string(),
            rounds: 10,
            memory: 1024,
            parallelism: 1,
            salt: "".to_string(),
            seed: "".to_string(),
        };
        OuterConfig {
            compression,
            cipher_id,
            encryption_iv,
            kdf_params,
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

