use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
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


/*
 * path_changer 
 * 
 * Instructed by:
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/path.rs
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/dynamic_shape.rs
 */


// pub const CHANGER_WINDOW_WIDTH: f32 = 1600.0;
// pub const CHANGER_WINDOW_HEIGHT: f32 = 1600.0;
pub const CHANGER_WINDOW_WIDTH: f32 = 3200.0;
pub const CHANGER_WINDOW_HEIGHT: f32 = 2400.0;
pub const CHANGER_STEP: f64 = 1.0;
pub const CHANGER_CLEAR_CLR: Color = Color::DARK_GREEN;
const CHANGER_FILL_CLR: Color = Color::ORANGE;
const CHANGER_STROKE_CLR: Color = Color::BLACK;
const CHANGER_STROKE: f32 = 10.0;
const CHANGER_COORDS_WIDTH: f32 = CHANGER_WINDOW_WIDTH/2.0;
const CHANGER_COORDS_HEIGHT: f32 = CHANGER_WINDOW_HEIGHT/2.0;
const CHANGER_MAX_SEGMENTS: u8 = 8;

pub fn path_changing_eg_setup(mut commands: Commands) {
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
        // DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
        DrawMode::Outlined {
            fill_mode: FillMode::color(CHANGER_FILL_CLR),
            outline_mode: StrokeMode::new(CHANGER_STROKE_CLR, CHANGER_STROKE),
        },
        Transform::default(),
    ));
}


pub fn path_changer(mut query: Query<&mut Path>) {
    let mut rng = thread_rng();

    let num_segments = rng.gen_range(2..CHANGER_MAX_SEGMENTS);
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::ZERO);

    // @HINT
    // Using an underscore to discard the iterator value since it's not being used
    for _i in 0..num_segments {
        path_builder.line_to(Vec2::new(
            rng.gen_range(-CHANGER_COORDS_WIDTH..CHANGER_COORDS_WIDTH),
            rng.gen_range(-CHANGER_COORDS_HEIGHT..CHANGER_COORDS_HEIGHT)
        ));
    }
    path_builder.close();
    let new_path = path_builder.build().0;

    let mut path = query.iter_mut().next().unwrap();
    *path = ShapePath::build_as(&new_path);
}


/*
 * curve_eg
 * 
 * Instructed by:
 * https://github.com/Nilirad/bevy_prototype_lyon/blob/master/src/path.rs
 */

pub const CURVE_WINDOW_WIDTH: f32 = 1600.0;
pub const CURVE_WINDOW_HEIGHT: f32 = 1600.0;
pub const CURVE_STEP: f64 = 1.0;
pub const CURVE_CLEAR_CLR: Color = Color::DARK_GRAY;
const CURVE_FILL_CLR: Color = Color::ORANGE;
const CURVE_STROKE_CLR: Color = Color::WHITE;
const CURVE_STROKE: f32 = 5.0;

const CURVE_CTRL_X: f32 = 200.0;
const CURVE_CTRL_Y: f32 = 200.0;
const CURVE_VALLEY_RADIUS: f32 = 100.0; // Radius to curve intersection

pub fn curve_eg_setup(mut commands: Commands) {
    let mut path_builder = PathBuilder::new();

    // Right side top
    path_builder.move_to(Vec2::new(0.0, CURVE_VALLEY_RADIUS));
    path_builder.quadratic_bezier_to(
        Vec2::new(CURVE_CTRL_X, CURVE_CTRL_Y), 
        Vec2::new(CURVE_VALLEY_RADIUS, 0.0)
    );

    // Right side bottom
    path_builder.quadratic_bezier_to(
        Vec2::new(CURVE_CTRL_X, -CURVE_CTRL_Y), 
        Vec2::new(0.0, -CURVE_VALLEY_RADIUS)
    );

    // Left side bottom
    path_builder.quadratic_bezier_to(
        Vec2::new(-CURVE_CTRL_X, -CURVE_CTRL_Y), 
        Vec2::new(-CURVE_VALLEY_RADIUS, 0.0)
    );

    // Left side top
    path_builder.quadratic_bezier_to(
        Vec2::new(-CURVE_CTRL_X, CURVE_CTRL_Y), 
        Vec2::new(0.0, CURVE_VALLEY_RADIUS)
    );
    path_builder.close();

    let path = path_builder.build().0;

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(GeometryBuilder::build_as(
        &path,
        // DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
        DrawMode::Outlined {
            fill_mode: FillMode::color(CURVE_FILL_CLR),
            outline_mode: StrokeMode::new(CURVE_STROKE_CLR, CURVE_STROKE),
        },
        Transform::default(),
    ));
}

// Curve example of drawing a heart that got too convoluted for use an an example. 
//
// pub const CURVE_WINDOW_WIDTH: f32 = 1600.0;
// pub const CURVE_WINDOW_HEIGHT: f32 = 1600.0;
// pub const CURVE_STEP: f64 = 1.0;
// pub const CURVE_CLEAR_CLR: Color = Color::DARK_GRAY;
// const CURVE_FILL_CLR: Color = Color::ORANGE;
// const CURVE_STROKE_CLR: Color = Color::WHITE;
// const CURVE_STROKE: f32 = 10.0;
// const CURVE_COORDS_WIDTH: f32 = CURVE_WINDOW_WIDTH/2.0;
// const CURVE_COORDS_HEIGHT: f32 = CURVE_WINDOW_HEIGHT/2.0;
// const CURVE_HEART_CREST_START: f32 = CURVE_COORDS_HEIGHT / 4.0; // Where the mounds start
// const CURVE_HEART_BOTTOM: f32 = -CURVE_COORDS_HEIGHT + CURVE_COORDS_HEIGHT / 6.0;
// const CURVE_HEART_PEAK: f32 = CURVE_WINDOW_HEIGHT - CURVE_WINDOW_HEIGHT / 4.0;

// pub fn curve_eg_setup(mut commands: Commands) {
//     let mut path_builder = PathBuilder::new();
//     let start_location = Vec2::new(0.0, CURVE_HEART_CREST_START);
//     path_builder.move_to(start_location);
//     path_builder.quadratic_bezier_to(
//         Vec2::new(CURVE_COORDS_WIDTH / 2.0, CURVE_HEART_PEAK), 
//         Vec2::new(CURVE_COORDS_WIDTH - CURVE_COORDS_WIDTH / 6.0, CURVE_HEART_CREST_START)
//     );

//     path_builder.line_to(Vec2::new(start_location.x, CURVE_HEART_BOTTOM));
//     path_builder.line_to(Vec2::new(-
//         CURVE_COORDS_WIDTH + CURVE_COORDS_WIDTH / 6.0, 
//         CURVE_HEART_CREST_START
//     ));
//     path_builder.quadratic_bezier_to(
//         Vec2::new(-CURVE_COORDS_WIDTH / 2.0, CURVE_HEART_PEAK), start_location
//     );
//     path_builder.close();

//     /*
//      * Irf: Temporary workaround until the fix mentioned in this issue is released:
//      * https://github.com/Nilirad/bevy_prototype_lyon/issues/138
//      */ 
//     // let line = path_builder.build();
//     let line = path_builder.build().0;

//     commands.spawn_bundle(OrthographicCameraBundle::new_2d());
//     commands.spawn_bundle(GeometryBuilder::build_as(
//         &line,
//         // DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
//         DrawMode::Outlined {
//             fill_mode: FillMode::color(CURVE_FILL_CLR),
//             outline_mode: StrokeMode::new(CURVE_STROKE_CLR, CURVE_STROKE),
//         },
//         Transform::default(),
//     ));
// }
