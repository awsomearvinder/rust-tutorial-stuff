// Enums are a way to say "or". An entity is a User *or* a Group.
// unlike other languages, enums in rust can hold data.
enum Entity {
    User(String),
    Group(String),
}

struct Greeting(String);

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
    let greeting = greet_entity(Greeting(String::from("Hello")), Entity::Group(String::from("Lobotomy corporation")));
    println!("{greeting}!");
}
