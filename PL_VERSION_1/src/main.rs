use std::io;

fn main() {
    let x = show_ui();
    println!("{}", x);
}

fn show_ui() -> String {
    println!("How can I help you?");
    println!("  quit    - quit program");
    println!("  add     - add new item to DB");
    println!("  remove  - remove selected item from DB (by id)");
    println!("  list    - list all items in database");
    println!("  filter  - setup filtering");
    println!("  search  - search for item");
    println!("");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input;
        },
        Err(_) => {
            return String::from("Error.");
        }
    }
}