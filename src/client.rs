use base64::decode as decode64;
use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct ConsulKV {
    pub Key: String,
    pub Value: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RawKV {
    pub key: String,
    pub value: String,
}

pub type ResulT<T> = Result<T, Box<dyn Error>>;

pub fn get(addr: &String, ns: &String) -> ResulT<Vec<RawKV>> {
    let url = format!("{}/v1/kv/services/{}?recurse=true", addr, ns);
    let key_values: Vec<ConsulKV> = reqwest::get(&url)?.json()?;
    key_values.into_iter().map(|kv| decode(kv)).collect()
}

fn decode(mut raw_kv: ConsulKV) -> ResulT<RawKV> {
    let txt = raw_kv.Value.get_or_insert("".to_string());
    let v = decode64(&txt)?;
    let dec: String = String::from_utf8(v)?;
    Ok(RawKV {
        key: raw_kv.Key,
        value: dec,
    })
}

// run with cargo test -- --nocapture to print result
#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, path::Path};
    #[test]
    fn from_json() {
        let file_path = Path::new("test.json");
        let json_file = File::open(file_path).expect("could not load json file");
        let result: Vec<ConsulKV> =
            serde_json::from_reader(json_file).expect("error while reading json");
        println!("Result {:?}", result);
    }
}
