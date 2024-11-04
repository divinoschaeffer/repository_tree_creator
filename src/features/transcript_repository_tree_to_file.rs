use std::fs::File;
use std::io::Write;
use std::io::BufWriter;
use std::path::PathBuf;
use crate::error::RepTreeError;
use crate::models::blob::Blob;
use crate::models::node::Node;
use crate::models::tree::Tree;

pub fn transcript_repository_to_object_files(root: & Node, path: &PathBuf) -> Result<(), RepTreeError>{
    let (filepath, filename) = create_details(root, path.clone());
    let file: File = create_file(&filepath, filename)?;
    let mut writer  = BufWriter::new(file);
    match root {
        Node::BlobNode( blob) => {
            transcript_blob(blob, &mut writer)?;
        },
        Node::TreeNode(tree) => {
            for node in tree.get_children().iter() {
                transcript_tree(tree, &mut writer)?;
                transcript_repository_to_object_files(node, path)?;
            }
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
    let file: File = File::create(path_buf.join(filename)).map_err(RepTreeError::IoError)?;
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
    writeln!(writer, "{}", blob.get_content()).map_err(RepTreeError::IoError)?;
    Ok(())
}