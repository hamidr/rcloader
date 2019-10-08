use crate::client::{ConsulKV, RawKV, ResulT};
use itertools::Itertools;
use serde::Serialize;
use std::cmp::Eq;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;


#[derive(Eq, PartialEq, Hash, Debug)]
pub enum Key {
    KeyValue(String),
    Directory(String),
}

impl Key {
    pub fn key(&self) -> String {
        match self {
            Key::KeyValue(k) => k.clone(),
            Key::Directory(k) => k.clone()
        }
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
    fn insert_key(nodes: &mut HashMap<Key, Node>, key: String, value: String) {
        match nodes.entry(Key::KeyValue(key.clone())) {
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
        match nodes.entry(Key::Directory(key.clone())) {
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
    pub fn get(&self, test: &Key) -> Option<&Node> {
        println!("{:?}", &self);
        match self {
            Node::Directory {
                key: _key,
                ref nodes,
            } => nodes.get(test),
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
    // Create the root namespace
    println!("Creating nodes");
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
