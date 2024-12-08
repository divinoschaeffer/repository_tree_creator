use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;
use crate::error::RepTreeError;
use crate::models::blob::{BLOB, Blob};
use crate::models::node::Node::{BlobNode, TreeNode};
use crate::models::tree::Tree;
use crate::utils::open_object_file;

/// **Description**  
/// This function traverses an object file containing repository tree information 
/// and reconstructs the corresponding tree structure in the root `Tree` object. 
/// It identifies nodes as either blobs or subtrees and adds them to the root tree.
///
/// **Parameters**  
/// - `root`: A mutable reference to a `Tree` object representing the root tree.  
/// - `object_id`: A reference to a string containing the identifier of the root object.  
/// - `object_path`: A reference to a `PathBuf` containing the path to the object files.  
///
/// **Returns**  
/// - `Result<(), RepTreeError>`:  
///   - `Ok(())` on success.  
///   - `Err(RepTreeError)` if an I/O error or unexpected behavior occurs.
pub fn get_repository_tree_from_object_files(root: &mut Tree, object_id: &String, object_path: &PathBuf) -> Result<(), RepTreeError> {
    let reader = open_object_file(object_id, object_path).map_err(RepTreeError::IoError)?;
    let lines = BufReader::new(reader).lines();

    for line in lines {
        let content = match line {
            Ok(content) => content,
            Err(..) => {
                return Err(RepTreeError::UnexpectedComportment("Error while reading a file".to_string()));
            }
        };
        let id = &content[5..45];
        let name = &content[46..];

        if &content[0..4] == BLOB {
            get_blob_from_object_file(root, String::from(name), String::from(id), object_path)?
        } else {
            let mut new_tree = Tree::new(String::from(name), Vec::new());
            new_tree.set_id(String::from(id));
            new_tree.set_path(root.get_path().join(String::from(name)));
            get_repository_tree_from_object_files(&mut new_tree, &String::from(id), object_path)?;
            let node = TreeNode(new_tree);
            root.add_node(node);
        }
    }
    Ok(())
}

fn get_blob_from_object_file(root: &mut Tree, file_name: String, id: String, object_path: &PathBuf) -> Result<(), RepTreeError> {
    let mut reader = open_object_file(&String::from(id.clone()), object_path).map_err(RepTreeError::IoError)?;
    
    let mut contents = String::new();
    reader.read_to_string(&mut contents).map_err(RepTreeError::IoError)?;
    
    let mut blob = Blob::new(file_name.clone(), contents);
    blob.set_id(id);
    blob.set_path(root.get_path().join(file_name));
    let node = BlobNode(blob);
    
    root.add_node(node);
    Ok(())
}