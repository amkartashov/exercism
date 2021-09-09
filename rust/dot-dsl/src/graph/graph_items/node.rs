use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    name: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Node {
            name: self.name,
            attrs: attrs
                .iter()
                .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                .collect(),
        }
    }

    pub fn get_attr(&self, name: &str) -> Option<&str> {
        None
    }
}
