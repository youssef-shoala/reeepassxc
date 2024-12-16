use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::fs::OpenOptions;

use crate::Vault;

// !!! Dependencies !!!
use serde::{Deserialize, Serialize};
use serde_json;



#[derive(Debug)]
pub struct OpenVault {
    vault: Vault,
    vault_contents: PathBuf,
}
impl OpenVault {
    pub fn new(vault: Vault) -> Self {
        // unzip vault file to sled db folder ./reepassdata/open-vault in file system
        let vault_path = vault.get_path();
        let file = std::fs::File::open(vault_path).unwrap();

        let mut archive = zip::ZipArchive::new(file).unwrap();
//        println!("{:?}", archive);

        let content_file_name = "./reeepassdata/open-vault/open-vault.kdbx";
        let mut content_file = archive.by_name_decrypt(content_file_name, b"password").unwrap();
        let mut content = String::new();
        content_file.read_to_string(&mut content).unwrap();
        println!("{}", content);
        let vault_contents = Path::new(content_file_name).to_path_buf();
        println!("Vault contents pathbuf: {:?}", vault_contents);
        //create parent folder
        let parent_folder = vault_contents.parent().unwrap();
        std::fs::create_dir_all(parent_folder).unwrap();
        let mut file = File::create(vault_contents.clone()).unwrap();
        file.write_all(content.as_bytes()).unwrap();

//        match archive.extract("./") {
//            Ok(_) => println!("Extracted"),
//            Err(e) => println!("Error: {:?}", e),
//        }



//        // get contents from open-vault.kdbx in open-vault folder
//        let vault_contents = File::open("./reeepassdata/open-vault/open-vault.kdbx").unwrap();
//        println!("{:?}", vault_contents);
//        let vault_contents_string = std::fs::read_to_string("./reeepassdata/open-vault/open-vault.kdbx").unwrap();
//        println!("{:?}", vault_contents_string);
       
        OpenVault {
            vault,
            vault_contents,
        }
    }
// >>del below
//    pub fn create_init_db() -> File {
//        // create vault in file system as .rdbx text file
//        let vault_content_path = Path::new("./reeepassdata/open-vault/open-vault.kdbx");
//        std::fs::create_dir_all("./reeepassdata/open-vault").unwrap();
//        let mut vault_contents_file = File::create(vault_content_path).unwrap();
//        let vault_contents = "Test Line 1\nTest Line 2\nTest Line 3\n";
//        vault_contents_file.write_all(vault_contents.as_bytes()).unwrap();
//        vault_contents_file
//    }
//    pub fn create_init_db() -> sled::Db {
//        // create vault in file system as sled db
//        let vault_content_path = Path::new("./reeepassdata/open-vault");
//        let vault_contents = sled::open(vault_content_path).unwrap();
//        vault_contents.insert(b"inner_config", "test".as_bytes()).unwrap();
//        println!("{:?}", vault_contents.get(b"inner_config").unwrap());
//        println!("{:?}", vault_contents);
//        vault_contents
//    }
    pub fn empty_db() -> Result<(), std::io::Error> {
        // create vault in file system as .rdbx text file
        let vault_content_path = Path::new("./reeepassdata/open-vault/open-vault.kdbx");
        std::fs::create_dir_all("./reeepassdata/open-vault").unwrap();
        let mut vault_contents_file = File::create(vault_content_path).unwrap();
        Ok(())
    }



    pub fn user_create_entry (
        &self,
        username: String,
        password: String,
        service_name: Option<String>,
        url: Option<String>, 
        tags: Option<Vec<String>>,
        notes: Option<String>, 
    ) {
        let entry = Entry::new(username, password, service_name, url, tags, notes);
        //serialize json
        let entry_json = serde_json::to_string(&entry).unwrap();
        //get open vault contents path
        let binding = self.get_vault_contents_path();
        let vault_contents_path = binding.as_path().to_str().unwrap();
        println!("{:?}", vault_contents_path);
        //write to file
        let mut vault_contents = OpenOptions::new().append(true).open(vault_contents_path).unwrap();
        writeln!(vault_contents, "{}", entry_json).unwrap();
        println!("successfully wrote to {:?}", vault_contents_path);
    }
    pub fn list_entries(&self) -> Vec<Entry> {
        //get open vault contents path
        let binding = self.get_vault_contents_path();
        let vault_contents_path = binding.as_path().to_str().unwrap();
        println!("{:?}", vault_contents_path);
        //read from file

//        let mut vault_contents = File::open(vault_contents_path).unwrap();
//        vault_contents.read_to_string(&mut contents).unwrap();

        let mut entries: Vec<Entry> = Vec::new();
        let mut contents = String::new();
        let vaults_contents_path_name = vault_contents_path.to_string();
        for line in std::fs::read_to_string(vaults_contents_path_name).unwrap().lines() {
            let entry: Entry = serde_json::from_str(&line).unwrap();
            entries.push(entry);
        }
        entries


        
//        for line in read_to_string(vault_contents_path.to_str()).unwrap().lines() {
//            result.push(line.to_string())
//        }

//        while Some(vault_contents.read_line(&mut contents).unwrap()) {
//            let contents = line.unwrap();
//
//            println!("{:?}", contents);
//            let entry: Entry = serde_json::from_str(&contents).unwrap();
//            entries.push(entry);
//        }
        //deserialize json
    }



    fn get_vault_contents_path(&self) -> PathBuf {
        self.vault_contents.clone()
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    username: String,
    password: String,
    service_name: Option<String>,
    url: Option<String>, 
    tags: Option<Vec<String>>,
    notes: Option<String>, 
}
impl Entry {
    fn new (
        username: String,
        password: String,
        service_name: Option<String>,
        url: Option<String>, 
        tags: Option<Vec<String>>,
        notes: Option<String>, 
    ) -> Self {
        Entry {
            username,
            password,
            service_name,
            url,
            tags,
            notes,
        }
    }
}




