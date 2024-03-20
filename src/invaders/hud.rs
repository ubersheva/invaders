use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::log::error;
use bevy::prelude::*;
use crate::invaders::{InvadersGame, InvaderState};

use crate::MainState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(MainState::Invaders), create_hud)
            .add_systems(OnExit(MainState::Invaders), destroy_hud)
            .add_systems(Update, update_hud.run_if(in_state(InvaderState::Game)))
        ;
    }
}

#[derive(Component)]
struct MHud;

fn create_hud(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let Some(font) = assets.get_handle("eight-bit-dragon.otf") else {
        error!("menu font not loaded");
        return;
    };

    let color = Color::rgb_u8(81, 130, 155);
    commands.spawn((
        TextBundle {
            text: Text::from_section("Score", TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color,
            }),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(5.0),
                top: Val::Px(795.0),
                ..default()
            },
            ..default()
        },
        MHud,
    ));
}

fn destroy_hud(
    mut commands: Commands,
    qhud: Query<Entity, With<MHud>>,
) {
    for e in qhud.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn update_hud(
    mut qscore: Query<&mut Text, With<MHud>>,
    game: Res<InvadersGame>,
) {
    if let Ok(mut text) = qscore.get_single_mut() {
        text.sections[0].value = format!("Score: {}", game.score);
    }
}