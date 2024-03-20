use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::{EnabledButtons, PresentMode};

use crate::fps_counter::FpsCounterPlugin;
use crate::invaders::InvadersPlugin;
use crate::main_menu::MainMenuPlugin;

mod invaders;
mod fps_counter;
mod main_menu;
mod state_plugin;

//palette https://colorhunt.co/palette/eadfb49bb0c151829bf6995c
// rgb(234, 223, 180)
// rgb(155, 176, 193)
// rgb(81, 130, 155)
// rgb(246, 153, 92)

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        enabled_buttons: EnabledButtons {
                            minimize: false,
                            maximize: false,
                            close: true,
                        },
                        resize_constraints: WindowResizeConstraints {
                            min_width: 1280.0,
                            min_height: 840.0,
                            max_width: 1280.0,
                            max_height: 840.0,
                        },
                        title: "Invaders etc.".into(),
                        ..default()
                    }),
                    ..default()
                })
        )
        .add_systems(Startup, setup)
        .insert_state(MainState::MainMenu)
        .add_plugins(FpsCounterPlugin::default())
        .add_plugins(MainMenuPlugin::for_state(MainState::MainMenu))
        .add_plugins(InvadersPlugin::for_state(MainState::Invaders))

        .run();
}

fn setup(mut commands: Commands) {
    let mut cam_bundle = Camera2dBundle::default();
    cam_bundle.camera.clear_color = ClearColorConfig::Custom(Color::rgb_u8(234, 223, 180));
    commands.spawn(cam_bundle);
}

#[derive(Component, States, Clone, PartialEq, Eq, Hash, Debug, Copy)]
enum MainState {
    MainMenu,
    Invaders,
}
