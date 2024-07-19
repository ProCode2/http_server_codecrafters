use std::collections::HashMap;

use itertools::Itertools;

use super::{
    request::{Method, Request},
    response::Response,
};

type Handler = Box<dyn Fn(&Request) -> Response + Sync + Send>;

struct Node {
    handlers: HashMap<Method, Handler>,
    children: HashMap<String, Node>,
    param: Option<String>,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Node \n It also has handler tho :) {:?} {:?}"#,
            self.children, self.param
        )
    }
}

impl Node {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            children: HashMap::new(),
            param: None,
        }
    }
}

#[derive(Debug)]
pub struct Router {
    root: Node,
}

impl Router {
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn add_route<F>(&mut self, method: Method, origin_form: &str, f: F)
    where
        F: Fn(&Request) -> Response + 'static + Send + Sync,
    {
        let segments = origin_form.split('/').filter(|s| !s.is_empty());
        let mut current_node = &mut self.root;
        for segment in segments {
            let key = if segment.starts_with("{") && segment.ends_with("}") {
                // extract the value for this segment as a param for adding to req later
                let param = segment[1..segment.len() - 1].to_string();
                current_node.param = Some(param);
                "{param}".to_string()
            } else {
                segment.to_string()
            };
            current_node = current_node.children.entry(key).or_insert_with(Node::new);
        }

        current_node.handlers.insert(method, Box::new(f));
    }

    pub fn get_handler_and_params(
        &self,
        method: Method,
        origin_form: &str,
    ) -> (HashMap<String, String>, Option<&Handler>) {
        let segments = origin_form.split('/').filter(|s| !s.is_empty());
        let mut current_node = &self.root;
        let mut params = HashMap::new();
        let segs = segments.clone();
        for (i, segment) in segments.enumerate() {
            if let Some(next_node) = current_node.children.get(segment) {
                current_node = next_node;
            } else if let Some(p) = current_node.param.as_ref() {
                params.insert(
                    p.clone(),
                    segs.clone().collect::<Vec<&str>>()[i..]
                        .join("/")
                        .to_string(),
                );

                if let Some(next_node) = current_node.children.get("{param}") {
                    current_node = next_node;
                } else {
                    break;
                }
            }
        }

        if let Some(handler) = current_node.handlers.get(&method) {
            return (params, Some(handler));
        } else {
            return (params, None);
        }
    }
}
