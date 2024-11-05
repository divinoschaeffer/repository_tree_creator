use std::fs::{File, OpenOptions};
use std::io::Write;
use std::io::BufWriter;
use std::path::PathBuf;
use crate::error::RepTreeError;
use crate::models::blob::Blob;
use crate::models::node::Node;
use crate::models::tree::Tree;

pub fn transcript_repository_to_object_files(root: &Node, path: &PathBuf) -> Result<(), RepTreeError>{
    let (filepath, filename) = create_details(root, path.clone());
    let file: File = create_file(&filepath, filename)?;
    let mut writer  = BufWriter::new(file);
    match root {
        Node::BlobNode( blob) => {
            transcript_blob(blob, &mut writer)?;
        },
        Node::TreeNode(tree) => {
            for node in tree.get_children().iter() {
                transcript_repository_to_object_files(node, path)?;
            }
            transcript_tree(tree, &mut writer)?;
        }
    }
    Ok(())
}

fn create_details(node: &Node, path_buf: PathBuf) -> (PathBuf, String){
    let id: String = node.get_id();
    let directory: String = id.chars().take(2).collect();
    let filename: String = id.chars().skip(2).collect();
    let path: PathBuf = path_buf.join(directory);
    (path, filename)
}

fn create_file(path_buf: &PathBuf, filename: String) -> Result<File, RepTreeError> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(path_buf.join(filename))
        .map_err(RepTreeError::IoError)?;
    Ok(file)
}

fn transcript_tree(tree: &Tree, writer: &mut BufWriter<File>) -> Result<(), RepTreeError> {
    for node in tree.get_children().iter() {
        match node {
            Node::TreeNode(tree)  => {
                writeln!(writer, "TREE {} {}", tree.get_id(), tree.get_name()).map_err(RepTreeError::IoError)?;
            },
            Node::BlobNode(blob) => {
                writeln!(writer, "BLOB {} {}", blob.get_id(), blob.get_name()).map_err(RepTreeError::IoError)?;
            }
        }
    }
    Ok(())
}

fn transcript_blob(blob: &Blob, writer: &mut BufWriter<File>) -> Result<(), RepTreeError> {
    write!(writer, "{}", blob.get_content()).map_err(RepTreeError::IoError)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::{File, OpenOptions};
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::path::PathBuf;
    use crate::features::transcript_repository_tree_to_file::{create_details, create_file, transcript_blob, transcript_tree};
    use crate::models::blob::Blob;
    use crate::models::node::Node::BlobNode;
    use crate::models::tree::Tree;

    #[test]
    fn test_should_transcript_blob() {
        let file_path = "tmp3";

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .expect("Failed to create file for writing");
        let mut writer = BufWriter::new(file);
        let blob = Blob::new(String::from("HAHA"), String::from("Hello"));
        transcript_blob(&blob, &mut writer).expect("Failed to transcript blob");
        
        writer.flush().expect("Failed to flush");

        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect("Failed to open file for reading");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file content");

        assert!(PathBuf::from(file_path).is_file(), "File should exist");
        assert_eq!("Hello", content, "Content should match");

        std::fs::remove_file(file_path).unwrap_or_else(|_| println!("Failed to delete temporary file"));
    }
    
    #[test]
    fn test_should_transcript_tree(){
        let file_path = "tmp4";

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .expect("Failed to create file for writing");
        let mut writer = BufWriter::new(file);
        let blob = Blob::new(String::from("HAHA"), String::from("Hello"));
        let tree = Tree::new("tree".to_string(), vec![BlobNode(blob)]);
        transcript_tree(&tree, &mut writer).expect("Failed to transcript");

        writer.flush().expect("Failed to flush");

        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect("Failed to open file for reading");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file content");

        assert!(PathBuf::from(file_path).is_file(), "File should exist");
        assert_eq!("BLOB  HAHA\n", content);

        std::fs::remove_file(file_path).unwrap_or_else(|_| println!("Failed to delete temporary file"));
    }
    
    #[test]
    fn should_create_file(){
        let filename = String::from("tmp5");
        create_file(&PathBuf::from(""), filename.clone()).expect("Failed to create");
        assert!(PathBuf::from("tmp5").is_file(), "File should exist");
        std::fs::remove_file(filename).unwrap_or_else(|_| println!("Failed to delete temporary file"));
    }
    
    #[test]
    fn should_create_details(){
        let mut blob = Blob::new("HI".to_string(), "Hello".to_string());
        blob.set_id("12345667890".to_string());
        
        let (directory, filename) = create_details(&BlobNode(blob), PathBuf::from(""));
        assert_eq!(PathBuf::from("12"), directory);
        assert_eq!("345667890", filename);
        
    }
}