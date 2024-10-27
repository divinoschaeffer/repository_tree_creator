use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use repository_tree_creator as rtc;
use repository_tree_creator::models::node::Node;

fn setup(){
    let dir_path = ".tmp/foo/feat";
    if !Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path).expect("Failed to create directory");
    }
    File::create(format!("{}/hello.txt", dir_path)).expect("Failed to create hello.txt");
    File::create(format!("{}/world.txt", dir_path)).expect("Failed to create world.txt");
}

fn teardown(){
    if Path::new(".tmp").exists() {
        fs::remove_dir_all(".tmp").expect("Failed to remove .tmp directory");
    }
}

#[test]
fn should_create_repository_tree(){
    setup();
    
    let paths: Vec<PathBuf> = vec![
        PathBuf::from(".tmp/foo/feat/hello.txt"),
        PathBuf::from(".tmp/foo/feat/world.txt")
    ];
    let mut node = rtc::features::create_repository_tree::create_repository_tree(paths).unwrap();
    
    teardown();
    
    let binding = node.get_children().unwrap();
    let mut tmp: Node = binding.get(0).unwrap().clone();

    let binding = tmp.get_children().unwrap();
    let mut foo: Node = binding.get(0).unwrap().clone();

    let binding = foo.get_children().unwrap();
    let mut feat: Node = binding.get(0).unwrap().clone();

    let binding = feat.get_children().unwrap();
    let hello: Node = binding.get(0).unwrap().clone();
    let world: Node = binding.get(1).unwrap().clone();
    
    assert_eq!(node.get_name(), "");
    assert_eq!(node.get_children().unwrap().len(), 1);
    assert_eq!(tmp.get_name(), ".tmp");
    assert_eq!(tmp.get_children().unwrap().len(), 1);
    assert_eq!(foo.get_name(), "foo");
    assert_eq!(tmp.get_children().unwrap().len(), 1);
    assert_eq!(feat.get_name(), "feat");
    assert_eq!(feat.get_children().unwrap().len(), 2);
    assert_eq!(hello.get_name(), "hello.txt");
    assert!(hello.is_blob());
    assert_eq!(world.get_name(), "world.txt");
    assert!(world.is_blob());
}
