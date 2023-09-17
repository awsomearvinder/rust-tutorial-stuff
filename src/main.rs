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

// We can write generic code now, if we wanted too.
// reads "foo takes a str, and returns some type T which implements From<&str>"
// The 'a business says that the *lifetime* of &str that From takes, is tied
// to the lifetime of *f*. That is, the memory that the T converts from lives
// as long as the memory `f` refers too.
fn from_str<'a, T: From<&'a str>>(f: &'a str) -> T {
    T::from(f)
}


fn greet_entity(Greeting(mut greeting): Greeting, entity: Entity) -> String {
    // you can pattern match on enums too! Handle the different
    // things an enum can be, and take the data out.
    match entity {
        Entity::User(u) => greeting.push_str(&u),
        Entity::Group(g) => {
            greeting.push_str(" everyone from ");
            greeting.push_str(&g);
        }
    }
    greeting
}

fn main() {
    let greeting = greet_entity(from_str("Hello "), Entity::Group(String::from("Lobotomy corporation")));
    println!("{greeting}!");
}
