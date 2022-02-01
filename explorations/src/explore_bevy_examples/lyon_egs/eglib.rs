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


pub const CHANGER_WINDOW_WIDTH: f32 = 1600.0;
pub const CHANGER_WINDOW_HEIGHT: f32 = 1600.0;
pub const CHANGER_STEP: f64 = 1.0;
pub const CHANGER_CLEAR_CLR: Color = Color::DARK_GREEN;
const CHANGER_FILL_CLR: Color = Color::ORANGE;
const CHANGER_STROKE_CLR: Color = Color::BLACK;
const CHANGER_STROKE: f32 = 10.0;


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
    
    let mut path_builder = PathBuilder::new();
    path_builder.move_to(Vec2::ZERO);
    path_builder.line_to(Vec2::new(
        rng.gen_range(-CHANGER_WINDOW_WIDTH/2.0..CHANGER_WINDOW_WIDTH/2.0),
        rng.gen_range(-CHANGER_WINDOW_HEIGHT/2.0..CHANGER_WINDOW_HEIGHT/2.0)
    ));
    path_builder.line_to(Vec2::new(
        rng.gen_range(-CHANGER_WINDOW_WIDTH/2.0..CHANGER_WINDOW_WIDTH/2.0),
        rng.gen_range(-CHANGER_WINDOW_HEIGHT/2.0..CHANGER_WINDOW_HEIGHT/2.0)
    ));
    path_builder.close();
    let new_path = path_builder.build().0;

    // let new_path = shapes::RegularPolygon {
    //     sides: rng.gen_range(3..8),
    //     feature: shapes::RegularPolygonFeature::Radius(200.0),
    //     ..shapes::RegularPolygon::default()
    // };

    let mut path = query.iter_mut().next().unwrap();
    *path = ShapePath::build_as(&new_path);
}