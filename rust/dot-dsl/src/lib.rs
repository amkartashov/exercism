pub mod graph {
    use graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn get_node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.name == name).cloned()
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.iter().cloned().collect(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges: edges.iter().cloned().collect(),
                ..self
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

    pub mod graph_items;
}
