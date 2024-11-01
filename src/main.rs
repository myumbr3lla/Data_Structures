#[derive(Clone)]

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i16,
}

impl Node {
    fn new(value: i16) -> Self {
        Node {
            left: None,
            right: None,
            value,
        }
    }

    fn insert(&mut self, value: i16) {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Less => {
                if let Some(left) = self.left.as_mut() {
                    left.insert(value);
                } else {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
            _ => {
                if let Some(right) = self.right.as_mut() {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
        }
    }

    fn search(&self, value: i16) -> Option<&Node> {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Less => self.left.as_ref()?.search(value),
            std::cmp::Ordering::Equal => Some(self),
            std::cmp::Ordering::Greater => self.right.as_ref()?.search(value),
        }
    }

    fn delete(&mut self, value: i16) -> Option<Box<Node>> {
        match value.cmp(&self.value) {
            std::cmp::Ordering::Less => {
                self.left.as_mut().map(|left| left.delete(value));
            },
            std::cmp::Ordering::Equal => match (&mut self.left, &mut self.right) {
                (None, right) => return right.take(),
                (left, None) => return left.take(),
                (_, Some(right_value)) => {
                    self.value = right_value.min_node().value;
                    self.right = right_value.delete(self.value);
                }
            },
            std::cmp::Ordering::Greater => {
                self.right.as_mut().map(|right| right.delete(value));
            }
        }

        Some(Box::new(self.clone()))
    }

    fn min_node(&self) -> &Node {
        self.left
            .as_ref()
            .map(|left| left.min_node())
            .unwrap_or(self)
    }

    fn traverse(&self) {
        if let Some(ref left) = self.left {
            left.traverse();
        }

        println!("{}", self.value);

        if let Some(ref right) = self.right {
            right.traverse();
        }
    }
}


fn new(slice: Vec<i16>) -> Option<Box<Node>> {
    let mut root: Option<Box<Node>> = None;

    for &val in &slice {
        if let Some(node) = root.as_mut() {
            node.insert(val);
        } else {
            root = Some(Box::new(Node::new(val)));
        }
    }

    root
}

fn main() {
    let mut root = new(vec![5, 3, 8, 1, 4]).unwrap();

    root.insert(6);
    root.insert(7);

    root.delete(3);
    if root.search(3).is_none() {
        println!("Node deleted successfully")
    }

    if let Some(found_node) = root.search(3) {
        println!("Found node with value: {}", found_node.value)
    } else {
        println!("Node not found")
    }

    root.traverse()
}
