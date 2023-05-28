use bevy::prelude::*;

use self::slider::{drag_slider, start_drag_slider, SliderHandle};

pub mod slider;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(drag_slider)
            .add_system(start_drag_slider)
            .register_type::<SliderHandle>();
    }
}
