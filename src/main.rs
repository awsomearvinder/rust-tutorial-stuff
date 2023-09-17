// structs hold data, they're sort of like classes in Java.
// this is known as a tuple struct, so it doesn't have it's fields named.
// you can have structs with it's properties being named too - but we'll
// avoid that.
struct Greeting(String);

// When we're declaring a variable *anywhere*, we can use what we call
// "pattern matching", to destructure the value. What is used to construct
// it on the right side, is used to destruct it on the left side.
// Here, greeting is just the inner string the `Greeting` type holds.
fn greet_john(Greeting(mut greeting): Greeting) -> String {
    greeting.push_str(" John");
    greeting
}

fn main() {
    let greeting = greet_john(Greeting(String::from("Hello")));
    println!("{greeting}!");
}
