mod client;
mod node;
mod printer;
use node::into_tree;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;
    use client::{ConsulKV, RawKV};
    use node::{into_tree, Key};
    use serde_json;
    use std::{fs::File, path::Path};

    fn raw(kv: ConsulKV) -> RawKV {
        RawKV {
            key: kv.Key.to_string(),
            value: kv.Value.unwrap().to_string(),
        }
    }
    fn from_json() -> Vec<RawKV> {
        let file_path = Path::new("test.json");
        let json_file = File::open(file_path).expect("could not load json file");
        let result: Vec<ConsulKV> =
            serde_json::from_reader(json_file).expect("error while reading json");
        let result = result.into_iter().map(|kv| raw(kv)).collect();
        result
    }

    #[test]
    fn builds_tree() {
        let tree = from_json();
        let tree = into_tree(tree);
        // println!("Tree: {:?}", tree);

        let test_branch = tree.get(&Key::Directory("services".to_string()));
        println!("test branch: {:?}", test_branch.unwrap());
        assert!(test_branch.is_some(), true);

        // let test_branch_b = tree.get(&Key::Directory("b".to_string()));
        // println!("test branch b: {:?}", test_branch_b.unwrap());
        // assert!(test_branch_b.is_some(), true);
        // let test_kv = test_branch.get(&Key::KeyValue("x".to_string()));
        // assert_eq!(
        //     test_kv.unwrap(),
        //     &Node::KeyValue {
        //         key: "x".to_string(),
        //         value: "eA==".to_string()
        //     }
        // );
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 5 {
        println!(
"rcloader [url=http://127.0.0.1:8500] [namespace=capi-gateway] [file=out.txt] [json|hocon] \
E.g.: rcloader http://127.0.0.1:8500 capi-gateway foo.txt hocon \
cargo run http://127.0.0.1:8500 test foo.txt json
");
        ::std::process::exit(1);
    }
    let url = &args[1];
    let ns = &args[2];
    let _output = &args[3];
    let prtr = match args[4].to_lowercase().as_ref() {
        "json" => printer::to_json,
        "hocon" => printer::to_hocon,
        _ => panic!("what?!"),
    };
    let raw_kvs = client::get(url, ns)?;
    let nodes = into_tree(raw_kvs);
    let _strr = prtr(&nodes);
    Ok(())
}
