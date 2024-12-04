use std::path::PathBuf;
use crate::error::RepTreeError;
use crate::models::node::Node;

pub fn remove_element_from_repository_tree(
    root: &mut Node,
    element: &PathBuf,
) -> Result<(), RepTreeError> {
    if let Some(pos) = root
        .get_mut_children()
        .and_then(|children| children.iter().position(|child| child.get_path() == *element)) 
    {
        root.get_mut_children().unwrap().remove(pos);
    } else {
        if let Some(children) = root.get_mut_children() {
            for child in children {
                remove_element_from_repository_tree(child, element)?;
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::features::remove_element_from_repository_tree::remove_element_from_repository_tree;
    use crate::models::node::Node;

    #[test]
    fn test_should_remove_blob() {
        let b1 = Node::create_blob_node("add".to_string(), "hello".to_string(), PathBuf::from("src/features/add"));
        let t2 = Node::create_tree_node("features".to_string(), vec![b1], PathBuf::from("src/features"));
        let mut t1 = Node::create_tree_node("src".to_string(), vec![t2], PathBuf::from("src"));
        remove_element_from_repository_tree(&mut t1, &PathBuf::from("src/features/add")).expect("Error removing node");
        assert_eq!(1, t1.get_children().unwrap().len());
        assert_eq!(0, t1.get_children().unwrap()[0].get_children().unwrap().len());
    }
    
    #[test]
    fn test_should_remove_tree() {
        let b1 = Node::create_blob_node("add".to_string(), "hello".to_string(), PathBuf::from("src/features/add"));
        let t2 = Node::create_tree_node("features".to_string(), vec![b1], PathBuf::from("src/features"));
        let mut t1 = Node::create_tree_node("src".to_string(), vec![t2], PathBuf::from("src"));
        remove_element_from_repository_tree(&mut t1, &PathBuf::from("src/features")).expect("Error removing node");
        assert_eq!(0, t1.get_children().unwrap().len());
    }
}