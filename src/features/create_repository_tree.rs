use std::path::{Path, PathBuf};
use std::fs::{self, File};
use crate::error::RepTreeError;
use crate::models::blob::Blob;
use crate::models::node::Node;
use crate::models::node::Node::{BlobNode, TreeNode};
use crate::models::tree::Tree;


/// Create a repository tree
///
/// # Arguments
///
/// `paths`- vector of `PathBuf` to add
///
/// # Returns
///
/// Result with root `node`, or `RepTreeError` if error
pub fn create_repository_tree(paths: Vec<PathBuf>) -> Result<Node,RepTreeError> {
    let tree: Tree = Tree::default();
    let mut root = TreeNode(tree);

    for path in paths.iter() {
        let mut directories_and_file = get_elements(path);
        if !directories_and_file.is_empty() {
            add_node_to_repository_tree(&mut root, &mut directories_and_file)?;
        }
    }

    Ok(root)
}

/// Return list of elements which composed specified path
///
/// # Arguments
///
/// * 'path' - path of file or directory
///
/// # Returns
///
/// List of elements which composed specified path
fn get_elements(path: &PathBuf) -> Vec<&Path>{
    let mut ancestors: Vec<_> = path.ancestors().collect();
    ancestors.pop();
    ancestors.reverse();
    ancestors
}

/// Add a `node` to the repository tree
///
/// # Arguments
///
/// * 'node' - `TreeNode` to which the rest of the tree will be added
fn add_node_to_repository_tree(node: &mut Node, paths: &mut Vec<&Path>) -> Result<(), RepTreeError>{
    if paths.is_empty() {
        return Ok(())
    }

    let element_path = paths[0];

    let file_name = element_path.file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            RepTreeError::UnexpectedComportement(format!("Failed to get file name: {:?}", element_path))
        })?;

    if element_path.is_dir() {
        _add_tree_node_to_repository_tree(node, paths, file_name)?;
    } else if element_path.is_file() {
        _add_blob_node_to_repository_tree(node, element_path, file_name)?;
    }

    Ok(())
}

fn _add_tree_node_to_repository_tree(node: &mut Node, paths: &mut Vec<&Path>, file_name: &str) -> Result<(), RepTreeError> {
    let mut tree_node: Node = Node::create_tree_node(file_name.to_string(), vec![]);

    if let Some(mut already_existing_node) = node.find_child(&tree_node) {
        paths.remove(0);
        add_node_to_repository_tree(&mut already_existing_node, paths)?;
    } else {
        paths.remove(0);
        add_node_to_repository_tree(&mut tree_node, paths)?;
        node.add_node_to_tree_node(tree_node);
    }

    Ok(())
}

fn _add_blob_node_to_repository_tree(node: &mut Node, element_path: &Path, file_name: &str) -> Result<(), RepTreeError> {
    let mut blob: Blob = Blob::new(file_name.to_string(), "".to_string());
    blob.set_content_from_file(element_path)?;
    node.add_node_to_tree_node(BlobNode(blob));
    Ok(())
}

#[ctor::ctor]
fn setup(){
    let dir_path = ".tmp/foo/feat";
    if !Path::new(dir_path).exists() {
        fs::create_dir_all(dir_path).expect("Failed to create directory");
    }
    File::create(format!("{}/hello.txt", dir_path)).expect("Failed to create hello.txt");
    File::create(format!("{}/world.txt", dir_path)).expect("Failed to create world.txt");
}

#[ctor::dtor]
fn teardown(){
    if Path::new(".tmp").exists() {
        fs::remove_dir_all(".tmp").expect("Failed to remove .tmp directory");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use super::*;


    #[test]
    fn should_pass_get_elements(){
        let path_buf: PathBuf = PathBuf::from("src/features/hello");
        let elements = get_elements(&path_buf);
        let expected: Vec<&Path> = vec![
            &Path::new("src"),
            &Path::new("src/features"),
            &Path::new("src/features/hello")
        ];
        assert_eq!(elements, expected)
    }

    #[test]
    fn should_add_tree_node_to_repository() {
        let mut root_node: Node = Node::create_tree_node("".to_string(), vec![]);
        let mut paths: Vec<&Path> = vec![
            &Path::new(".tmp"),
        ];

        let result = add_node_to_repository_tree(&mut root_node, &mut paths);

        let binding = root_node.get_children().unwrap();
        let child: &Node = binding.get(0).unwrap();

        assert!(result.is_ok());
        assert_eq!(root_node.get_children().unwrap().len(), 1);
        assert_eq!(child.get_name(), ".tmp");
        assert!(child.is_tree())
    }

    #[test]
    fn should_add_blob_node_to_repository() {
        File::create("tmp1").unwrap();

        let mut root_node: Node = Node::create_tree_node("".to_string(), vec![]);
        let mut paths: Vec<&Path> = vec![
            &Path::new("tmp1"),
        ];

        let result = add_node_to_repository_tree(&mut root_node, &mut paths);

        let binding = root_node.get_children().unwrap();
        let child: &Node = binding.get(0).unwrap();

        fs::remove_file("tmp1").unwrap();

        assert!(result.is_ok());
        assert_eq!(root_node.get_children().unwrap().len(), 1);
        assert_eq!(child.get_name(), "tmp1");
        assert!(child.is_blob());
    }

    #[test]
    fn should_not_add_tree_node_to_repository(){
        let mut file = File::create("tmp2").unwrap();
        file.write_all(b"Hello, World").unwrap();

        let mut root_node: Node = Node::create_tree_node("".to_string(), vec![]);
        let mut paths: Vec<&Path> = vec![
            &Path::new("tmp2"),
        ];

        let result = add_node_to_repository_tree(&mut root_node, &mut paths);

        file.write_all(b", Everybody").unwrap();
        let result = add_node_to_repository_tree(&mut root_node, &mut paths);

        let binding = root_node.get_children().unwrap();
        let child: &Node = binding.get(0).unwrap();

        fs::remove_file("tmp2").unwrap();

        assert!(result.is_ok());
        assert_eq!(root_node.get_children().unwrap().len(), 1);
        assert_eq!(child.get_name(), "tmp2");
        assert!(child.is_blob());
        assert_eq!(child.get_content().unwrap(), "Hello, World, Everybody");
    }
}
