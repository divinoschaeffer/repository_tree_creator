use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;
use repository_tree_creator::features::transcript_repository_to_files::{Mode, transcript_repository_tree_to_files};
use repository_tree_creator::models::blob::Blob;
use repository_tree_creator::models::node::Node::{BlobNode, TreeNode};
use repository_tree_creator::models::tree::Tree;

fn setup() {
    create_dir_all("tmp1/dir1").unwrap();
}

fn teardown() {
    remove_dir_all("tmp1").unwrap();
}

#[test]
fn should_transcript_repository_to_files() {
    setup();
    
    let mut b1 = Blob::new("file1".to_string(),"content".to_string());
    b1.set_id(String::from("1234567891"));
    let mut r2 = Tree::new("dir2".to_string(), vec![]);
    r2.set_id(String::from("1234567892"));
    let mut r1 = Tree::new("dir1".to_string(), vec![BlobNode(b1), TreeNode(r2)]);
    r1.set_id(String::from("1234567890"));
    
    transcript_repository_tree_to_files(&TreeNode(r1), &PathBuf::from("tmp1"), &Mode::Complete).unwrap();
    
    assert!(PathBuf::from("tmp1/dir1").is_dir());
    assert!(PathBuf::from("tmp1/dir1/file1").is_file());
    assert!(PathBuf::from("tmp1/dir1/dir2").is_dir());
    
    teardown();
}