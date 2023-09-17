fn say_hello(u: String) {
    // This returns the `()` type implicitly.
    // unlike java, all functions return a value here.
    // The difference is, `()`, or the empty tuple is what
    // functions return by default.
    println!("Hello, {u}!");
}

fn main() {
    say_hello(String::from("john"));
}
