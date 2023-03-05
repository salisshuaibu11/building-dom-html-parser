use std::collections::{HashMap, HashSet};
use std::fmt;

struct Node {
    children: Vec<Node>,
    node_type: NodeType
}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String)
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    fn new(tag_name: String, attributes: AttrMap) -> ElementData {
        ElementData { tag_name, attributes }
    }

    fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            // If we get back an option of "Some" s
            // we split and push it to hashset
            Some(s) => s.split(' ').collect(),
            None => HashSet::new()
        }
    }
}

type AttrMap = HashMap<String, String>;

impl Node {
    fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e)
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut attributes_string = String::new();

        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!("{}=\"{}\"", attr, value));
        }

        write!(f, "<{},{}>", self.tag_name, attributes_string)
    }
}

// Allows us to print our Node as well as its descendeds with indentations
fn pretty_print(n: &Node, indent_size: usize) {
    // Iterate from 0 to indent_size and then map a closure
    // That will return a space into a vector string and this
    // Will give us approprite indent size for everything that we need
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Comment(ref c) => println!("{}<!---{}--->", indent, c)
    }

    for child in n.children.iter() {
        pretty_print(&child, indent_size + 2)
    }

    match n.node_type {
        NodeType::Element(ref e) => println!("{}</{}>", indent, e.tag_name),
        _ => {}
    }
}