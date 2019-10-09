use crate::node::Node;
use crate::client::ResulT;
use std::error::Error;
use serde_json::Value;
use serde::ser::{Serialize, SerializeMap, Serializer};
use json_patch::merge;
use hocon::HoconLoader;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

struct WrapDire<'a, T> {
    name: &'a str,
    value: &'a T
}

impl<'a, T> Serialize for WrapDire<'a, T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut ser = serializer.serialize_map(None)?;
        ser.serialize_key(self.name)?;
        ser.serialize_value(self.value)?;
        ser.end()
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

pub trait Printer {
    fn to_string(&self, node: &Node) -> ResulT<String>;
}

pub struct ToJson;
impl Printer for ToJson {
    fn to_string(&self, node: &Node) -> ResulT<String> {
        make_json(node).and_then(|n| serde_json::to_string_pretty(&n)).map_err(|e| e.into())
    }
}

pub struct ToHocon;
impl Printer for ToHocon { 
    fn to_string(&self, node: &Node) -> ResulT<String> {
        let s = ToJson.to_string(node)?;
        let ss = s.as_str();
        let loader: ResulT<_> = HoconLoader::new()
            .load_str(ss).and_then(|l| l.hocon())
            .map_err(|e| e.into());
        loader?.as_string().ok_or(Box::new(MyError { details: "Can't convert the stupid json to string".to_string() }))
    }
}

