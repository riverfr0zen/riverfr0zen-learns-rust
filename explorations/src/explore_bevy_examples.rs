use bevy::prelude::*;
use bevy::core::FixedTimestep;

pub mod snakemod;


/* 
 * Tutorial at https://mbuffett.com/posts/bevy-snake-tutorial/
 */

 pub fn snakeapp() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(snakemod::CLEAR_COLOR))
        .add_startup_system(snakemod::setup_camera)
        .add_startup_system(snakemod::spawn_snake)
        .add_system(snakemod::snake_movement)
        /*
         * We don’t want this going off constantly. We want to spawn food 
         * every second, not every frame. Since this is a common need in 
         * game development, to want something to happen at a fixed 
         * timestep, bevy provides the ultra-convenient FixedTimestep run 
         * criteria.
         * 
         * Somewhat self-explanatory, we’re adding a new set of systems 
         * (although there’s only one), which will run at a fixed timestep, 
         * in this case every 1 second.
         */
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(snakemod::FOOD_STEP))
                .with_system(snakemod::food_spawner),
        )
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