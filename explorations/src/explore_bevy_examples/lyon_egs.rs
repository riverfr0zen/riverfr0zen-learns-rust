use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;


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