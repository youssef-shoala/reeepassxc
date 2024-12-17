use std::path::PathBuf;
use std::path::Path;
use std::io::Read;
use std::io::Write;

use crate::OpenVault;

// !!! Dependencies !!!
use serde::{Deserialize, Serialize};
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
    pub fn create(&self, password: &str) -> Result<(), std::io::Error> {
        // check if vault name exists
        if self.path.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Vault already exists"));
        }

        // create init db
        OpenVault::empty_db().unwrap();
//        let open_vault: Option<OpenVault> = Some(OpenVault::new(self.clone(), password));
        let vault = self.clone();
        let password_hash = password.to_string();
        // compress it,  encrypt it,  write it to vault file
        OpenVault::encrypt_and_delete_db(vault, password_hash).unwrap();
        Ok(())
    }
    pub fn delete(&self) -> Result<(), std::io::Error> {
        let path_to_delete = self.path.clone();
        std::fs::remove_file(path_to_delete)
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn get_group(&self) -> Option<VaultGroup> {
        let vault_group = match &self.group {
            None => None,
            Some(group) => Some(group.clone()),
        };
        vault_group
    }
}

#[derive(Debug, Clone)]
pub struct VaultGroup {
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
    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

