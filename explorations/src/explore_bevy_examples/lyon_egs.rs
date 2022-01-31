use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::core::FixedTimestep;

pub mod eglib;


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
            width: eglib::WINDOW_WIDTH,
            height: eglib::WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(eglib::setup_shifty_circle)
        // Note setting with_run_criteria on a single system
        // (Found it here: https://bevy-cheatbook.github.io/programming/run-criteria.html#run-criteria-labels)
        .add_system(
            eglib::translate_circle
                .with_run_criteria(FixedTimestep::step(eglib::SHIFTY_CIRCLE_STEP)))
        .add_system(
            eglib::change_circle_destination
                .with_run_criteria(FixedTimestep::step(eglib::SHIFTY_CHANGE_STEP)))
        .run();
}


