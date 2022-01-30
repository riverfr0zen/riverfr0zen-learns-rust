use bevy::prelude::*;
use rand::prelude::random;

pub const WINDOW_WIDTH: f32 = 1600.0;
pub const WINDOW_HEIGHT: f32 = 1600.0;
pub const CLEAR_COLOR: Color = Color::rgb(0.04, 0.04, 0.04);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);
const FOOD_SIZE: f32 = 0.8;
pub const FOOD_STEP: f64 = 1.0;
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
// Obviously if the snake head (or any component) is > 1.0, then it's 
// position can spill over grid blocks (e.g. the snake can be moved 
// onto intersections of grid blocks)
const SNAKE_HEAD_SIZE: f32 = 1.0;
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
const SNAKE_SEGMENT_SIZE: f32 = 0.8;
pub const SNAKE_STEP: f64 = 0.05;
const ARENA_WIDTH: u32 = 50;
const ARENA_HEIGHT: u32 = 50;

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
pub struct Food;


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

pub struct GrowthEvent;


/*
 * One thing we need to think about for growing is that we need to know where 
 * the last segment was before moving, because that’s where the new segment 
 * goes. Let’s create that and add it as a new resource:
 */
#[derive(Default)]
pub struct LastTailPosition(Option<Position>);


pub struct GameOverEvent;


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
        .insert(Size::square(SNAKE_SEGMENT_SIZE))
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
            .insert(Size::square(SNAKE_HEAD_SIZE))
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


/*
 * (From tut) We’ll need to change how we get the position of the head, because we 
 * can’t have two queries that are competing for Position components, and we’ll 
 * need to have access to our SnakeSegments resource. (irf: Should look up what
 * these resource competition means. Is the Query system really so easily
 * locked up? Why can we have one Query against Entity, SnakeHead and Position, 
 * and another against just Position? ... On further observation I think he just
 * means that it will lock things up unnecessarily, since later on there is a
 * system that does multiple queries against Position (see `snake_eating`))
 * 
 * (From tut) There’s a lot going on here. We’re getting the Entity of the snake 
 * head this time, instead of getting its position from a Query. Then we use 
 * positions.get_mut(head_entity).unwrap(), to get the Position of the head. The 
 * segment positions are retrieved in a similar manner, by just iterating over the 
 * segments we have in the SnakeSegments resource, and getting the Position for each 
 * one, from the positions query.
 * 
 * After we change the head position, we just need to set the position of each 
 * segment to the position of the segment in front of it. The first tail segment 
 * gets set to the head position, second tail segment gets set to the first tail 
 * segment position, etc. There’s some fun iterator magic in there, but it’s not 
 * bevy-specific so I’m not going to spend too much time on it. (irf: haha dammit)
 */
pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut last_tail_position: ResMut<LastTailPosition>,
    /*
     * (From tut) In our snake_movement system, we want access to the game over 
     * event, so we can send events:
     */
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
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
        /* 
         * Alright, so we’ve got our snake_movement system sending game over events.
         */
        if head_pos.x < 0
        || head_pos.y < 0
        || head_pos.x as u32 >= ARENA_WIDTH
        || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer.send(GameOverEvent);
        }        
        /*
         * For the final touch to round out our snake game, let’s add the bit of 
         * logic we need to send a game over event when we hit our own tail, inside 
         * the snake_movement system, right after the check for running out of 
         * bounds:
         */
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }
        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}


/*
 * (From tut) Just iterating through all food positions and seeing if they share 
 * a position with the head of the snake. If they do, we remove them using the 
 * handy despawn function, then trigger a GrowthEvent.
 * 
 * Using events is a new concept. You can send and recieve events between systems. 
 * They can be arbitrary structs so you can include any data you want in your 
 * events. For example, you may have one system that sends jump events, then a 
 * separate system that processes them. In our case, we’ll have a system that sends 
 * growth events, and a growth system to process them.
 */
pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}


pub fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments
            .0
            .push(spawn_segment(commands, last_tail_position.0.unwrap()));
    }
}


/*
 * (From tut) Let’s create a new system that listens for the game over events.
 * 
 * The cool thing here is that we can use the spawn_snake function directly. 
 * It’s now being used as both a system and a helper function.
 */
pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segments_res);
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
        .insert(Size::square(FOOD_SIZE));
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


