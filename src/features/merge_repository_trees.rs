use crate::models::blob::Blob;
use crate::models::node::Node;
use crate::models::node::Node::{BlobNode, TreeNode};

#[derive(PartialEq)]
pub enum Mode {
    Partial,
    Complete
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Partial => "PARTIAL",
            Mode::Complete => "COMPLETE",
        }
    }
}

pub fn merge_repository_trees(n1: Node, n2: Node, mode: &Mode) -> Option<Node>{
    if n1.is_root() && n1.get_id() == n2.get_id() {
        return None;
    }
    
    match (n1, n2) {
        (BlobNode(mut b1), BlobNode(b2)) => {
            match mode {
                Mode::Partial => {
                    Some(BlobNode(b2))
                },
                Mode::Complete => {
                    merge_blob(&mut b1, &b2);
                    Some(BlobNode(b1))
                }
            }
        },
        (TreeNode(mut t1), TreeNode(t2)) => {
            for node1 in t1.get_children() {
                let name_node1 = node1.get_name();
                for node2 in t2.get_children() {
                    if name_node1 == node2.get_name() && Node::is_same_type(&node1, &node2) {
                        if let Some(result) = merge_repository_trees(node1.clone(), node2, mode) {
                            t1.replace_node_among_children(result);
                        }
                    }
                }
            }

            for node in t2.get_children().iter() {
                if !t1.exist_node_same_name_and_type(node) {
                    t1.add_node(node.to_owned())
                }
            }
            
            Some(TreeNode(t1))
        },
        _ => None
    }
}

fn merge_blob(b1: &mut Blob, b2: &Blob) {
    b1.set_content(merge_content(&b1.get_content(), &b2.get_content()));
}

fn merge_content(content1: &String, content2: &String) -> String {
    let mut result = String::new();
    let mut in_conflict = false;

    for diff in diff::lines(&content1, &content2) {
        match diff {
            diff::Result::Left(old) => {
                if !in_conflict {
                    result.push_str("\n<<<<<< HEAD (current change)\n");
                    in_conflict = true;
                }
                result.push_str(old);
            }
            diff::Result::Right(new) => {
                if in_conflict {
                    result.push_str("\n======\n");
                }
                result.push_str(new);
                result.push_str("\n>>>>>> (incoming change)\n");
                in_conflict = false; // End the conflict block
            }
            diff::Result::Both(common, _) => {
                if in_conflict {
                    result.push_str("\n======\n>>>>>> (incoming change)\n");
                    in_conflict = false; // Close the conflict block
                }
                result.push_str(common);
            }
        }
    }

    if in_conflict {
        result.push_str("\n======\n>>>>>> (incoming change)\n");
    }

    result
}

#[cfg(test)]
mod test {
    use crate::features::merge_repository_trees::merge_blob;
    use crate::models::blob::Blob;

    #[test]
    fn test_merge() {
        let text1 = r#"def add(a, b):
    return a + b

# Test
print(add(2, 3))"#.to_string();

        let text2 = r#"def add(a, b):
    return a + b + 1

# Test
print(add(2, 4))"#.to_string();

        let mut b1 = Blob::new("blob1".to_string(), text1);
        let b2 = Blob::new("blob2".to_string(), text2);

        let result = r#"def add(a, b):
<<<<<< HEAD (current change)
    return a + b
======
    return a + b + 1
>>>>>> (incoming change)
# Test
<<<<<< HEAD (current change)
print(add(2, 3))
======
print(add(2, 4))
>>>>>> (incoming change)
"#.to_string();

        merge_blob(&mut b1, &b2);
        assert_eq!(b1.get_content(), result);
    }
}