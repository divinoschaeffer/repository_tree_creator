use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path};
use dit_id_generator::features::generator::generate;
use crate::error::RepTreeError;
use dit_id_generator::traits::generator::Generator;

pub const BLOB: &str = "BLOB";
#[derive(Clone, Default, Debug)]
pub struct Blob{
    id: String,
    name: String,
    content: String
}

impl Blob {
    
    pub fn default() -> Blob{
        Blob {
            id: "".to_string(),
            name: "".to_string(),
            content: "".to_string()
        }
    }
    
    pub fn new(name: String, content: String) -> Blob {
        Blob {
            id: "".to_string(),
            name,
            content
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn set_id(&mut self, id: String){
        self.id = id;
    }
    
    pub fn get_content(&self) -> String {
        self.content.to_string()
    }

    /// Check if two `Blob` instances has same name
    ///
    /// # Arguments
    /// * `other`- reference to the blob
    ///
    /// # Returns    
    ///  `True` if both blobs have the same name, otherwise `false`
    ///
    /// # Examples
    ///
    /// ```
    /// use repository_tree_creator::models::blob::Blob;
    /// let b1 = Blob::new(String::from("Oak"),String::from(""));
    /// let b2 = Blob::new(String::from("Oak"),String::from(""));
    /// let b3 = Blob::new(String::from("Pine"),String::from(""));
    ///
    /// assert!(b1.has_same_name(&b2));
    /// assert!(!b1.has_same_name(&b3));
    /// ```
    pub fn has_same_name(&self, other: &Blob) -> bool {
        self.name == other.name
    }
    
    pub fn set_content_from_file(&mut self, path_buf: &Path) -> Result<(),RepTreeError>{
        let file = File::open(path_buf).map_err(RepTreeError::IoError)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).map_err(RepTreeError::IoError)?;
        self.content = contents;
        self.generate_id();
        Ok(())
    }
    
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

impl Generator for Blob {
    fn generate_id(&mut self) -> String {
        let content = self.content.clone();
        let id = generate(content);
        self.set_id(id.clone());
        id
    }
}
