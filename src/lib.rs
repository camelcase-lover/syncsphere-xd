use std::collections::HashMap;
use serde_json::Value;
use std::fs;
use dryoc::secretbox::{SecretBox, Nonce, Key};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;
use std::io;
use dryoc::dryocsecretbox::{DryocSecretBox, Key, Nonce};
use dryoc::types::*;
#[derive(Deserialize, Serialize, Default)]
pub struct OfflineStore{
    collections: HashMap<String, Vec<Value>>,
    #[serde(skip)]
    file_path: PathBuf,
}

const NONCE_SIZE: usize = 24;

impl OfflineStore{

    pub fn open(path: impl Into<PathBuf>) -> io::Result<Self>{
      	let file_path = path.into();
	if file_path.exists(){
		let data = fs::read(&file_path)?;
		if data.len() < NONCE_SIZE + 16 {
		     return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid or currupted storage file"));
		}
		let nonce = Nonce::try_from(&data[..NONCE_SIZE])
			.map_err(|_| io::Error::new(
			io::ErrorKind::InvalidData, "invalid encryption nonce"
		))?;

		let encrypted = DryocSecretBox::from_bytes(&data[NONCE_SIZE..])
		.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid encrypted payload"))?;

		let decrypted = encrypted
			.decrypt_to_vec(&nonce, key)
			.map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Decryption failed: incorrect key or modified file"))?;

		let mut store: Self = serde_json::from_slice(&decrypted)
		.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
		store.file_path = file_path;
		Ok(Store)
	} else {
		Ok(Self{
			collections: HashMap::new(),
			file_path,
		})
	}
    }

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
    
    fn retrieve(&mut self, collection: &str) ->Option<&Value>{
        self.collections
	    .get(collection)
	    .map(|records| records.as_slice())
    }

    fn retrieve_all(&mut self) -> &HashMap<String, Vec<Value>>{
        &self.collections
    }
    fn pop(&mut self, collection: &str) -> Option<Value>{
          self.collections
              .get_mut(collection)?
              .pop()
    }
    fn delete(&mut self, collection: &str) -> &mut Self{
	self.collections.remove(collection);
	self	
    }
    fn delete_record(&mut self, collection: &str, id: &str) -> &mut Self {
	if let Some(records) = self.collections.get_mut(collection) {
	     records.retain(|record| {
	        record.get("id")
		       .and_then(Value::as_str)
		       != Some(id)     
	 });
      }
     fn update(&mut self, collection: &str, id: &str, data: Value) -> &mut Self {
	if let Some(records) = self.collections.get_mut(collection) {
           if let Some(record) = records.iter_mut().find(|record| {
	      	record.get("id")
		.and_then(Value::as_str) == Some(id)
	 }) {
             *record = data;
            }
         } 
         self
      }
    }

    fn count(&self, collection: &str) -> usize {
        self.collections
	    .get(collection)
	    .map_or(0, |records| records.len())	
    }
    
    fn save(&self) -> io::Result<()> {
       if let Some(parent) = self.file_path.parent() {
	    if !parent.as_os_str().is_empty() {
	     fs::create_dir_all(parent)?;
          }
	}
        let serialized = serde_json::to_vec(&self.collections)
	.map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
	
	let nonce = Nonce::generate();
	let encrypted = DryocSecretBox::encrypt_to_vecbox(&serialized, &nonce, key)
	.map_err(|err| io::Error::other(err.to_string()))?;

	let mut cipher_text = nonce.to_vec();
	
	fs::write(&self.file_path, cipher_text)?;

	Ok(())
    }
    
    fn location(&self) -> &PathBuf {
	&self.file_path
    }	

}

#[derive(Default)]
pub struct Path;

impl Path{
    fn secure_path(&self, app_name: &str) ->io::Result<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            if let Some(proj_dirs) = ProjectDirs::from("com", "SyncSphereXd", app_name) {
                let path = proj_dirs.data_dir().join("syncsphere-xd.dat");
		
		if let Some(parent) = path.parent() {
                    fs::create_dir_all(&path)?;
                }

                return Ok(path);
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(proj_dirs) = ProjectDirs::from("com", "SyncSphereXd", app_name) {
                let path = proj_dirs.data_dir().join("syncsphere-xd.dat");

		if let Some(parent) = path.parent() {
                   fs::create_dir_all(&path)?;
                } 
                return Ok(path);
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(proj_dirs) = ProjectDirs::from("com", "SyncSphereXd", app_name){
                let path = proj_dirs.data_dir().join("syncsphere-xd.dat");

		if let Some(parent) = path.parent(){
                     fs::create_dir_all(&path)?;
                }
                return Ok(path);
            }
        }
        #[cfg(target_os = "ios")]
        {
            let mut path = PathBuf::from("Documents")
		.join(app_name)
		.join("syncsphere-xd.dat");
            if let Some(parent) = path.parent(){
                fs::create_dir_all(&path)?;
	    }
            Ok(path)
        }

        #[cfg(target_os = "android")]
        {
            let mut path = PathBuf::from("/storage/emulated/0")
            	.join("Documents")
		.join(app_name)
		.join("syncsphere-xd.dat");
            if let Some(parent) = path.parent(){
                fs::create_dir_all(&path)?;
            }
            Ok(path)
        }

        let mut fallback = PathBuf::from(".");
        fallback.push(app_name);
        fs::create_dir_all(&fallback)?;
        Ok(fallback)
    }
}
