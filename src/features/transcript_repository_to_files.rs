use std::cmp::PartialEq;
use std::{fs, io};
use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::PathBuf;
use crate::error::RepTreeError;
use crate::models::blob::Blob;
use crate::models::node::Node;
use crate::models::tree::Tree;

/// Represents the different operational modes for the program.
///
/// The `Mode` enum defines three possible modes:
/// - `Partial`: A partial mode, used for certain limited operations.
/// - `Modify`: A modification mode, which allows for modifying a directory without deleting it.
/// - `Complete`: A complete mode that may involve more extensive operations, including potentially
///   deleting and recreating directories. 
#[derive(PartialEq)]
pub enum Mode {
    Partial,
    Modify,
    Complete,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Partial => "PARTIAL",
            Mode::Modify => "MODIFY",
            Mode::Complete => "COMPLETE",
        }
    }
}

pub fn transcript_repository_tree_to_files(root: &Node, path: &PathBuf, mode: &Mode) -> Result<(), RepTreeError> {
    if !path.exists() {
        Err(RepTreeError::IoError(io::Error::new(ErrorKind::NotFound, "path not found")))?;
    }
    match root {
        Node::BlobNode(blob) => {
            create_file(blob, path, mode)?;
        },
        Node::TreeNode(tree) => {
            let directory_path = create_directory(tree, path, mode)?;
            if directory_path != PathBuf::from("") {
                for node in tree.get_children().iter() {
                    transcript_repository_tree_to_files(node, &directory_path, mode)?;
                }
            }
        }
    }
    Ok(())
}

fn create_file(blob: &Blob, path: &PathBuf, mode: &Mode) -> Result<(), RepTreeError> {
    let file_path = path.join(blob.get_name());
    if *mode != Mode::Partial || !file_path.is_file() {
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(file_path).map_err(RepTreeError::IoError)?;

        file.write_all(blob.get_content().as_bytes()).map_err(RepTreeError::IoError)?;
    }
    Ok(())
}

fn create_directory(tree: &Tree, path: &PathBuf, mode: &Mode) -> Result<PathBuf, RepTreeError> {
    let directory_path = path.join(tree.get_name());
    let exist = directory_path.is_dir();
    match mode { 
        Mode::Partial => {
            Ok(PathBuf::from(""))
        },
        Mode::Modify => {
            if !exist {
                fs::create_dir(directory_path.clone()).map_err(RepTreeError::IoError)?;
            }
            Ok(directory_path)
        },
        Mode::Complete => {
            if exist {
                fs::remove_dir_all(directory_path.clone()).map_err(RepTreeError::IoError)?;
            }
            fs::create_dir(directory_path.clone()).map_err(RepTreeError::IoError)?;
            Ok(directory_path)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::{File, OpenOptions};
    use std::io::Read;
    use std::path::PathBuf;
    use crate::features::transcript_repository_to_files::{create_file, Mode};
    use crate::models::blob::Blob;

    #[test]
    fn test_should_create_file(){
        let blob = Blob::new("blob".to_string(), "Hello, World".to_string());
        create_file(&blob, &PathBuf::from(""), &Mode::Complete).unwrap();
        let mut content = String::from("");
        let mut file = OpenOptions::new()
            .read(true)
            .open("blob").unwrap();
        file.read_to_string(&mut content).unwrap();
        
        assert!(PathBuf::from("blob").is_file());
        assert_eq!("Hello, World", content);
        
        fs::remove_file("blob").unwrap();
    }
    
    #[test]
    fn test_should_not_create_file(){
        File::create("blob1").unwrap();
        
        let blob = Blob::new("blob1".to_string(), "Hello, World".to_string());
        create_file(&blob, &PathBuf::from(""), &Mode::Partial).unwrap();
        
        let mut content = String::from("");
        let mut file = OpenOptions::new()
            .read(true)
            .open("blob1").unwrap();
        file.read_to_string(&mut content).unwrap();

        assert_eq!("", content);

        fs::remove_file("blob1").unwrap();
    }
}