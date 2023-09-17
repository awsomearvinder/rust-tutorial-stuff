fn greet_john(mut greeting: String) -> String {
    // if we take in an *owned* copy of the string
    // (see how there's no reference? We're giving control
    // of the memory to the function), we can modify the memory
    // and return the same copy back.
    greeting.push_str(" John");
    greeting
}

fn main() {
    // Strings are owned types. They allocate memory from the OS.
    // By passing control of the memory or a *mutable*
    // reference, we can modify it without having to allocate another
    // one.
    let greeting = greet_john(String::from("Hello"));
    println!("{greeting}!");
}
