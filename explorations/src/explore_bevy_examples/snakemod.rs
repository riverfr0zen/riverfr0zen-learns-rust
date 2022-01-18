use bevy::prelude::*;


const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SCALE: f32 = 10.0;
const SNAKE_SPEED: i32 = 1;
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;


pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


/*
 * (From tut) SnakeHead is just an empty struct, it’s sort of like a tag that we’ll 
 * put on an entity, and then we can find that entity later by querying 
 * for entities with the SnakeHead component.
 */
 #[derive(Component)]
pub struct SnakeHead;


#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}


#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32
}


impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}


pub fn spawn_snake(mut commands: Commands) {
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
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}


pub fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<&mut Position, With<SnakeHead>>
) {
    /*
    * The main new concept here is that Query type. We can use it to iterate through 
    * all entities that have both the SnakeHead component and the Transform component. 
    * 
    * Note the use of `With` in the signature above, used to Query for entities
    * that have the SnakeHead component, but without actually needing to *retrieve*
    * it (and thus improving parallelization). See "Controlling the snake" section 
    * in tutorial.
    */
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= SNAKE_SPEED;
            // println!("x: {}", transform.translation.x);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += SNAKE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += SNAKE_SPEED;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= SNAKE_SPEED;
        }
    }
}


pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Transform)>) {
    /*
     * (From tut): The sizing logic goes like so: if something has a width of 1 
     * in a grid of 40, and the window is 400px across, then it should have a 
     * width of 10.
     */
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}


pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    /*
     * (From tut): The position translation: if an item’s x coordinate is at 5 in our
     * system, the width in our system is 10, and the window width is 200, then the 
     * coordinate should be 5 / 10 * 200 - 200 / 2. We subtract half the window width 
     * because our coordinate system starts at the bottom left, and Translation starts 
     * from the center. We then add half the size of a single tile, because we want our 
     * sprites bottom left corner to be at the bottom left of a tile, not the center.
     */

    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}