use bevy::prelude::*;
use bevy::window::WindowCreated;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;
use rand::prelude::thread_rng;



// pub const WINDOW_WIDTH: f32 = 3200.0;
// pub const WINDOW_HEIGHT: f32 = 2400.0;
pub const WINDOW_WIDTH: f32 = 1600.0;
pub const WINDOW_HEIGHT: f32 = 1600.0;
pub const SHIFTY_CIRCLE_STEP: f64 = 0.01;
pub const SHIFTY_CHANGE_STEP: f64 = 0.5;
pub const CLEAR_COLOR: Color = Color::INDIGO;
const SHIFTY_CIRCLE_RADIUS: f32 = 50.0;
const SHIFTY_CIRCLE_STROKE: f32 = 5.0;
const SHIFTY_CIRCLE_MIN_SPEED: f32 = 0.1;
const SHIFTY_CIRCLE_MAX_SPEED: f32 = 50.0;
// const SHIFTY_CIRCLE_FILL_COLOR: Color = Color::rgba(0.0, 1.0, 0.0, 0.2);
const SHIFTY_CIRCLE_FILL_COLOR: Color = Color::RED;
const SHIFTY_CIRCLE_STROKE_COLOR: Color = Color::ORANGE;
const DEST_LOW_X: f32 = -WINDOW_WIDTH/2.0+SHIFTY_CIRCLE_RADIUS;
const DEST_HIGH_X: f32 = WINDOW_WIDTH/2.0-SHIFTY_CIRCLE_RADIUS;
const DEST_LOW_Y: f32 = -WINDOW_HEIGHT/2.0+SHIFTY_CIRCLE_RADIUS;
const DEST_HIGH_Y: f32 = WINDOW_HEIGHT/2.0-SHIFTY_CIRCLE_RADIUS;


#[derive(Component)]
pub struct ShiftyCircle;


// #[derive(Component, Clone, Copy, PartialEq, Eq)]
#[derive(Component)]
pub struct Destination {
    x: f32,
    y: f32,
    speed: f32,
}


pub fn setup_shifty_circle(mut commands: Commands) {
    // let mut rng = thread_rng();
    let mycircle = shapes::Circle {
        radius: SHIFTY_CIRCLE_RADIUS,
        // I thought this could be a way to start the circle in a random
        // position, but found out what this does is set the center of
        // the circle **in the world**. Any future translations would
        // be based off this center. Of, course that isn't what
        // I wanted. (What I wanted is in fact actually achieved in the 
        // first cycle of the`change_circle_destination` system).
        //
        // center: Vec2::new(
        //     rng.gen_range(-WINDOW_WIDTH/2.0..WINDOW_WIDTH/2.0), 
        //     rng.gen_range(-WINDOW_HEIGHT/2.0..WINDOW_HEIGHT/2.0)),
        ..Default::default()

    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &mycircle,
        DrawMode::Outlined {
            fill_mode: FillMode::color(SHIFTY_CIRCLE_FILL_COLOR),
            outline_mode: StrokeMode::new(SHIFTY_CIRCLE_STROKE_COLOR, SHIFTY_CIRCLE_STROKE),
        },
        Transform::default(),
    ))
    .insert(ShiftyCircle)
    .insert(Destination { x: 0.0, y: 0.0, speed: SHIFTY_CIRCLE_MIN_SPEED });
}


// Based on https://github.com/bevyengine/bevy/issues/175
// 
// Call the handle_browser_resize system once at startup (if window is created)
// to cover for the short period before handle_browser_resize kicks in
// (since that system will likely be set to a FixedTimeStep)
pub fn setup_browser_size(
    windows: ResMut<Windows>, 
    mut window_created_reader: EventReader<WindowCreated>
) {
    if window_created_reader.iter().next().is_some() {
        handle_browser_resize(windows);
    }
}


// Based on this Discord conversation: https://i.imgur.com/osfA8PH.png AND
// https://github.com/mrk-its/bevy-robbo/blob/master/src/main.rs
pub fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );
    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}



pub fn translate_circle(mut q: Query<(&mut Transform, &Destination)>) {
    for (mut transform, dest) in q.iter_mut() {
        if dest.x > transform.translation.x {
            transform.translation.x += dest.speed;
        }
        if dest.x < transform.translation.x {
            transform.translation.x -= dest.speed;
        }

        if dest.y > transform.translation.y {
            transform.translation.y += dest.speed;
        }
        if dest.y < transform.translation.y {
            transform.translation.y -= dest.speed;
        }
    }

}


pub fn change_circle_destination(mut q: Query<&mut Destination, With<ShiftyCircle>>) {
    let mut rng = thread_rng();
    for mut dest in q.iter_mut() {
        dest.x = rng.gen_range(DEST_LOW_X..DEST_HIGH_X);
        dest.y = rng.gen_range(DEST_LOW_Y..DEST_HIGH_Y);
        dest.speed = rng.gen_range(SHIFTY_CIRCLE_MIN_SPEED..SHIFTY_CIRCLE_MAX_SPEED);
        // println!("x: {}", dest.x);
        // println!("y: {}", dest.y);
        // println!("speed: {}", dest.speed);
        // println!("---");
    }

}
