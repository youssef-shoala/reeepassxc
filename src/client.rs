use std::path::{Path, PathBuf};
use std::fs;

use crate::OpenVault;
use crate::Vault;

// !!! Dependencies !!!
use toml;
use serde::{Deserialize, Serialize};
use rand::Rng;



// Config struct
/*
    - vaults_path: PathBuf

    - pub fn save_to_file(&self) -> Result<(), std::io::Error>
    - pub fn read_from_file() -> Config
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub vaults_path: PathBuf,
}
impl Config {
    fn new(vaults_path: PathBuf) -> Config {
        Config {
            vaults_path,
        }
    }
    fn file_exists() -> bool {
        let default_config_path = Path::new("./reeepassdata/config.toml");
        if default_config_path.exists() {
            true
        } else {
            false
        }
    }
    pub fn save_to_file(&self) -> Result<(), std::io::Error> {
        let config_path = Path::new("./reeepassdata/config.toml");
        let config_content = toml::to_string(&self).unwrap();
        let result: Result<(), std::io::Error>;
        if Config::file_exists() {
            result = fs::write(config_path, config_content)
        } else {
            //create folder
            match fs::create_dir_all("./reeepassdata/"){
                Ok(_) => {
                    ()
                },
                Err(e) => {
                    println!("Error creating folder: {:?}", e);
                },
            }
            result = fs::write(config_path, config_content)
        }
        result
    }
    pub fn read_from_file() -> Config {
        let config_path = Path::new("./reeepassdata/config.toml");
        let config: Config;
        if Config::file_exists() {
            let config_content = fs::read_to_string(config_path).expect("Unable to read file");
            config = toml::from_str(&config_content).unwrap();
        } else {
            let default_vaults_path = Path::new("./reeepassdata/vaults/");
            config = Config::new(default_vaults_path.to_path_buf());
            match config.save_to_file() {
                Ok(_) => {
                    ()
                },
                Err(e) => {
                    println!("Error saving config: {:?}", e);
                },
            }
        }
        config
    }
}








// Client
/*
    - config: Config
    - vaults: Vec<Vault>
!!! Important !!!
Unencrypted vault, with the following in memory protection: 
    - open_vault: Option<OpenVault>
            - 
!!! Important !!!
    - pub fn new(config: Config) -> Client
*/
#[derive(Debug)]
pub struct Client {
    config: Config,
    vaults: Vec<Vault>,
    // !!! Important !!!
    open_vault: Option<OpenVault>,
    // !!! Important !!!
}
impl Client {
    pub fn new(config: Config, vaults: Vec<Vault>) -> Client {
        Client {
            config,
            vaults: vaults,
            open_vault: None,
        }
    }
    pub fn get_vaults_path(&self) -> PathBuf {
        self.config.vaults_path.clone()
    }
    pub fn get_vaults(&self) -> Vec<Vault> {
        self.vaults.clone()
    }
    pub fn add_vault(&mut self, vault: Vault) {
        self.vaults.push(vault);
    }
    pub fn open_vault(&mut self, vault: Vault, password: &str) {
        let open_vault = OpenVault::new(vault, password);
        self.open_vault = Some(open_vault);
    }
    pub fn get_open_vault(&self) -> Option<OpenVault> {
        match &self.open_vault {
            Some(open_vault) => Some(open_vault.clone()),
            None => None,
        }
    }
    pub fn get_user_input() -> String {
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input.trim().to_lowercase()
    }
    pub fn get_secure_user_input() -> String {
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input).unwrap();
        user_input.trim().to_string()
    }
    pub fn generate_password(len: u64) -> String {
//        let password = rand::thread_rng()
//            .sample_iter(&Alphanumeric)
//            .take(len as usize); 
//        password
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789\
                                !@#$%^&*()_+-=[]{}|;:,.<>?";
        let mut rng = rand::thread_rng();

        // Generate a password by randomly selecting characters
        let password: String = (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        password
    }

}
