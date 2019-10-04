use reqwest;
use serde::Deserialize;
use base64::decode as decode64;
use std::error::Error;

#[derive(Deserialize)]
pub struct ConsulKV {
    pub Key: String,
    pub Value: Option<String>
}

pub struct RawKV {
    pub key: String,
    pub value: String
}

pub type ResulT<T> = Result<T, Box<dyn Error>>;

pub fn get(addr: &String, ns: &String) -> ResulT<Vec<RawKV>> {
    let url = format!("{}/v1/kv/services/{}?recurse=true", addr, ns);
    let f = reqwest::get(&url)?.text()?;
    println!("{:?}", f);
    let key_values: Vec<ConsulKV> =  reqwest::get(&url)?.json()?;
    key_values.into_iter().map(|kv| { decode(kv) }).collect()
}

fn decode(mut raw_kv: ConsulKV) -> ResulT<RawKV> {
    let txt = raw_kv.Value.get_or_insert("".to_string());
    let v = decode64(&txt)?;
    let dec: String = String::from_utf8(v)?;
    Ok(RawKV { key: raw_kv.Key, value: dec })
}