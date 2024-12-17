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
use zip::AesMode;



#[derive(Clone, Debug)]
pub struct OpenVault {
    vault: Vault,
    vault_contents: PathBuf,
    password_hash: String,
}
impl OpenVault {
    pub fn new(vault: Vault, password: &str) -> Self {
        //save password hash
        let password_hash = password.to_string();
        // unzip vault file to sled db folder ./reepassdata/open-vault in file system
        let vault_path = vault.get_path();
        let file = std::fs::File::open(vault_path).unwrap();
        let mut archive = zip::ZipArchive::new(file).unwrap();
        let content_file_name = "./reeepassdata/open-vault/open-vault.kdbx";
        let mut content_file = archive.by_name_decrypt(content_file_name, password_hash.as_bytes()).unwrap();
        let mut content = String::new();
        content_file.read_to_string(&mut content).unwrap();
        let vault_contents = Path::new(content_file_name).to_path_buf();
        //create parent folder
        let parent_folder = vault_contents.parent().unwrap();
        std::fs::create_dir_all(parent_folder).unwrap();
        let mut file = File::create(vault_contents.clone()).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        OpenVault {
            vault,
            vault_contents,
            password_hash,
        }
    }
    pub fn empty_db() -> Result<(), std::io::Error> {
        // create vault in file system as .rdbx text file
        let vault_content_path = Path::new("./reeepassdata/open-vault/open-vault.kdbx");
        std::fs::create_dir_all("./reeepassdata/open-vault").unwrap();
        let _ = File::create(vault_content_path).unwrap();
        Ok(())
    }
    pub fn encrypt_and_delete_db(vault: Vault, password_hash: String) -> Result<(), std::io::Error> {
        // compress it,  encrypt it,  write it to vault file
        let compression = zip::CompressionMethod::Deflated;
        let binding = vault.get_path();
        let dst_path = Path::new(&binding);
        let _ = match vault.get_group() {
            Some(_) => {
                let dst_parent = dst_path.parent().unwrap();
                std::fs::create_dir_all(dst_parent).unwrap();
            },
            None => {
                ()
            },
        };
        let mut zip = zip::ZipWriter::new(std::fs::File::create(dst_path).unwrap());
        zip.set_flush_on_finish_file(true);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(compression)
            .with_aes_encryption(AesMode::Aes256, password_hash.as_str())
            .unix_permissions(0o755);
        let path = Path::new("./reeepassdata/open-vault/open-vault.kdbx");
        let path_str = path.to_str().unwrap();
        zip.start_file(path_str, options).unwrap();
        let mut f = std::fs::File::open(path).unwrap();
        let file_size = std::fs::metadata(path).unwrap().len();
        let mut buffer = vec![0u8; file_size as usize];
        f.read_exact(&mut buffer).unwrap();
        zip.write_all(&buffer).unwrap();
        buffer.clear();
        std::fs::remove_dir_all("./reeepassdata/open-vault").unwrap();
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
        //write to file
        let mut vault_contents = OpenOptions::new().append(true).open(vault_contents_path).unwrap();
        writeln!(vault_contents, "{}", entry_json).unwrap();
        println!("successfully wrote to {:?}", vault_contents_path);
    }
    pub fn list_entries(&self) -> Vec<Entry> {
        //get open vault contents path
        let binding = self.get_vault_contents_path();
        let vault_contents_path = binding.as_path().to_str().unwrap();
        //read from file
        let mut entries: Vec<Entry> = Vec::new();
        let vaults_contents_path_name = vault_contents_path.to_string();
        for line in std::fs::read_to_string(vaults_contents_path_name).unwrap().lines() {
            let entry: Entry = serde_json::from_str(&line).unwrap();
            entries.push(entry);
        }
        entries
    }
    pub fn delete_entry(&self, username: String) {
        //get open vault contents path
        let binding = self.get_vault_contents_path();
        let vault_contents_path = binding.as_path().to_str().unwrap();
        //read from file
        let mut entries: Vec<Entry> = Vec::new();
        let vaults_contents_path_name = vault_contents_path.to_string();
        for line in std::fs::read_to_string(vaults_contents_path_name).unwrap().lines() {
            let entry: Entry = serde_json::from_str(&line).unwrap();
            if entry.get_username() != username {
                entries.push(entry);
            }
        }
        //write to file
        //clear file
        // Open the file with write and truncate options
        let _ = OpenOptions::new()
            .write(true)
            .truncate(true) // Truncate the file to zero length
            .open(vault_contents_path);


        let mut vault_contents = OpenOptions::new().append(true).open(vault_contents_path).unwrap();
        for entry in entries {
            let entry_json = serde_json::to_string(&entry).unwrap();
            writeln!(vault_contents, "{}", entry_json).unwrap();
        }
        println!("successfully deleted {} from {:?}", username, vault_contents_path);
    }



    fn get_vault_contents_path(&self) -> PathBuf {
        self.vault_contents.clone()
    }
    pub fn get_vault(&self) -> Vault {
        self.vault.clone()
    }
    pub fn get_password_hash(&self) -> String {
        self.password_hash.clone()
    }
}


// Entry
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
    pub fn get_username(&self) -> String {
        self.username.clone()
    }
}




