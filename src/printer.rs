use crate::node::Node;
use serde_json::Value;
use serde::ser::{Serialize, SerializeMap, Serializer};
use json_patch::merge;

struct WrapDire<'a, T> {
    name: &'a str,
    value: &'a T
}

impl<'a, T> Serialize for WrapDire<'a, T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer, T: Serialize
    {
        let mut ser = serializer.serialize_map(None)?;
        ser.serialize_key(self.name)?;
        ser.serialize_value(self.value)?;
        ser.end()
    }
}

// struct WrapKV<'a> {
//     name: &'a str,
//     value: &'a str
// }

// impl<'a> Serialize for WrapKV<'a> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut ser = serializer.serialize_map(None)?;
//         ser.serialize_key(self.name)?;
//         ser.serialize_value(self.value)?;
//         ser.end()
//     }
// }

fn merge_jsons(nodes: &Vec<Value>) -> Value {
    let value = nodes.iter().fold(serde_json::Value::Null, |acc, n| {
        let mut cacc = acc.clone();
        merge(&mut cacc, n);
        cacc
    });
    value
}

fn make_json(node: &Node) -> Result<Value, serde_json::error::Error> {
    match node {
        Node::KeyValue{key, value} => {
            let wrap = WrapDire {name: key, value: value};
            serde_json::to_value(wrap)
        },
        Node::Directory{key, nodes} => {
            let jnodes: Result<Vec<_>, _>= nodes.iter()
                .filter(|(k, _)| !k.key().is_empty()) //clean empty leafs!
                .map(|(_, n)| make_json(n)).collect();
            let parts = jnodes?;
            let value = merge_jsons(&parts);
            serde_json::to_value(WrapDire {name: &key, value: &value})
        }
    }
}

pub fn to_json(_node: &Node) -> String {
    let strr =  make_json(_node).and_then(|n| serde_json::to_string_pretty(&n));
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

pub fn to_hocon(_nodes: &Node) -> String {
    println!("{:?}", _nodes); "".to_string()
}

