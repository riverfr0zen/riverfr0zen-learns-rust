/**
 * Following Bevy Book "Getting Started" tutorial at:
 * https://bevyengine.org/learn/book/getting-started/apps/
 * 
 */

use bevy::prelude::*;

fn main() {
    App::build()
        // Among other things, DefaultPlugins add a WindowPlugin and
        // and a WinitPlugin, and also adds an event loop
        .add_plugins(DefaultPlugins)
        // Startup systems only run at the beginning
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}


fn hello_world() {
    println!("hello world!");
}


struct Person;


struct Name(String);


fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("David Lister".to_string()));
    commands.spawn().insert(Person).insert(Name("Arnold Rimmer".to_string()));
    commands.spawn().insert(Person).insert(Name("Cat".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    // From tutorial: You can interpret the Query above as: 
    // "iterate over every Name component for entities that 
    // also have a Person component"
    for name in query.iter() {
        if name.0 == "Arnold Rimmer" {
            println!("It's Arnold Arnold Arnold Rimmer!")
        } else {
            println!("Alright {}!", name.0)
        }
    }
}

