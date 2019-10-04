use crate::node::Node;
use serde_json::Value as JsonValue;

fn form_json(nodes: &Vec<Node> ) -> JsonValue {

}
pub fn to_json(_nodes: &Vec<Node>) -> String {
    println!("{:?}", _nodes); "".to_string()
}

pub fn to_hocon(_nodes: &Vec<Node>) -> String {
    println!("{:?}", _nodes); "".to_string()
}

