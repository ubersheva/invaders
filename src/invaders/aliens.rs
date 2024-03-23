use bevy::math::Rect;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::{Rng};
use crate::invaders::{InvadersGame, InvaderState, MInvaders};
use crate::invaders::bat::MBat;
use crate::invaders::bullet::{MAlienBullet, MBullet, ShootEvent};
use crate::MainState;


pub struct AliensPlugin<T: States+Copy> {
    mystate: T,
}

impl<T:States+Copy> AliensPlugin<T> {
    pub fn for_state(state: T) -> Self {
        Self {
            mystate: state,
        }
    }
}

impl<T:States+Copy> Plugin for AliensPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AliensAssets>()

            .add_systems(OnEnter(MainState::Invaders), aliens_setup)
            .add_systems(Update, aliens_move.run_if(in_state(self.mystate)))
            .add_systems(Update, check_shot.run_if(in_state(self.mystate)))
            .add_systems(PostUpdate, check_win.run_if(in_state(self.mystate)))
            .add_systems(PostUpdate, check_lose.run_if(in_state(self.mystate)))
            .add_systems(PostUpdate, shoot.run_if(in_state(self.mystate)))

            .add_systems(Update, cheat_win.run_if(in_state(self.mystate)))
        ;
    }
}

#[derive(Component, Debug)]
struct MAlienBox {
    area: Rect,
    last_update: f32,
    step: f32,
}

#[derive(Component, Debug)]
struct MAlien;

#[derive(Resource)]
struct AliensAssets {
    alien_mesh: Handle<Mesh>,
    alien_mat: Handle<ColorMaterial>,
}

impl FromWorld for AliensAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            alien_mesh: world.resource_mut::<Assets<Mesh>>().add(Rectangle::new(1.0, 1.0)),
            alien_mat: world.resource_mut::<Assets<ColorMaterial>>().add(Color::rgb_u8(81, 130, 155)),
        }
    }
}

fn aliens_setup (
    mut commands: Commands,
    assets: Res<AliensAssets>
) {
    let alien_area = Rect {
        min: Vec2::new(-640.0, -100.0),
        max: Vec2::new(460.0, 400.0),
    };
    let inv_size = alien_area.width() / (2 * 11 - 1) as f32;

    commands.spawn((
        MInvaders,
        MAlienBox {
            area: alien_area,
            last_update: 0.0,
            step: inv_size / 3.0,
        },
        Transform::from_xyz(alien_area.center().x, alien_area.center().y, 0.0),
        GlobalTransform::IDENTITY,
        InheritedVisibility::default(),
        Visibility::default(),
    )).with_children(|commands| {
        for y in 0..6 {
            for x in 0..11 {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(assets.alien_mesh.clone()),
                        material: assets.alien_mat.clone(),
                        transform: Transform::from_xyz(
                            -alien_area.width() / 2.0 + x as f32 * (2.0*inv_size) + inv_size / 2.0,
                            alien_area.height() / 2.0 - y as f32 * (inv_size + (alien_area.height() - 6.0*inv_size)/5.0) - inv_size / 2.0,
                            0.0).with_scale(Vec3::ONE * inv_size),
                        ..default()
                    },
                    MAlien));
            }
        }
    });
}


fn aliens_move(
    mut qalien_box: Query<(&mut MAlienBox, &mut Transform, Option<&Children>)>,
    qaliens: Query<&Transform, Without<MAlienBox>>,
    time: Res<Time>
) {
    let (mut alien_box, mut area, Some(children)) = qalien_box.single_mut() else {
        return;
    };
    let now = time.elapsed_seconds();
    if now - alien_box.last_update < 1.0 {
        return;
    }
    alien_box.last_update = now;
    let mut area_new = Rect::default();
    for &alien_entity in children {
        let Ok(alien) = qaliens.get(alien_entity) else {
            error!("entity  {:?} not found", alien_entity);
            continue;
        };
        let arect = Rect::from_center_size(alien.translation.xy(), alien.scale.xy());
        if area_new.is_empty() {
            area_new = arect;
        } else {
            area_new = area_new.union(arect);
        }
        // println!("child {:?} scale {:?}->{:?}", arect, alien.scale, area_new);
    }
    area_new.min += area.translation.xy();
    area_new.max += area.translation.xy();
    alien_box.area = area_new;

    if alien_box.step > 0.0 {
        if area_new.max.x < 640.0 - alien_box.step {
            area.translation.x += alien_box.step;
        } else {
            area.translation.y -= alien_box.step;
            alien_box.step *= -1.0;
        }
    } else {
        if area_new.min.x > -640.0 - alien_box.step {
            area.translation.x += alien_box.step;
        } else {
            area.translation.y += alien_box.step;
            alien_box.step *= -1.0;
        }
    }
    // println!("new alien area {:?}", area_new);
}

fn check_shot(
    mut commands: Commands,
    qaliens: Query<(Entity, &Parent, &GlobalTransform), With<MAlien>>,
    qbullet: Query<(Entity, &Transform), (With<MBullet>, Without<MAlienBullet>)>,
    mut game: ResMut<InvadersGame>,
) {
    for (ae, ap, at) in qaliens.iter() {
        for (be, bt) in qbullet.iter() {
            let at = at.compute_transform();
            let abox = Rect::from_center_size(at.translation.xy(), at.scale.xy());
            if abox.contains(bt.translation.xy()) {
                commands.entity(ap.get()).remove_children(&[ae]);
                commands.entity(ae).despawn();
                commands.entity(be).despawn();
                game.score += 30;
            }
        }
    }
}


fn cheat_win(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    qaliens: Query<(Entity, &Parent), With<MAlien>>
) {
    if input.just_pressed(KeyCode::KeyZ) && input.any_pressed([KeyCode::AltRight, KeyCode::AltLeft]) {
        for (e, p) in qaliens.iter() {
            commands.entity(p.get()).remove_children(&[e]);
            commands.entity(e).despawn();
        }
    }
}

fn check_win(
    qaliens: Query<&MAlien>,
    // mut game: ResMut<InvadersGame>,
    mut state: ResMut<NextState<InvaderState>>,
) {
    if !qaliens.is_empty() {
        return;
    }
    // game.win = true;
    state.set(InvaderState::Win);
}

fn check_lose(
    alien_box: Query<&MAlienBox>,
    // mut game: ResMut<InvadersGame>,
    mut state: ResMut<NextState<InvaderState>>,
) {
    let alien_box = alien_box.single();
    if alien_box.area.min.y <= -350.0 {
        state.set(InvaderState::Gameover);
    }
}

fn shoot(
    qbox: Query<(Entity, Option<&Children>, &Transform), With<MAlienBox>>,
    qalien: Query<&Transform, With<MAlien>>,
    game: Res<InvadersGame>,
    mut event: EventWriter<ShootEvent>,
    time: Res<Time>,
    mut last_upd: Local<f32>,
    qbat: Query<&Transform, With<MBat>>,
) {
    let now = time.elapsed_seconds_wrapped();
    let delta = now - *last_upd;
    if delta < 9.0 / (3.0 + game.time / 10.0) {
        return;
    }
    *last_upd = now;

    let (_e, Some(_children), alien_box) = qbox.single() else {
        return;
    };
    let mut rng = rand::thread_rng();
    if rng.gen::<f32>() > 0.5 {
        let bat = qbat.single();
        let shooters = qalien.iter()
            .filter(|t| (t.translation.x + alien_box.translation.x - bat.translation.x).abs() < 200.0)
            .collect::<Vec<_>>();
        // println!("shooters {}", shooters.len());
        if shooters.is_empty() {
            return;
        }
        let alien = shooters[rng.gen::<usize>() % shooters.len()];
        // let Ok(alien) = qalien.get(children[rng.gen::<usize>() % children.len()]) else {
        //     return;
        // };
        event.send(ShootEvent::new(
            (alien.translation + alien_box.translation + Vec3::NEG_Y * alien.scale / 2.0).xy(),
            Vec2::new(0.0, -150.0),
            true
        ));
        // println!("shoot! {:?}", &children.len());
    }
}