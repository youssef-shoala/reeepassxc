use reeepassxc::{Config, Client, OpenVault, Vault};

// !!! Dependencies !!!
use clap::{Parser, Subcommand};
use walkdir::WalkDir;



// Cli parser
/*
    - Open
    - List
    - Search
    - Create
    - Convert
    - Delete
    - Move
    - Modify-config 
    - Unlock-and-copy-to-path
    - Copy-to-path
*/

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Open {
        vault_name: String,
        vault_group_name: Option<String>,
    }, 
    List {
        vault_group_name: Option<String>,
    },
    /* TODO 

    Search {
        search_terms: String,
        search_groups: Option<Vec<String>>,
        search_tags: Option<Vec<String>>,
    },

    */
    Create {
        vault_name: String,
        vault_group_name: Option<String>, 
    },
    /* TODO

    Convert {
        kbdx_file_path: String,
    },

    */
    Delete {
        vault_name: String,
        vault_group_name: Option<String>,
    },
    /* TODO 

    Move {
        vault_name: String,
        vault_group_new: String,
        vault_group_old: String,
    },

    */
    /* TODO

    Modify-config {
        config_key: String,
        config_value: String,
    }, 

    */
    /* TODO REQUIRED

    Unlock-and-copy-to-path {
        dst_path: Path,
        vault_name: String,
        vault_group_name: Option<String>,
    }, 
    Copy-to-path {
        dst_path: Path,
        vault_name: String,
        vault_group_name: Option<String>,
    },

    */
}





// Entry Point
/*
    - instantiate client from client config at ~/.reeepassdata/config.toml
    - create vaults folder if it doesn't exist
    - client has a vector of vaults in vaults folder
*/

fn main() {
    let config = Config::read_from_file();
    match std::fs::create_dir_all(config.vaults_path.clone()) {
        Ok(_) => {
            ()
        },
        Err(e) => {
            println!("Error creating vaults folder: {:?}", e);
        },
    }
    // create vault struct instances from vaults path
    let mut gen_vaults: Vec<Vault> = Vec::new();
    for entry in WalkDir::new(config.vaults_path.clone()) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let vault_path = entry.path().to_path_buf();
                    let vault_name = entry.file_name().to_str().unwrap().to_string();
                    //for vault group check if parent folder is root, if not set group to parent folder path
                    let group_path = entry.path().parent().unwrap().to_path_buf();
                    let mut mut_vault_group_path = None;
                    let mut mut_vault_group_name = None;
                    if group_path != config.vaults_path {
                        let group_name = group_path.file_name().unwrap().to_str().unwrap().to_string();
                        mut_vault_group_path = Some(group_path);
                        mut_vault_group_name = Some(group_name);
                    } 
                    let vault_group_path = mut_vault_group_path;
                    let vault_group_name = mut_vault_group_name;
                    let vault = Vault::new(vault_path, vault_name, vault_group_path, vault_group_name);
                    gen_vaults.push(vault);
                }
            },
            Err(e) => {
                println!("Error reading vault path: {:?}", e);
            },
        }
    }
    let vaults: Vec<Vault> = gen_vaults;
    // init client
    let mut client = Client::new(config, vaults);
    // cli parser
    let args = Cli::parse();
    match args.cmd {



        Commands::Open {vault_name, vault_group_name} => {
            println!("Opening vault: {:?}", vault_name);
            let group_name = vault_group_name.clone();
            let vault_group_path = match group_name {
                Some(group_name) => Some(client.get_vaults_path().join(format!("{}/", group_name))),
                None => None,
            };
            let group_path = vault_group_path.clone();
            let vault_path = match group_path {
                Some(group_path) => group_path.join(format!("{}.kbdx", vault_name)),
                None => client.get_vaults_path().join(format!("{}.kbdx", vault_name)),
            };
            let target_vault = Vault::new(vault_path, vault_name, vault_group_path, vault_group_name);

            let mut cli_mode = false;
            for vault in client.get_vaults() {
                if target_vault.get_path() == vault.get_path() {
                    println!("Enter password to open vault: ");
                    let password: &str = &Client::get_user_input();
                    client.open_vault(target_vault.clone(), password);
                    cli_mode = true;
                }
            }

            // enter cli mode
            while cli_mode {
                println!("Enter command: ");
                let user_input = Client::get_user_input();
                match user_input.as_str() {
                    "delete" => {
                        println!("Deleting entry");
                        println!("Enter username: ");
                        let entry_input = Client::get_user_input();
                        let username = entry_input.clone();
                        client.get_open_vault().unwrap().delete_entry(username);
                    },
                    "list" => {
                        println!("Listing entries");
                        let entries = client.get_open_vault().unwrap().list_entries();
                        for entry in entries {
                            println!("{:?}", entry);
                        }
                    },
                    "add" => {
                        println!("Adding entry");
                        println!("Enter username: ");
                        let mut entry_input = Client::get_user_input();
                        let username = entry_input.clone();
                        println!("Enter password (blank for client gen password): ");
                        entry_input = Client::get_user_input();
                        let password = match entry_input.as_str() {
                            "" => {
                                // generate password
                                println!("Generating password");
                                let generated_password_len = 20;
                                let password = Client::generate_password(generated_password_len);
                                println!("Generated password: {:?}", password);
                                password
                            },
                            _ => {
                                let password = entry_input.clone();
                                password
                            },
                        };
                        println!("Enter service name: ");
                        entry_input = Client::get_user_input();
                        let service_name = entry_input.clone();
                        println!("Enter url: ");
                        entry_input = Client::get_user_input();
                        let url = entry_input.clone();
                        println!("Enter tags (space separated): ");
                        entry_input = Client::get_user_input();
                        let tags = entry_input.clone();
                        println!("Enter notes: ");
                        entry_input = Client::get_user_input();
                        let notes = entry_input.clone();
                        client.get_open_vault().unwrap().user_create_entry(
                            username, 
                            password, 
                            Some(service_name), 
                            Some(url), 
                            Some(tags.split_whitespace().map(|s| s.to_string()).collect()), 
                            Some(notes)
                            );
                    },
                    "hi" => {
                        println!("Hello");
                    },
                    "exit" => {
                        cli_mode = false;
                        // save vault, compress and encrypt to file
                        let openvault = client.get_open_vault().unwrap();
                        let vault = openvault.get_vault();
                        let password_hash = openvault.get_password_hash();
                        OpenVault::encrypt_and_delete_db(vault, password_hash).unwrap();
                    },
                    _ => {
                        println!("Invalid command");
                    },
                }
            }
        },



        Commands::List {vault_group_name} => {
            println!("Listing vaults");
            let group_name = vault_group_name.clone();
            let vault_group_path = match group_name {
                Some(group_name) => Some(client.get_vaults_path().join(format!("{}/", group_name))),
                None => None,
            };

            for vault in client.get_vaults() {
                match vault_group_path {
                    Some(ref vault_group_path) => {
                        if vault.get_group().unwrap().get_path() == *vault_group_path {
                            println!("{:?}", vault);
                        }
                    },
                    None => {
                        println!("{:?}", vault);
                    },
                }
            }
        },



        //creates empty file with vault name
        Commands::Create {vault_name, vault_group_name} => {
            println!("Creating vault: {:?}", vault_name);

            let group_name = vault_group_name.clone();
            let vault_group_path = match group_name {
                Some(group_name) => Some(client.get_vaults_path().join(format!("{}/", group_name))),
                None => None,
            };
            let group_path = vault_group_path.clone();
            let vault_path = match group_path {
                Some(group_path) => group_path.join(format!("{}.kbdx", vault_name)),
                None => client.get_vaults_path().join(format!("{}.kbdx", vault_name)),
            };
            let vault = Vault::new(vault_path, vault_name, vault_group_path, vault_group_name);

            println!("Enter master password: ");
            let password: &str = &Client::get_user_input();
            match vault.create(password) {
                Ok(_) => {
                    println!("Vault created");
                    client.add_vault(vault);
                },
                Err(e) => {
                    println!("Error creating vault: {:?}", e);
                },
            }
            println!("{:?}", client);
        },



        Commands::Delete {vault_name, vault_group_name} => {
            println!("Deleting vault: {:?}", vault_name);
            for vault in client.get_vaults() {
                if vault.get_name() == format!("{}.kbdx", vault_name) {
                    match vault_group_name {
                        Some(ref vault_group_name) => {
                            if vault.get_group().unwrap().get_name() == *vault_group_name {
                                match vault.delete() {
                                    Ok(_) => {
                                        println!("Vault deleted");
                                    },
                                    Err(e) => {
                                        println!("Error deleting vault: {:?}", e);
                                    },
                                }
                            }
                        },
                        None => {
                            if vault.get_group().is_none() {
                                match vault.delete() {
                                    Ok(_) => {
                                        println!("Vault deleted");
                                    },
                                    Err(e) => {
                                        println!("Error deleting vault: {:?}", e);
                                    },
                                }
                            }
                        },
                    }
                }
            }
        },




    }
}
