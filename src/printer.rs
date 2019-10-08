use crate::node::Node;
use serde_json::Value;
use serde::ser::{Serialize, SerializeMap, Serializer, SerializeSeq};
use json_patch::merge;
use crate::client::ResulT;

struct WrapKV<'a> {
    name: &'a String,
    value: &'a String
}

impl<'a> Serialize for WrapKV<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_map(None)?;
        ser.serialize_key(self.name)?;
        ser.serialize_value(self.value)?;
        ser.end()
    }
}

fn form_json(node: &Node) -> Result<Value, serde_json::error::Error> {
    match node {
        Node::KeyValue{name, value, ..} => {
            let wrap = WrapKV {name: name, value: value};
            serde_json::to_value(wrap)
        },
        Node::Directory{nodes, ..} => {
            let jnodes: Result<Vec<_>, _>= nodes.iter().map(|n| form_json(n)).collect();
            let parts = jnodes?;
            let value = merge_jsons(&parts);
            Ok(value)
        }
    }
}

fn merge_jsons(nodes: &Vec<Value>) -> Value {
    let value = nodes.iter().fold(serde_json::Value::Null, |acc, n| {
        let mut cacc = acc.clone();
        merge(&mut cacc, n);
        cacc
    });
    value
}

fn merge_nodes(nodes: &Vec<Node>) -> Result<Value, serde_json::error::Error> {
    let jsons = nodes.iter().map(|n| form_json(n)).collect::<Result<Vec<_>, _>>()?;
    let merged = merge_jsons(&jsons);
    Ok(merged)
}

pub fn to_json(_nodes: &Vec<Node>) -> String {
    println!("{:?}", _nodes);
    let strr =  merge_nodes(_nodes).and_then(|n| serde_json::to_string_pretty(&n));
    match strr {
        Ok(x) => {
            println!("{}", x);
            x
        },
        Err(_) => {
            unimplemented!("fsd")
        }
    }
}

pub fn to_hocon(_nodes: &Vec<Node>) -> String {
    println!("{:?}", _nodes); "".to_string()
}

