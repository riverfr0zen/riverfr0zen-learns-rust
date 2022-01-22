use bevy::prelude::*;
use rand::prelude::random;

pub const CLEAR_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
pub const FOOD_STEP: f64 = 1.0;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const SNAKE_SCALE: f32 = 10.0;
// pub const SNAKE_SPEED: f64 = 0.150;
// pub const SNAKE_STEP: f64 = 0.150;
pub const SNAKE_STEP: f64 = 0.150;
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}


#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}


impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}


/*
 * (From tut) SnakeHead is just an empty struct, it’s sort of like a tag that we’ll 
 * put on an entity, and then we can find that entity later by querying 
 * for entities with the SnakeHead component.
 */
 #[derive(Component)]
pub struct SnakeHead {
    direction: Direction
}


#[derive(Component)]
pub struct SnakeSegment;


/*
 * (From tut) The tail of the snake is somewhat complex. For each segment, we need 
 * to know where it needs to go next. The way we’re going to approach this is to 
 * put the snake segments in a Vec, and store that as a resource. (see snakeapp())
 */
#[derive(Default)]
pub struct SnakeSegments(Vec<Entity>);


/*
 * There’s a bit of a problem with our current system setup , because we’re asking 
 * for input in the same system as we’re moving our snake. So here’s the goal: we 
 * want to split up the input handling from the movement, and make sure the movement 
 * happens at a fixed timestep. 
 * 
 * To accomplish this we’re introducing the concept of system labels.
 * 
 * Any type that implements SystemLabel can be used for labeling. Here we’re defining 
 * our own enum and letting bevy derive SystemLabel for us. 
 */
#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}


#[derive(Component)]
struct Food;


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


/*
 * (From tut) Since we’re going to be spawning segments from a couple places (when you 
 * eat food and when you initialize the snake), we’ll create a helper function.
 * 
 * This should look very similar to the spawning of the SnakeHead, but instead of a 
 * SnakeHead component, it’s got a SnakeSegment component. Something new here is that 
 * we’re then getting that Entity (which is really just an id), by using the id 
 * function, and returning it so that callers can use it.
 */
fn spawn_segment(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}


/*
 * (If you are looking at the diff for "Adding a tail", you will notice that
 * the contents of the SpriteBundle has changed, removing the Transform, etc.
 *  -- this actually happened in a previous part of the tutorial ("Slapping a 
 * grid on it") -- I just forgot to update it then. But it fits here meaningfully
 * too).
 * 
 * (From tut) Now, we’ll need to modify our game setup function. Instead of just a head, 
 * it’s also going to spawn… a snake segment (shocked pikachu meme).
 * 
 * Our first segment is the head, which you can see now has a .insert(SnakeSegment) 
 * addition. (irf: This way the head part is also Queryable as a SnakeSegment
 * I guess?)
 * 
 * Our second segment comes from our spawn_segment function. Voila, we’ve got a 
 * detached little “tail”:
 */
pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
    // Remember, segments (SnakeSegments) is a TupleStruct with it's first 
    // member being a Vec type, so this is why it is being assigned to 
    // `segments.0` here.
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 2 }),
    ];
}


pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        /*
         * Notice this interesting way of assigning `dir` with an `if` statement
         */
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        /*
         * This is what prevents the snake from turning on itself
         */
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}


pub fn snake_movement(mut heads: Query<(&mut Position, &SnakeHead)>) {
    if let Some((mut head_pos, head)) = heads.iter_mut().next() {
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
    }
}


pub fn food_spawner(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        })
        .insert(Size::square(0.8));
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


