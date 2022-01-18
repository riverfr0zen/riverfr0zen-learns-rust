use bevy::prelude::*;

pub mod snakemod;


/* 
 * Tutorial at https://mbuffett.com/posts/bevy-snake-tutorial/
 */

 pub fn snakeapp() {
    App::new()
       .add_startup_system(snakemod::setup_camera)
       .add_startup_system(snakemod::spawn_snake)
       .add_system(snakemod::snake_movement)
       .add_plugins(DefaultPlugins)
       .run();   
}