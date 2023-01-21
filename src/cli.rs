use std::io;
use std::io::stdin;
use console::{ Term, Style };
use reqwest;
use serde_json;
use serde::Deserialize;
use std::thread;
use std::time::Duration;
use parity_scale_codec::{Decode, Encode};
use sp_core::{hexdisplay::HexDisplay, OpaqueMetadata, H256};



#[derive(Deserialize)]
pub struct RPCResponses {
    // jsonrpc: String,
    result: String,
    // id: u8,
}
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub enum Call {
	// from, to, amount, fee
	Transfer([u8; 32], [u8; 32], u32, u32),
	Upgrade(Vec<u8>),
	NewAccount([u8; 32]),
	Mint([u8; 32]),
}
#[derive(Debug, Encode, Decode, PartialEq, Eq, Clone)]
pub struct BasicExtrinsic(Call);

impl BasicExtrinsic {
	fn new_unsigned(call: Call) -> Self {
		<Self as sp_runtime::traits::Extrinsic>::new(call, None).unwrap()
	}
}

impl sp_runtime::traits::Extrinsic for BasicExtrinsic {
	type Call = Call;
	type SignaturePayload = ();

	fn new(data: Self::Call, _: Option<Self::SignaturePayload>) -> Option<Self> {
		Some(Self(data))
	}
}

pub fn run(term: &Term) -> io::Result<()> {
    let cyan = Style::new().cyan();
    let ferris = Style::new().color256(214).bold();

    term.write_line(&ferris.apply_to("--- Welcome! Let's send some LOLO loot ---").to_string())?;
    thread::sleep(Duration::from_millis(900));
    // find out if you have to create an account 
    let account_question = &cyan.apply_to("Do you have an LOLO account? (y/n)").to_string();
    let account_response: String = get_string(&account_question, &term);

    if account_response == String::from("n") {
        let user_number = get_u32_input("Okay, give me a number then and I'll create an account for you:", &term) as u8;
        let new_account = BasicExtrinsic::new_unsigned(Call::NewAccount([user_number; 32]));

        println!("new account_1: {:?}", HexDisplay::from(&new_account.encode()));
        // user needs a new account created 
        // call new account extrinsic and give [u8; 32]
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
        
        // call mint extrinsic 
        // welcome! by joining, you've minted 10 lolo tokens 
        // rpc call out & create a new account 
        // but for now we'll set it manually 
    } else {
        let id: u32 = get_u32_input("What is your account id?", &term);
    }
     // do you want to send tokens or check your balance? 
    let action_response: String = get_string("Do you want to send tokens or check your balance? (S) to send (C) to check ", &term);

    if action_response == String::from("S") || action_response == String::from("s") {
        // send tokens 
        let recipient_id: u32 = get_u32_input("Who do you want to send to? (their account id)", &term);
        let amount: u32 = get_u32_input("How many LOLO loot do you want to send?", &term);
        let fee: u32 = get_u32_input("What is the fee you want to pay? (Must be > 1)", &term);
        // RPC CALL to send tokens 
            // logic for whether it was successful or not
        // show ending balance 
        // exit program 
    } else if action_response == String::from("C") || action_response == String::from("c") {
         // check balance 

        // rpc call to check balance 
        // exit program 
    } else {
        term.write_line("Sorry that's all this program does at the time!")?;
        // exit program 
    }

    Ok(())
}

fn get_u32_input(message: &str, term: &Term) -> u32 {
    term.write_line(&message);

    let mut input: String = String::new();
    stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}


fn get_string(message: &str, term: &Term) -> String {
    term.write_line(&message).unwrap();

    let mut input: String = String::new();
    stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}

    // create an account or what is your account id? 

    // do you want to send tokens or check your balance? 
        // press (s) to transfer tokens
        // press (c) to check your balance
    // s: who do you want to send to? (their address)
    // s: how much do you want to send? (amount)
    // validated! or rejected :( 
    // at the end show (c)

    // c: shows current balance & exits 
