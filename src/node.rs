use crate::client::RawKV;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Key(String);

impl Key {
    pub fn new(k: &str) -> Key { 
        Key(k.to_string()) 
    }

    pub fn key(&self) -> &String {
        &self.0
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Node {
    KeyValue {
        key: String,
        value: String,
    },
    Directory {
        key: String,
        nodes: HashMap<Key, Node>,
    },
}

impl Node {
    pub fn nodes(&self) -> Vec<&Node> {
        match self {
            Node::KeyValue{..} => vec!(self),
            Node::Directory{nodes, ..} => {
                nodes.values().collect()
            }
        }
    }
    fn insert_key(nodes: &mut HashMap<Key, Node>, key: String, value: String) {
        match nodes.entry(Key(key.clone())) {
            Entry::Occupied(mut state) => {
                state.insert(Node::KeyValue {
                    key: key,
                    value: value,
                });
            }
            Entry::Vacant(state) => {
                state.insert(Node::KeyValue {
                    key: key,
                    value: value,
                });
            }
        }
    }
    fn branch(nodes: &mut HashMap<Key, Node>, key: String, remainder: Vec<String>, value: String) {
        match nodes.entry(Key(key.clone())) {
            Entry::Occupied(mut state) => {
                // We already have a branch of that name, we just
                // forward the call and move on
                state.get_mut().add_value(remainder, value)
            }
            Entry::Vacant(state) => {
                // We need to create the node
                let mut node = Node::Directory {
                    key: key,
                    nodes: HashMap::new(),
                };
                let status = node.add_value(remainder, value);
                state.insert(node);
                status
            }
        };
    }
    pub fn get(&self, test: Key) -> Option<&Node> {
        let key = &test;
        match self {
            Node::Directory {
                key: _key,
                ref nodes,
            } => nodes.get(key),
            _ => None,
        }
    }
    pub fn add_value(&mut self, mut path: Vec<String>, value: String) {
        (match self {
            Node::KeyValue {
                key: _key,
                value: _value,
            } => None,
            Node::Directory {
                key: _key,
                ref mut nodes,
            } => Some(nodes),
        })
        .map(|contents| match path.len() {
            0 => panic!("Path cannot be empty"),
            1 => Node::insert_key(contents, path.pop().unwrap(), value),
            _ => Node::branch(contents, path.pop().unwrap(), path, value),
        });
    }
}

pub fn into_tree(collection: Vec<RawKV>) -> Node {
    let mut root_node = Node::Directory {
        key: "/".to_string(),
        nodes: HashMap::new(),
    };
    for node in collection {
        let mut path_elements: Vec<String> = node.key.split("/").map(|r| r.to_string()).collect();
        path_elements.reverse();
        root_node.add_value(path_elements, node.value);
    }
    root_node
}
