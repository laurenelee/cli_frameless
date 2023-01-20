use console::Term;
mod cli;
fn main() {
    let term = Term::stdout();

    match cli::run(&term) {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}
