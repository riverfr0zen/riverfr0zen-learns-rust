use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::core::FixedTimestep;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod eglib;
pub mod shiftyc;

pub fn lyon_eg_app() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(eglib::eg1_setup)
        .run();
}


pub fn lyon_path_eg_app() {
    // From https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/path.rs
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(eglib::path_eg_setup)
        .run();
}


pub fn shifty_circle_app() {
    // From https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/path.rs
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Shifty Circle".to_string(),
            width: shiftyc::WINDOW_WIDTH,
            height: shiftyc::WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(shiftyc::CLEAR_COLOR))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(shiftyc::setup_shifty_circle)
        // Note setting with_run_criteria on a single system
        // (Found it here: https://bevy-cheatbook.github.io/programming/run-criteria.html#run-criteria-labels)
        .add_system(
            shiftyc::translate_circle
                .with_run_criteria(FixedTimestep::step(shiftyc::SHIFTY_CIRCLE_STEP)))
        .add_system(
            shiftyc::change_circle_destination
                .with_run_criteria(FixedTimestep::step(shiftyc::SHIFTY_CHANGE_STEP)))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}


