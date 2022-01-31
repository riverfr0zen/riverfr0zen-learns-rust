use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Circle;
use rand::Rng;
use rand::prelude::thread_rng;


pub fn eg1_setup(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
        },
        Transform::default(),
    ));
}


pub fn path_eg_setup(mut commands: Commands) {
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::ZERO);
    path_builder.line_to(100.0 * Vec2::ONE);

    path_builder.line_to(Vec2::new(100.0, 0.0));
    path_builder.close();

    /*
     * Irf: Temporary workaround until the fix mentioned in this issue is released:
     * https://github.com/Nilirad/bevy_prototype_lyon/issues/138
     */ 
    // let line = path_builder.build();
    let line = path_builder.build().0;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &line,
        DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
        Transform::default(),
    ));
}


pub const SHIFTY_CIRCLE_RADIUS: f32 = 100.0;
pub const SHIFTY_CIRCLE_STROKE: f32 = 1.0;
pub const SHIFTY_CIRCLE_SPEED: f32 = 1.0; // I.e. how much it translates per step
pub const SHIFTY_CIRCLE_STEP: f64 = 0.01;
pub const SHIFTY_CHANGE_STEP: f64 = 0.1;
pub const WINDOW_HEIGHT: f32 = 1600.0;
pub const WINDOW_WIDTH: f32 = 1600.0;


#[derive(Component)]
pub struct ShiftyCircle;


pub fn setup_shifty_circle(mut commands: Commands) {
    let mut rng = thread_rng();
    let mycircle = shapes::Circle {
        radius: SHIFTY_CIRCLE_RADIUS,
        // I thought this could be a way to start the circle in a random
        // position, but found out what this does is set the center of
        // the circle **in the world**. Any future translations would
        // be based off this random center. Of, course that isn't what
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
            fill_mode: FillMode::color(Color::rgba(0.0, 1.0, 0.0, 1.0)),
            outline_mode: StrokeMode::new(Color::BLACK, 5.0),
        },
        Transform::default(),
    ))
    .insert(ShiftyCircle);
}


// From snake:
// pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
pub fn translate_circle(mut q: Query<&mut Transform, With<ShiftyCircle>>) {
    for mut transform in q.iter_mut() {
        // println!("{}", transform.translation.x);
        transform.translation.x += SHIFTY_CIRCLE_SPEED;
        transform.translation.y += SHIFTY_CIRCLE_SPEED / 2.0;
    }

}


pub fn change_circle_destination(mut q: Query<&mut Transform, With<ShiftyCircle>>) {

    let mut rng = thread_rng();
    for mut transform in q.iter_mut() {
        transform.translation.x = rng.gen_range(
            -WINDOW_WIDTH/2.0+SHIFTY_CIRCLE_RADIUS..WINDOW_WIDTH/2.0-SHIFTY_CIRCLE_RADIUS
        );
        transform.translation.y = rng.gen_range(
            -WINDOW_HEIGHT/2.0+SHIFTY_CIRCLE_RADIUS..WINDOW_HEIGHT/2.0-SHIFTY_CIRCLE_RADIUS
        );
        println!("x: {}", transform.translation.x);
        println!("y: {}", transform.translation.y);
        println!("---");
    }

}