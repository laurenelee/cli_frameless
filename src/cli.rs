use std::io;
use std::io::stdin;
use console::{ Term, Style };



pub fn run(term: &Term) -> io::Result<()> {
    let cyan = Style::new().cyan();
    let ferris = Style::new().color256(214).bold();

    term.write_line(&ferris.apply_to("--- Welcome! ---").to_string())?;

    // find out if you have to create an account 
    let account_question = &cyan.apply_to("Do you have an LOLO account? (y/n)").to_string();
    let account_response: String = get_string(&account_question, &term);

    if account_response == String::from("n") {
        // create an account for them
        // rpc call out & create a new account 
        // but for now we'll set it manually 
        let id: u32 = 4;
    } else {
        let id: u32 = get_u32_input("What is your account id?", &term);
    }
     // do you want to send tokens or check your balance? 
    let action_response: String = get_string("Do you want to send tokens or check your balance? (S) to send (C) to check ", &term);

    if action_response == String::from("S") || action_response == String::from("s") {
        // send tokens 
        let recipient_id: u32 = get_u32_input("Who do you want to send to? (their account id)", &term);
        let amount: u32 = get_u32_input("How many LOLO tokens do you want to send?", &term);
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
