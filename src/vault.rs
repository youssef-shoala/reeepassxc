use std::path::PathBuf;

//move to new file: openvault.rs
#[derive(Debug)]
pub struct OpenVault {
    vault: Vault,
    entries: Vec<String>,
}

#[derive(Debug)]
pub struct Vault {
    path: PathBuf, 
    name: String,
    group: Option<VaultGroup>,
    //tags: Option<Vec<VaultTag>>,
//    outer_config: OuterConfig,
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

//        let outer_config = OuterConfig::new(&path);

        Vault {
            path,
            name,
            group,
//            outer_config,
        }
    }
}
#[derive(Debug)]
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
#[derive(Debug)]
struct OuterConfig {
    compression: String, 
    cipher_id: String,
    encryption_iv: String,
    kdf_params: KDFParameters,
}
#[derive(Debug)]
struct KDFParameters {
    kdf: String,
    rounds: u64,
    memory: u64,
    parallelism: u64,
    salt: String,
    seed: String,
}

