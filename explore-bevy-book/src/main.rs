/**
 * Following Bevy Book "Getting Started" tutorial at:
 * https://bevyengine.org/learn/book/getting-started/apps/
 * 
 */

use bevy::prelude::*;

fn main() {
    App::new()
        // Among other things, DefaultPlugins add a WindowPlugin and
        // and a WinitPlugin, and also adds an event loop
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

struct HelloTimer(Timer);

fn hello_world(time: Res<Time>, mut timer: ResMut<HelloTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello world!");
    }
}


#[derive(Component)]
struct Person;


#[derive(Component)]
struct Name(String);


fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("David Lister".to_string()));
    commands.spawn().insert(Person).insert(Name("Arnold Rimmer".to_string()));
    commands.spawn().insert(Person).insert(Name("Cat".to_string()));
}

struct GreetTimer(Timer);


fn greet_people(
    time: Res<Time>, 
    mut timer: ResMut<GreetTimer>, 
    query: Query<&Name, With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            // From tutorial: You can interpret the Query above as: 
            // "iterate over every Name component for entities that 
            // also have a Person component"
            if name.0 == "Arnold Rimmer" {
                println!("It's Arnold Arnold Arnold Rimmer!")
            } else {
                println!("Alright {}!", name.0)
            }
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HelloTimer(Timer::from_seconds(1.5, true)))
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            // Startup systems only run at the beginning
            .add_startup_system(add_people.system())
            .add_system(hello_world.system())
            .add_system(greet_people.system());
    }
}