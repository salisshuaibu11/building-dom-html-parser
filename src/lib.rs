use std::collections::HashMap;
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