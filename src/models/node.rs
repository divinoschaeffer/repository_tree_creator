use crate::models::blob::Blob;
use crate::models::node::Node::{BlobNode, TreeNode};
use crate::models::tree::Tree;

#[derive(Clone, Debug)]
pub enum Node {
    BlobNode(Blob),
    TreeNode(Tree),
}

impl Node {
    
    pub fn is_same_type(n1: &Node, n2: &Node) -> bool {
        match (n1, n2) {
            (BlobNode(_),BlobNode(_)) => true,
            (TreeNode(_),TreeNode(_)) => true,
            _ => false
        }
    }

    /// Check if `Node` instance has same name
    ///
    /// # Arguments
    /// * `other`- reference to the `Node`
    ///
    /// # Returns    
    ///  `True` if both `Node` have the same name, otherwise `false`
    ///
    /// # Examples
    ///
    /// ```
    /// # use repository_tree_creator::models::blob::Blob;
    /// # use repository_tree_creator::models::node::Node::{BlobNode, TreeNode};
    /// # use repository_tree_creator::models::tree::Tree;
    /// let n1 = TreeNode(
    ///     Tree::new(String::from("Oak"), vec![])
    /// );
    /// let n2 = TreeNode(
    ///     Tree::new(String::from("Oak"), vec![])
    /// );
    /// let n3 = BlobNode(
    ///     Blob::new(String::from("Pine"), String::from(""))
    /// );
    /// let n4 = BlobNode(
    ///     Blob::new(String::from("Oak"), String::from(""))
    /// );
    ///
    /// assert!(n1.has_same_name(&n2));
    /// assert!(!n3.has_same_name(&n4));
    /// assert!(n2.has_same_name(&n4));
    /// ```
    pub fn has_same_name(&self, other: &Node) -> bool {
        match (self, other) {
            (BlobNode(b1), BlobNode(b2)) => b1.has_same_name(b2),
            (TreeNode(t1),TreeNode(t2)) => t1.has_same_name(t2),
            (TreeNode(t1), BlobNode(b1)) => t1.get_name() == b1.get_name(),
            _ => false
        }
    }

    pub fn is_tree(&self) -> bool {
        match self {
            TreeNode(_) => true,
            _ => false
        }
    }

    pub fn is_blob(&self) -> bool {
        match self {
            BlobNode(_) => true,
            _ => false
        }
    }

    /// Create a `TreeNode`
    ///
    /// # Arguments
    ///
    /// * `name` - directory name
    /// * `children` - children list
    ///
    /// # Returns
    ///
    /// Return a Node of type TreeNode
    pub fn create_tree_node(name: String, children: Vec<Node>) -> Node {
        let tree: Tree = Tree::new(name, children);
        TreeNode(tree)
    }

    /// Create a `BlobNode`
    ///
    /// # Arguments
    ///
    /// * `name` - file name
    /// * `content` - file content
    ///
    /// # Returns
    ///
    /// Return a Node of type BlobNode
    pub fn create_blob_node(name: String, content: String) -> Node {
        let blob: Blob = Blob::new(name,content);
        BlobNode(blob)
    }

    /// Add or replace a `node` amongst `children`
    /// 
    /// # Arguments
    /// 
    /// * `future_child` - node to add
    /// 
    /// # Returns
    /// 
    /// `True` when function call on `TreeNode`, otherwise `false`
    pub fn add_node_to_tree_node(&mut self, future_child: Node) -> bool {
        match self {
            TreeNode(tree) => {
                tree.add_node(future_child);
                true
            }
            _ => false
        }
    }

    pub fn get_children(&mut self) -> Option<Vec<Node>> {
        match self {
            TreeNode(tree) => Some(tree.get_children()),
            _ => None
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            TreeNode(tree) => tree.get_name(),
            BlobNode(blob) => blob.get_name()
        }
    }

    /// Find child of a `TreeNode`
    ///
    /// # Returns
    ///
    /// Return `Option<&mut Node>`, `Some(&mut Node)` if node is `TreeNode`, otherwise `None`
    ///
    pub fn find_child(&mut self, node: &Node) ->  Option<&mut Node> {
        match self {
            TreeNode(tree) => tree.find_child(node),
            _ => None
        }
    }

    /// Get content of a `BlobNode`
    ///
    /// # Returns
    ///
    /// Return `Option<String>`, `Some(String)` if node is `BlobNode`, otherwise `None`
    ///
    /// # Examples
    ///
    /// ```
    /// # use repository_tree_creator::models::blob::Blob;
    /// # use repository_tree_creator::models::node::Node::{BlobNode, TreeNode};
    /// # use repository_tree_creator::models::tree::Tree;
    /// let b_node = BlobNode(Blob::new(String::from("Oak"), String::from("John Doe")));
    /// let t_node = TreeNode(Tree::new(String::from("Pine"), vec![]));
    ///
    /// assert_eq!(b_node.get_content(), Some(String::from("John Doe")));
    /// assert_eq!(t_node.get_content(), None);
    /// ```
    ///
    pub fn get_content(&self) -> Option<String> {
        match self {
            BlobNode(blob) => Some(blob.get_content()),
            _ => None
        }
    }
}
