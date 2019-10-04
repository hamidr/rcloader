mod printer;
mod node;
mod client;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
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
        "json"  => printer::to_json, 
        "hocon" => printer::to_hocon,
        _ => panic!("what?!")
    };
    let raw_kvs = client::get(url, ns)?;
    let nodes = node::Node::new(raw_kvs)?;
    let _strr = prtr(&nodes);
    Ok(())
}


