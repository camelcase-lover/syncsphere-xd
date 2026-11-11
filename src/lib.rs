use std::collections::HashMap;
use serde_json::Value;
use std::fs;
use dryoc::secretbox::{SecretBox, Nonce, Key};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;
use std::io;

#[derive(Deserialize, Serialize, Default)]
pub struct OfflineStore{
    collections: HashMap<String, Vec<Value>>,
}

const STORE_FILE_PATH: &str = "";

impl OfflineStore{
    fn push(&mut self, collection: &str, data: Value) ->&mut Self{
        self.collections
            .entry(collection.to_string())
            .or_default()
            .push(data);
        self
    }

    fn append(&mut self, collection: &str, data: Vec<Value>) ->&mut Self{
        self.collections
            .entry(collection.to_string())
            .or_default()
            .extend(data);
        self
    }
    
    fn retrieve(&mut self, collection: &str) ->&Self{
        if let Some(records) = self.collections.get(collection){

        }
        self
    }

    fn retrieve_all(&mut self) -> &Self{
        self
    }

    fn delete(&mut self, collection: &str)
}

#[derive(Default)]
pub struct Path;

impl Path{
    fn secure_path(&self, app_name: &str) ->io::Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            if let Some(proj_dirs) = ProjectDirs::from("com", "SyncSphereXd", app_name) {
                let path = proj_dirs.data_dir().to_path_buf();
                fs::create_dir_all(&path)?;
                return Ok(path);
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(proj_dirs) = ProjectDirs::from("com", "SyncSphereXd", app_name) {
                let path = proj_dirs.data_dir().to_path_buf();
                fs::create_dir_all(&path)?;
                return Ok(path);
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(proj_dirs) = ProjectDirs::from("com", "SyncSphereXd", app_name){
                let path = proj_dirs.data_dir().to_path_buf();
                fs::create_dir_all(&path)?;
                return Ok(path);
            }
        }
        #[cfg(target_os = "ios")]
        {
            let mut path = PathBuf::from("/var/mobile/Containers/Data/Application");
            path.push("Library");
            path.push("Application Support");
            path.push(app_name);

            fs::create_dir_all(&path)?;
            Ok(path)
        }

        #[cfg(target_os = "android")]
        {
            let mut path = PathBuf::from("/storage/emulated/0");
            path.push("Documents");
            path.push(app_name);
            fs::create_dir_all(&path)?;
            Ok(path)
        }

        let mut fallback = PathBuf::from(".");
        fallback.push(app_name);
        fs::create_dir_all(&fallback)?;
        Ok(fallback)
    }
}
