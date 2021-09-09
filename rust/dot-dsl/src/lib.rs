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
            None
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            self
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            self
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            self
        }
    }

    pub mod graph_items;
}
