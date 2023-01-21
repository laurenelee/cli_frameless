// use console::Term;
use reqwest;
use serde_json;
use serde::Deserialize;
// use serde_json::Result;


// mod cli;
#[derive(Deserialize)]
pub struct RPCResponses {
    jsonrpc: String,
    result: String,
    id: u8,
}

fn main() {


    let client = reqwest::blocking::Client::new();
    let res = client.post("http:/localhost:9933/")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "author_submitExtrinsic",
            "params": ["020c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c"]
         }))
        .send();
        let response_body: RPCResponses = res.unwrap().json().unwrap();

    println!("{:?}", response_body.result);


    // let term = Term::stdout();

    // match cli::run(&term) {
    //     Err(e) => println!("{:?}", e),
    //     _ => (),
    // }
}
