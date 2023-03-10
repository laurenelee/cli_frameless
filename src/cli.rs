use std::io;
use std::io::stdin;
use console::{ Term, Style, style };
use reqwest;
use serde_json;
use serde::Deserialize;
use std::thread;
use std::time::Duration;
use parity_scale_codec::{Decode, Encode};
use sp_core::{hexdisplay::HexDisplay};
use hex;
use spinners::{Spinner, Spinners};

#[derive(Deserialize)]
pub struct RPCResponses {
    result: String,
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

fn get_u32_input(message: &str, term: &Term) -> u32 {
    term.write_line(&message).unwrap();

    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn get_string(message: &str, term: &Term) -> String {
    term.write_line(&message).unwrap();

    let mut input: String = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn create_account(client: &reqwest::blocking::Client, _term: &Term, user_number: u8) -> io::Result<()> {
    let new_account = BasicExtrinsic::new_unsigned(Call::NewAccount([user_number; 32]));

    client.post("http:/localhost:9933/")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "author_submitExtrinsic",
            "params": [HexDisplay::from(&new_account.encode()).to_string()]
        }))
        .send()
        .expect("Failed to create account");
    Ok(())
}

fn mint_loot(client: &reqwest::blocking::Client, term: &Term, user_number: u8) -> io::Result<()> {
    let mint = BasicExtrinsic::new_unsigned(Call::Mint([user_number; 32]));

    client.post("http:/localhost:9933/")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "author_submitExtrinsic",
            "params": [HexDisplay::from(&mint.encode()).to_string()]
        }))
        .send()
        .expect("Failed to mint");
    
    thread::sleep(Duration::from_millis(900));
    term.write_line("Welcome new friend! By joining, you've minted 10 ???LOLO??? LOOT")?;
    thread::sleep(Duration::from_millis(900));
    Ok(())
}

fn check_balance(client: &reqwest::blocking::Client, term: &Term, user_number: u8) -> io::Result<()> {
    // CLI magic ???
    let mut sp = Spinner::new(Spinners::Dots9, "Checking your balance...".to_string());
    thread::sleep(Duration::from_millis(3000));
    sp.stop();
    term.clear_line()?;

    let balance_res = client.post("http:/localhost:9933/")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "state_getStorage",
            "params": [HexDisplay::from(&[user_number; 32].encode()).to_string()]
        }))
        .send();
    let balance_response_body: RPCResponses = balance_res.unwrap().json().unwrap();
    let hex_rep = balance_response_body.result.split("0x").collect::<Vec<&str>>()[1];
    let converted_hex = hex::decode(hex_rep);
    match converted_hex {
        Ok(v) => {
            let balance = u32::decode(&mut &v[..]).unwrap();
            println!("Your current balance is: {:?}", style(balance).green());
        },
        Err(e) => println!("Error: {:?}", e)
    }
    Ok(())
}

fn transfer_loot(client: &reqwest::blocking::Client, term: &Term, user_number: u8) -> io::Result<()> {
    let action_response: String = get_string("Do you want to send LOOT or check your balance? ( Enter (S) to send (C) to check ) ", &term);

    match action_response.as_str() {
        "S" | "s" => {
            let recipient_id: u8 = get_u32_input("Who do you want to send to? (Their account id)", &term) as u8;
            let amount: u32 = get_u32_input("How many LOLO loot do you want to send? (Remember, new users only have 10 LOOT)", &term);
            let fee: u32 = get_u32_input("What is the fee you want to pay? (Must be > 1)", &term);

            let transfer_extrinsic = BasicExtrinsic::new_unsigned(Call::Transfer([user_number; 32], [recipient_id; 32], amount, fee));

            client.post("http:/localhost:9933/")
                .json(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "author_submitExtrinsic",
                    "params": [HexDisplay::from(&transfer_extrinsic.encode()).to_string()]
                }))
                .send()
                .expect("Transfer failed");
            
            // CLI magic ???
            let mut sp = Spinner::new(Spinners::Dots9, "Initiating your transfer...".to_string());
            thread::sleep(Duration::from_millis(3000));
            sp.stop();
            term.clear_line()?;
            term.write_line("Your transfer has been submitted to the chain!")?;

            check_balance(client, term, user_number)?;
            Ok(())
        },
        "C" | "c" => {
            check_balance(client, term, user_number)?;
            Ok(())
        },
        _ => {
            println!("Invalid input");
            Ok(())
        }
    }
}

pub fn run(term: &Term) -> io::Result<()> {
    let client = reqwest::blocking::Client::new();

    let cyan = Style::new().cyan();
    let ferris = Style::new().color256(214).bold();
    
    term.write_line(&ferris.apply_to("--- Welcome! Let's send some LOLO loot ---").to_string())?;
    thread::sleep(Duration::from_millis(900));
    term.write_line(&cyan.apply_to("Let's create an account for you!").to_string())?;
    thread::sleep(Duration::from_millis(900));
    
    let user_number = get_u32_input("Give me a number and I'll create an account for you:", &term) as u8;

    create_account(&client, &term, user_number)?;
    mint_loot(&client, &term, user_number)?;
    transfer_loot(&client, &term, user_number)?;

    thread::sleep(Duration::from_millis(900));
    term.write_line(&ferris.apply_to("That's all folks, thanks for playing!").to_string())?;

    Ok(())
}