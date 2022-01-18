use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SCALE: f32 = 10.0;


/* 
 * Tutorial at https://mbuffett.com/posts/bevy-snake-tutorial/
 */

pub fn snakeapp() {
    App::new()
       .add_startup_system(setup_camera)
       .add_startup_system(spawn_snake)
       .add_plugins(DefaultPlugins)
       .run();   
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


/*
 * (From tut) SnakeHead is just an empty struct, it’s sort of like a tag that we’ll 
 * put on an entity, and then we can find that entity later by querying 
 * for entities with the SnakeHead component.
 */
 #[derive(Component)]
struct SnakeHead;

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            transform: Transform {
                scale: Vec3::new(SNAKE_SCALE, SNAKE_SCALE, SNAKE_SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead);
}