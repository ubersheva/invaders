use bevy::app::{App, Plugin, Update};
use bevy::asset::{Assets, Handle};
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::invaders::{InvadersGame, InvaderState, MInvaders};
use crate::invaders::bullet::{MBullet, ShootEvent};
use crate::MainState;

pub struct BatPlugin<T: States+Copy> {
    mystate: T,
}

impl<T:States+Copy> BatPlugin<T> {
    pub fn for_state(state: T) -> Self {
        Self {
            mystate: state,
        }
    }
}

impl<T:States+Copy> Plugin for BatPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<BatAssets>()

            .add_systems(OnEnter(MainState::Invaders), bat_setup)

            .add_systems(PreUpdate, bat_key_input.run_if(in_state(self.mystate)))
            .add_systems(PreUpdate, bat_shoot.run_if(in_state(InvaderState::Game)))
            .add_systems(Update, bat_update.run_if(in_state(self.mystate)))
            .add_systems(Update, check_shot.run_if(in_state(self.mystate)))
        ;
    }
}


#[derive(Component, Default, Debug)]
pub struct MBat {
    mass: f32,
    v: f32,
    f: f32,
    last_shoot: f32,
}

#[derive(Resource)]
struct BatAssets {
    bat_mesh: Handle<Mesh>,
    bat_mat: Handle<ColorMaterial>,
}

impl FromWorld for BatAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            bat_mesh: world.resource_mut::<Assets<Mesh>>().add(Rectangle::new(1.0, 1.0)),
            bat_mat: world.resource_mut::<Assets<ColorMaterial>>().add(Color::rgb_u8(246, 153, 92)),
        }
    }
}

fn bat_setup(
    mut commands: Commands,
    assets: Res<BatAssets>
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(assets.bat_mesh.clone()),
            material: assets.bat_mat.clone(),
            transform: Transform::from_xyz(0.0, -360.0, -1.0).with_scale([80.0, 20.0, 1.0].into()),
            ..default()
        },
        MInvaders,
        MBat {
            mass: 1.0,
            ..default()
        }));
}

fn bat_update(
    mut qbat: Query<(&mut MBat, &mut Transform)>,
    game: Res<InvadersGame>,
    time: Res<Time>
) {
    let (mut bat, mut transform) = qbat.single_mut();

    let delta = time.delta_seconds();
    bat.v += delta * (bat.f - bat.v * game.drag) / bat.mass;
    transform.translation.x += delta * bat.v;
    // println!("after transform: {:?} f {},dd {:?} {:?}", bat, bat.f - bat.v * game.drag, transform, game);

    let x_range = (-640.0 + transform.scale.x / 2.0)..=(640.0 - transform.scale.x / 2.0);
    if !x_range.contains(&transform.translation.x) {
        transform.translation.x = transform.translation.x.clamp(*x_range.start(), *x_range.end());
        bat.v = - bat.v * 0.7;
    }
}

fn bat_key_input(
    input: Res<ButtonInput<KeyCode>>,
    mut qbat: Query<&mut MBat>,
    game: Res<InvadersGame>,
) {
    let mut new_f = 0.0;
    if input.pressed(KeyCode::KeyD) {
        new_f += game.f;
    }
    if input.pressed(KeyCode::KeyA) {
        new_f -= game.f;
    }
    qbat.single_mut().f = new_f;
}

fn bat_shoot (
    mut event: EventWriter<ShootEvent>,
    input: Res<ButtonInput<MouseButton>>,
    mut qbat: Query<(&mut MBat, &Transform)>,
    time: Res<Time>,
    mut game: ResMut<InvadersGame>,
) {
    let (mut bat, t) = qbat.single_mut();
    if input.pressed(MouseButton::Left) {
        let now = time.elapsed_seconds_wrapped();
        if now - bat.last_shoot >= game.shoot_delay {
            event.send(ShootEvent::new((t.translation + Vec3::Y * 20.0).xy(), Vec2::new(0.0, 100.0), false));
            bat.last_shoot = now;
            game.score = 0.max(game.score - 10);
        }
    }
}

fn check_shot(
    mut commands: Commands,
    qbat: Query<(&MBat, &Transform), With<MBat>>,
    qbullet: Query<(Entity, &Transform), With<MBullet>>,
    mut state: ResMut<NextState<InvaderState>>,
    mut game: ResMut<InvadersGame>,
) {
    let (_bat, tbat) = qbat.single();
    for (e, t) in qbullet.iter() {
        let bat_box = Rect::from_center_size(tbat.translation.xy(), tbat.scale.xy());
        if bat_box.contains(t.translation.xy()) {
            commands.entity(e).despawn();
            game.gameover = true;
            state.set(InvaderState::Menu);
        }
    }
}