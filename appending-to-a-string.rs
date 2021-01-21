fn main() {
    let mut my_string: String = "happy".to_string();

    println!("{}", my_string);

    add_an_a(&mut my_string);

    println!("{}", my_string);
}

fn add_an_a(str: &mut String) {
    str.push_str("a");
}

