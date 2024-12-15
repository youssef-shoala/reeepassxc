use std::path::{Path, PathBuf};
use std::fs;

use super::vault::{OpenVault, Vault};

// !!! Dependencies !!!
use toml;
use serde::{Deserialize, Serialize};



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
//            println!("Saving file override the curr config file.");
            result = fs::write(config_path, config_content)
        } else {
//            println!("Saving file but the config file does not exist.");
            //create folder
            match fs::create_dir_all("./reeepassdata/"){
                Ok(_) => {
//                    println!("Created folder");
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
//            println!("Reading file and the config file exists.");
            let config_content = fs::read_to_string(config_path).expect("Unable to read file");
            config = toml::from_str(&config_content).unwrap();
        } else {
//            println!("Reading file but the config file does not exist. Creating default config and saving to config file.");
            let default_vaults_path = Path::new("./reeepassdata/vaults/");
            config = Config::new(default_vaults_path.to_path_buf());
            match config.save_to_file() {
                Ok(_) => {
//                    println!("Config saved to default file");
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
    pub fn add_vault(&mut self, vault: Vault) {
        self.vaults.push(vault);
    }
}
