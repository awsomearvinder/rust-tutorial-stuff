// Enums are a way to say "or". An entity is a User *or* a Group.
// unlike other languages, enums in rust can hold data.
enum Entity {
    User(String),
    Group(String),
}

struct Greeting(String);

// rust has "traits", which are like java interfaces.
// (Or more precisely, like Haskell typeclasses)
// They let you implement shared behavior over multiple types.
// In this case, we implement `From<String>` and `From<&str>`
// to say that a Greeting can be created from either.
impl From<String> for Greeting {
    fn from(value: String) -> Self {
        Greeting(value)
    }
}

impl From<&str> for Greeting {
    fn from(value: &str) -> Self {
        Greeting(value.to_owned())
    }
}

fn build_greeter(Greeting(s): Greeting) -> impl Fn(&Entity) -> String {
    // we take a greeter, and return a *closure* (or unnamed function).
    // other languages call this a lambda. This lambda takes control of the
    // memory that it "captures" (or refers too), hence the `move`. This way
    // we don't have to worry about the closure referring to some outside memory,
    // and having to tie the lifetime of the closure to something.
    move |entity| {
        let mut s = s.clone();
        match entity {
            Entity::User(u) => s.push_str(&u),
            Entity::Group(g) => {
                s.push_str(" everyone from ");
                s.push_str(&g);
            }
        }
        s
    }
}

fn main() {
    let greeter = build_greeter(Greeting::from("Hello "));
    let greeting = greeter(&Entity::Group(String::from("Lobotomy corporation")));
    println!("{greeting}!");
}
