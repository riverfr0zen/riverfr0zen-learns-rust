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
        /*
         * These are a bit of a special case; they should run in the PostUpdate 
         * stage. They need to be in a separate stage because commands only get 
         * executed after each stage, so if we add a new entity in our Update 
         * stage, we need that stage to finish before position_translation and 
         * size_scaling will be able to see that entity.
         */
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
            .with_system(snakemod::position_translation)
            .with_system(snakemod::size_scaling),
         )
        .add_plugins(DefaultPlugins)
        .run();   
}