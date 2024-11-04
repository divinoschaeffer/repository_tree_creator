use dit_id_generator::features::generator::generate;
use dit_id_generator::traits::generator::Generator;
use crate::error::RepTreeError;
use crate::models::node::Node;

#[derive(Clone, Debug, Default)]
pub struct Tree {
    id: String,
    name: String,
    children: Vec<Node>
}

impl Tree {

    /// Create Tree with all fields empty
    ///
    /// # Returns
    ///
    /// Return a Tree
    pub fn default() -> Tree {
        Tree {
            id: "".to_string(),
            name: "".to_string(),
            children: vec![]
        }
    }

    /// Create Tree
    ///
    /// # Arguments
    ///
    /// * `name` - directory name
    /// * `children` - list directory's elements
    ///
    /// # Returns
    ///
    /// Return a Tree
    pub fn new(name: String, children: Vec<Node>) -> Tree {
        Tree {
            id: "".to_string(),
            name,
            children
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn set_id(&mut self, id: String){
        self.id = id;
    }
    
    pub fn get_children(&self) -> Vec<Node> {
        self.children.clone()
    }

    pub fn find_child(&mut self, node: &Node) -> Option<&mut Node> {
        self.children.iter_mut().find(|x| x.has_same_name(node) && x.is_tree())
    }
    
    /// Check if two `Tree` instances has same name
    /// 
    /// # Arguments
    /// * `other`- reference to the tree
    /// 
    /// # Returns    
    ///  `True` if both trees have the same name, otherwise `false`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use repository_tree_creator::models::tree::Tree;
    /// let t1 = Tree::new(String::from("Oak"),vec![]);
    /// let t2 = Tree::new(String::from("Oak"),vec![]);
    /// let t3 = Tree::new(String::from("Pine"),vec![]);
    /// 
    /// assert!(t1.has_same_name(&t2));
    /// assert!(!t1.has_same_name(&t3));
    /// ```
    pub fn has_same_name(&self, other: &Tree) -> bool {
        self.name == other.name
    }

    /// Add node to tree or replace a node if a node with same name and type already exist
    /// 
    /// # Arguments
    /// 
    /// * `node` - Node
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use repository_tree_creator::models::blob::Blob;
    /// # use repository_tree_creator::models::node::Node;
    /// # use repository_tree_creator::models::node::Node::{BlobNode, TreeNode};
    /// # use repository_tree_creator::models::tree::Tree;
    /// # let mut t1 = Tree::new(String::from("Pine"), vec![]);
    /// # let mut t2 = Tree::new(String::from("Oak"), vec![]);
    /// # let mut t3 = Tree::new(String::from("Syrup"), vec![]);
    /// # let blob = Blob::new(String::from("Ball"), String::from("booing booing"));
    /// # let tree = Tree::new(String::from("Ball"), vec![]);
    /// # let blob_node:Node = BlobNode(blob);
    /// # let tree_node: Node = TreeNode(tree);
    ///
    /// t1.add_node(blob_node.clone());
    ///
    /// t2.add_node(blob_node.clone());
    /// t2.add_node(blob_node.clone());
    /// 
    /// t3.add_node(blob_node);
    /// t3.add_node(tree_node);
    ///
    /// assert_eq!(t1.get_children().len(), 1);
    /// assert_eq!(t2.get_children().len(), 1);
    /// assert_eq!(t3.get_children().len(), 2);
    /// ```
    pub fn add_node(&mut self, node: Node) {

        if let Some(position) = self.children
            .iter()
            .position(|n| Node::has_same_name(n, &node) && Node::is_same_type(n, &node)) {
            
            self.children.remove(position);
        }
        
        self.children.push(node);
    }
}

impl Generator for Tree {
    fn generate_id(&mut self) -> String {
        let mut content = String::from("");
        return if self.children.len() == 0 {
            let content = self.get_name().clone();
            let id = generate(content);
            self.set_id(id.clone());
            id
        } else {
            for node in self.children.iter_mut() {
                node.generate_id();
                content += &*node.get_id()
            }
            let id = generate(content);
            self.set_id(id.clone());
            id
        }
    }
}