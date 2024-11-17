use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use repository_tree_creator::models::blob::Blob;
use repository_tree_creator::models::node::Node::{BlobNode, TreeNode};
use repository_tree_creator::models::tree::Tree;
use repository_tree_creator as rtc;

fn setup(){
    fs::create_dir_all("tempdir1/12").unwrap();
}

fn teardown(){
    fs::remove_dir_all("tempdir1").unwrap()
}

#[test]
fn should_transcript_repository_to_object_files() {
    setup();
    
    let mut b1 = Blob::new("file1".to_string(),"content".to_string());
    b1.set_id(String::from("1234567891"));
    let mut r2 = Tree::new("dir2".to_string(), vec![]);
    r2.set_id(String::from("1234567892"));
    let mut r1 = Tree::new("dir1".to_string(), vec![BlobNode(b1), TreeNode(r2)]);
    r1.set_id(String::from("1234567890"));
    
    rtc::features::transcript_repository_tree_to_object_files::transcript_repository_to_object_files(&TreeNode(r1), &PathBuf::from("tempdir1")).unwrap();
    let mut file = OpenOptions::new()
        .read(true)
        .open("tempdir1/12/34567890")
        .unwrap();
    
    let mut content = String::from(""); 
    file.read_to_string(&mut content).unwrap();
    
    assert!(PathBuf::from("tempdir1/12/34567890").is_file());
    assert!(PathBuf::from("tempdir1/12/34567891").is_file());
    assert!(PathBuf::from("tempdir1/12/34567892").is_file());
    assert_eq!("BLOB 1234567891 file1
TREE 1234567892 dir2
", content);
    
    teardown();
}