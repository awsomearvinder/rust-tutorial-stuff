fn say_hello(u: &str) {
    // we now take in a reference to a string, or a "string slice"
    // rather then a string itself. This is a pointer, and by
    // using this, we don't have to copy data around (or move it).
    println!("Hello, {u}!");
}

fn main() {
    // string literals are by default a reference. This is because
    // instead of *allocating* memory, they're just pointers to the
    // relevant section in the binary in memory. (It's the same
    // reason string literals are a char* or `string_view` in C++).
    say_hello("john");
}
