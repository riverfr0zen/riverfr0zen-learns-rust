use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SCALE: f32 = 10.0;
const SNAKE_SPEED: f32 = 2.0;

/* 
 * Tutorial at https://mbuffett.com/posts/bevy-snake-tutorial/
 */

pub fn snakeapp() {
    App::new()
       .add_startup_system(setup_camera)
       .add_startup_system(spawn_snake)
       .add_system(snake_movement)
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
    /*
     * (From tut): This will spawn a new entity, which will have all the components 
     * from a SpriteBundle, and also the SnakeHead component. 
     */
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..Default::default()
            },
            transform: Transform {
                // scale: Vec3::new(SNAKE_SCALE, SNAKE_SCALE, SNAKE_SCALE),
                scale: Vec3::new(SNAKE_SCALE, SNAKE_SCALE, SNAKE_SCALE),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeHead);
}

 fn snake_movement(mut head_positions: Query<(&SnakeHead, &mut Transform)>) {
    /*
    * The main new concept here is that Query type. We can use it to iterate through 
    * all entities that have both the SnakeHead component and the Transform component. 
    */
    for (_head, mut transform) in head_positions.iter_mut() {
        transform.translation.y += SNAKE_SPEED;
    }
}
