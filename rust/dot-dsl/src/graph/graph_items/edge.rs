use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    node1: String,
    node2: String,
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(node1: &str, node2: &str) -> Self {
        Edge {
            node1: node1.to_string(),
            node2: node2.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        self
    }
}
