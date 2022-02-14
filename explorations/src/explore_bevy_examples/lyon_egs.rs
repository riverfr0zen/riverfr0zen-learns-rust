use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::core::FixedTimestep;
#[cfg(feature = "framestats")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod eglib;
pub mod shiftyc;

#[cfg(target_arch = "wasm32")]
const RESIZE_CHECK_STEP: f64 = 1.0;


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

    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
            title: "Shifty Circle".to_string(),
            width: shiftyc::WINDOW_WIDTH,
            height: shiftyc::WINDOW_HEIGHT,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#window-matching-canvas".to_string()),
            ..Default::default()
        }
    ).insert_resource(shiftyc::AppGlobals {
        dest_low_x: -shiftyc::WINDOW_WIDTH / 2.0,
        dest_high_x: shiftyc::WINDOW_WIDTH / 2.0,
        dest_low_y: -shiftyc::WINDOW_HEIGHT / 2.0,
        dest_high_y: shiftyc::WINDOW_HEIGHT / 2.0,
    }).insert_resource(ClearColor(shiftyc::CLEAR_COLOR))
    .insert_resource(Msaa { samples: 4 });

    info!("--Logging does not start before DefaultPlugins so this log won't appear--");
    app.add_plugins(DefaultPlugins);
    warn!("--Logging has been set up in DefaultPlugins--");

    app.add_plugin(ShapePlugin);

    // Example of "feature-flipping". 
    // See https://doc.rust-lang.org/cargo/reference/features.html
    #[cfg(feature = "framestats")]
    app.add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default());

    app.add_startup_system(shiftyc::setup_shifty_circle);

    #[cfg(target_arch = "wasm32")]
    app.add_startup_system(shiftyc::setup_browser_size)
    .add_system(
        shiftyc::handle_browser_resize.with_run_criteria(FixedTimestep::step(RESIZE_CHECK_STEP))
    );

    // Note setting with_run_criteria on a single system
    // (Found it here: https://bevy-cheatbook.github.io/programming/run-criteria.html#run-criteria-labels)
    app.add_system(
        shiftyc::translate_circle
            .with_run_criteria(FixedTimestep::step(shiftyc::SHIFTY_CIRCLE_STEP))
    ).add_system(
        shiftyc::change_circle_destination
            .with_run_criteria(FixedTimestep::step(shiftyc::SHIFTY_CHANGE_STEP))
    );

    app.run();
}


pub fn lyon_path_changing_eg_app() {
    // From https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/path.rs
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Path Changing Example".to_string(),
            width: eglib::CHANGER_WINDOW_WIDTH,
            height: eglib::CHANGER_WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(eglib::CHANGER_CLEAR_CLR))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(eglib::path_changing_eg_setup)
        .add_system(
            eglib::path_changer
                .with_run_criteria(FixedTimestep::step(eglib::CHANGER_STEP)))
        .run();
}


pub fn lyon_curve_eg_app() {
    // From https://github.com/Nilirad/bevy_prototype_lyon/blob/master/examples/path.rs
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Curve Example".to_string(),
            width: eglib::CURVE_WINDOW_WIDTH,
            height: eglib::CURVE_WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(eglib::CURVE_CLEAR_CLR))
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(eglib::curve_eg_setup)
        .run();
}

