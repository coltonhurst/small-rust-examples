/*
    Explanation:

    In this example, we ask the user for their name,
    get their name with the get_user_input() function,
    and tell them hello!

    Extra notes:

    - We 'use' 'std::io::{self, Write}' because we want to just use
    the shortened io, not std::io, and we also want to access the Write trait
    in std::io.
    - We need to do 'io::stdout().flush().unwrap()' so the print!() macro
    actually prints the &str before we expect user input. The reason
    for this is mentioned in the Rust std docs: https://doc.rust-lang.org/std/macro.print.html
    - We set the variable name to whatever the get_user_input() function returns.
    - Then we print the contents of name!
*/

use std::io::{self, Write};

fn main() {
    print!("Hello! Please enter your name: ");
    io::stdout().flush().unwrap();

    let name: String = get_user_input();


    println!("Hello {}!", name);
}

/*
    Explanation:

    In this function, we return user input (after truncating the newline).

    Extra Notes:

    - We create a mutable String called name
    - We read the line into the String variable name. The read_line borrows the
    memory name points to so it can append the user input.
    - After the data is read in to name, we shorten the end of the string by 1.
    We do this because the last value (we assume) is a new line. We don't want
    this because names usually don't have new lines in them!
    - Then we return name
*/
fn get_user_input() -> String {
    let mut name: String = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    name.truncate(name.len() - 1);

    name
}