use bevy::prelude::*;
use bevy::core::FixedTimestep;

pub mod gamelib;

/* 
 * Tutorial at https://mbuffett.com/posts/bevy-snake-tutorial/
 */

pub fn snakeapp() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: gamelib::WINDOW_WIDTH,
            height: gamelib::WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(gamelib::CLEAR_COLOR))
        .insert_resource(gamelib::SnakeSegments::default())
        .insert_resource(gamelib::LastTailPosition::default())
        .add_event::<gamelib::GrowthEvent>()
        .add_event::<gamelib::GameOverEvent>()
        .add_startup_system(gamelib::setup_camera)
        .add_startup_system(gamelib::spawn_snake)
        /*
         * There’s a few new things here. .label(x) tags a system 
         * (or system set) with a label. On its own this does nothing, 
         * but the gain is that you can then use .before(x) or .after(x) on 
         * other systems, to specify order. So here we’re tagging the 
         * snake_movement system with .label(SnakeMovement::Movement), and 
         * for the input system we’re adding .before(SnakeMovement::Movement), 
         * to ensure that on a given frame, we get the user input before we 
         * move the snake.
         */
        .add_system(
            gamelib::snake_movement_input
                .label(gamelib::SnakeMovement::Input)
                .before(gamelib::SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(gamelib::SNAKE_STEP))
                .with_system(gamelib::snake_movement.label(
                    gamelib::SnakeMovement::Movement
                ))
                /*
                 * We want the eating check to happen after movement, so we add 
                 * .after(SnakeMovement::Movement) so bevy makes it happen in 
                 * the right order.
                 */
                .with_system(
                    gamelib::snake_eating
                        .label(gamelib::SnakeMovement::Eating)
                        .after(gamelib::SnakeMovement::Movement),
                )
                .with_system(
                    gamelib::snake_growth
                        .label(gamelib::SnakeMovement::Growth)
                        .after(gamelib::SnakeMovement::Eating),
                )
        )
        // Add game_over system, and make sure it runs after Movement:
        .add_system(gamelib::game_over.after(gamelib::SnakeMovement::Movement))
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
                .with_run_criteria(FixedTimestep::step(gamelib::FOOD_STEP))
                .with_system(gamelib::food_spawner),
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
            .with_system(gamelib::position_translation)
            .with_system(gamelib::size_scaling),
        )
        .add_plugins(DefaultPlugins)
        .run();   
}