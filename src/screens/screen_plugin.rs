use bevy::prelude::*;

use crate::screens::splashscreen::splashscreen_plugin::SplashScreenPlugin;

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SplashScreenPlugin));
    }
}
