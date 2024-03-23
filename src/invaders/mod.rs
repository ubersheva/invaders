use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::invaders::aliens::AliensPlugin;
use crate::invaders::bat::BatPlugin;
use crate::invaders::bullet::BulletPlugin;
use crate::invaders::hud::HudPlugin;
use crate::invaders::menu::{MenuPlugin};
use crate::MainState;

mod bat;
mod aliens;
mod bullet;
mod menu;
mod hud;

pub struct InvadersPlugin<T: States+Copy> {
    mystate: T,
}

impl<T: States+Copy> InvadersPlugin<T> {
    pub fn for_state(state: T) -> Self {
        Self {
            mystate: state
        }
    }
}

impl<T: States+Copy> Plugin for InvadersPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ExitEvent>()

            .insert_resource(InvadersGame {
                drag: 1.0,
                f: 100.0,
                shoot_delay: 1.5,
                ..default()
            })

            .insert_state(InvaderState::None)

            .add_systems(OnEnter(self.mystate), invaders_setup)
            .add_systems(OnEnter(InvaderState::Game), clear_input)
            .add_systems(OnExit(self.mystate), invaders_exit)
            .add_systems(PreUpdate, invaders_key_input.run_if(in_state(InvaderState::Game)))
            .add_systems(Update, invaders_exit_event.run_if(in_state(InvaderState::Game)))
            .add_systems(Update, reduce_score.run_if(in_state(InvaderState::Game)))
            .add_systems(Update, count_time.run_if(in_state(InvaderState::Game)))

            .add_plugins(BatPlugin::for_state(InvaderState::Game))
            .add_plugins(AliensPlugin::for_state(InvaderState::Game))
            .add_plugins(BulletPlugin)
            .add_plugins(MenuPlugin)
            .add_plugins(HudPlugin)
        ;
    }
}

#[derive(Resource, Default, Debug)]
struct InvadersGame {
    drag: f32,
    f: f32,
    shoot_delay: f32,
    score: i32,
    time: f32,
}


#[derive(Event)]
struct ExitEvent;

#[derive(Component)]
struct MInvaders;

#[derive(Component, States, Clone, PartialEq, Eq, Hash, Debug, Copy)]
enum InvaderState {
    None,
    Start,
    Game,
    Pause,
    Win,
    Gameover,
}

fn invaders_setup(
    mut camera: Query<(&mut OrthographicProjection, &mut Transform)>,
    mut invaders_state: ResMut<NextState<InvaderState>>,
    mut game: ResMut<InvadersGame>,
) {
    let (mut projection, mut cam_trans) = camera.single_mut();
    projection.scaling_mode = ScalingMode::AutoMin {min_width: 1280.0, min_height: 840.0};
    cam_trans.translation = Vec3::ZERO;

    game.score = 0;
    game.time = 0.0;

    invaders_state.set(InvaderState::Start);
}

fn invaders_key_input(
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<InvaderState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        state.set(InvaderState::Pause);
        input.clear_just_pressed(KeyCode::Escape);
        // println!("input {:?}", i);
    }
}

fn invaders_exit_event(
    event: EventReader<ExitEvent>,
    mut state: ResMut<NextState<MainState>>,
    mut inv_state: ResMut<NextState<InvaderState>>
) {
    if !event.is_empty() {
        state.set(MainState::MainMenu);
        inv_state.set(InvaderState::None);
    }
}

fn invaders_exit(mut commands: Commands, mut q: Query<Entity, With<MInvaders>>) {
    for entity in q.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

fn reduce_score(
    mut game: ResMut<InvadersGame>,
    time: Res<Time>,
    mut last_update: Local<f32>,
) {
    let now = time.elapsed_seconds_wrapped();
    if now - *last_update > 1.0 {
        game.score = 0.max(game.score - 2);
        *last_update = now;
    }
}

fn clear_input(
    mut key: ResMut<ButtonInput<KeyCode>>,
    mut mouse: ResMut<ButtonInput<MouseButton>>
) {
    key.reset_all();
    mouse.reset_all();
}

fn count_time(
    mut game: ResMut<InvadersGame>,
    time: Res<Time>,
) {
    game.time += time.delta_seconds();
}