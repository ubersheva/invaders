use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::invaders::{InvaderState, MInvaders};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShootEvent>()
            .init_resource::<BulletAssets>()

            .add_systems(Update, bullet_move.run_if(in_state(InvaderState::Game)))
            .add_systems(Update, bullet_cleanup.run_if(in_state(InvaderState::Game)))
            .add_systems(Update, spawn_bullet.run_if(in_state(InvaderState::Game)))
        ;
    }
}


#[derive(Component)]
pub struct MBullet {
    v: Vec2,
}

#[derive(Component)]
pub struct MAlienBullet;

#[derive(Event)]
pub struct ShootEvent {
    pos: Vec2,
    v: Vec2,
    alien: bool,
}

impl ShootEvent {
    pub fn new(pos: Vec2, v: Vec2, alien: bool) -> Self {
        Self {pos, v, alien}
    }
}

#[derive(Resource)]
struct BulletAssets {
    mesh: Handle<Mesh>,
    mat: Handle<ColorMaterial>,
}

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            mesh: world.resource_mut::<Assets<Mesh>>().add(Circle::new(3.0)),
            mat: world.resource_mut::<Assets<ColorMaterial>>().add(Color::rgb_u8(155, 176, 193)),
        }
    }
}

fn bullet_move (
    time: Res<Time>,
    mut qbullets: Query<(&MBullet, &mut Transform)>
) {
    let delta = time.delta_seconds();
    for (b, mut t) in qbullets.iter_mut() {
        t.translation.x += b.v.x * delta;
        t.translation.y += b.v.y * delta;
    }
}

fn bullet_cleanup(
    mut commands: Commands,
    qbullets: Query<(Entity, &ViewVisibility), With<MBullet>>
) {
    for (e, vis) in qbullets.iter() {
        if !vis.get() {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn spawn_bullet(mut event: EventReader<ShootEvent>, mut commands: Commands, assets: Res<BulletAssets>) {
    for e in event.read() {
        let mut bullet = commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(assets.mesh.clone()),
                material: assets.mat.clone(),
                transform: Transform::from_xyz(e.pos.x, e.pos.y, 0.0),
                ..default()
            },
            MBullet { v: e.v },
            MInvaders,
        ));
        if e.alien {
            bullet.insert(MAlienBullet);
        }
    }
}