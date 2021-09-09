use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
        Self {
            attrs: attrs
                .iter()
                .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                .collect(),
            ..self
        }
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(String::as_str)
    }
}
