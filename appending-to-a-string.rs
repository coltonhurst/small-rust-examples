/*
    Explanation:

    In this example, we are creating a mutable String named my_string.
    We print my_string, then append an 'a' to the string with the add_an_a() function.
    Then we print my_string again.

    Extra notes:

    - "happy" is a string slice. The to_string method converts the string slice to a string.
    - println!() is a macro. In the examples below we print the value in my_string to the console.
*/

fn main() {
    let mut my_string: String = "happy".to_string();

    println!("{}", my_string);

    add_an_a(&mut my_string);

    println!("{}", my_string);
}


/*
    Explanation:

    This function accepts a mutable String reference as a parameter.
    It "borrows" the String str and appends an 'a' to the end with push_str().

    Extra notes:

    - Here, push_str() appends a string slice on the String str.
    - Because "a" here is a string literal, it's a &str (string slice reference).
*/
fn add_an_a(str: &mut String) {
    str.push_str("a");
}
