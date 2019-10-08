use crate::client::{RawKV, ResulT};
use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Location {
    ns: Vec<String>
}

impl Location {
    fn path(&self) -> String {
        self.ns.join("/")
    }

    fn name(&self) -> String {
        self.ns.last().get_or_insert(&String::from(".")).clone()
    }

    fn base(&self) -> Location {
        match self.ns.first() {
            None => Location { ns: vec!() },
            Some(head)  => Location { ns: vec!(head.clone()) }
        }
    }

    fn drop_base_mut(&mut self) -> &Location {
        self.ns.remove(0); self
    }
}

impl PartialEq for Location {
    fn eq(&self, other: &Location) -> bool {
        self.ns == other.ns
    }
}

#[derive(Clone, Debug)]
pub enum Node {
    KeyValue { path: Location, name: String, value: String },
    Directory { path: Location, nodes: Vec<Node> }
}

impl Node {
    pub fn new(r1: Vec<RawKV>) -> ResulT<Vec<Node>> {
        let raws = Node::preprocess(r1)?;
        Ok(Node::process(raws))
    }

    fn preprocess(raw_kvs: Vec<RawKV>) -> ResulT<Vec<Node>> {
        let nodes = raw_kvs.into_iter().map(|rkv| {
            let path: Vec<String> = rkv.key.split("/").map(|s| s.to_string()).collect();
            let name = path.last().ok_or("Can't construct a node")?;
            let loc = Location { ns: path.clone() };
            if !rkv.key.ends_with("/") && !rkv.value.is_empty() {
                let n = Node::KeyValue { path: loc, name: name.clone(), value: rkv.value.clone() };
                Ok(n)
            } else {
                let dir = Node::Directory { path: loc, nodes: vec!() };
                Ok(dir)
            }
        });
        nodes.collect()
    }

    fn process(mut nodes: Vec<Node>) -> Vec<Node> {
        let grouped = nodes.iter_mut().group_by(|n| n.path().base());
        grouped.into_iter().map (|(key, group)| {
                let (leafs, dirs): (Vec<&Node>, Vec<&Node>) = 
                    group.into_iter().map(|n| n.drop_base_mut()).partition(|n| n.path().ns.is_empty());

                let lfs: Vec<&Node> = leafs.into_iter().filter(|n| match n {
                    Node::Directory{nodes, ..} => !nodes.is_empty(),
                    _ => true
                }).collect();
                let mut n1: Vec<Node> = lfs.into_iter().map(move |x| x.clone()).collect();
                let nodes = if dirs.is_empty() {
                        n1
                    } else {
                        let dir2: Vec<Node> = dirs.into_iter().map(move |x| x.clone()).collect();
                        let mut s = Node::process(dir2);
                        n1.append(&mut s);
                        n1
                    };
                Node::Directory { path: key, nodes: nodes }
            }).collect::<Vec<Node>>()
        }   

        fn drop_base_mut(&mut self) -> &Node {
            match self {
                Node::KeyValue {path, ..}  => {
                    path.drop_base_mut(); self
                },
                Node::Directory {path, ..}=> {
                    path.drop_base_mut(); self
                }
            }
        }

    fn path(&self) -> &Location {
        match self {
            Node::KeyValue {path, ..}  => path,
            Node::Directory {path, ..} => path
        }
    }
}

// pub fn sample() -> Node {
//     let path = Location{ns: vec!("t1".to_string(), "t2".to_string())};
//     let a = Node::KeyValue { path: path.clone(), name: "hello".to_string(), value: "bye".to_string() };
//     let dr: Node = Node::Directory {path: path.clone(), nodes: vec!(a.clone(), a.clone(), a.clone())};
//     dr
// }