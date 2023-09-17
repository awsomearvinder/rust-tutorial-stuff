// structs hold data, they're sort of like classes in Java.
// this is known as a tuple struct, so it doesn't have it's fields named.
// you can have structs with it's properties being named too - but we'll
// avoid that.
struct Greeting(String);

fn greet_john(mut greeting: Greeting) -> String {
    //Take a greeting, and return a string.
    greeting.0.push_str(" John");
    greeting.0
}

fn main() {
    let greeting = greet_john(Greeting(String::from("Hello")));
    println!("{greeting}!");
}
