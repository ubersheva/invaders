use bevy::prelude::*;
use bevy::app::{App, Update};
use bevy::prelude::PositionType::Absolute;

#[derive(Component)]
struct MFpsCounter;

#[derive(Component)]
struct MCursorPos;

#[derive(Component)]
struct MDebugText;

#[derive(Resource, Default)]
struct FpsCounter {
    last_update: u128,
    frames: u32,
}

pub struct FpsCounterPlugin {
    pub update_delta: u128,
}

impl Default for FpsCounterPlugin {
    fn default() -> Self {
        Self {update_delta: 500}
    }
}

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        let update_delta = self.update_delta;
        app
            .add_systems(Update, move |time: Res<Time>,
                         // mut counter: ResMut<FpsCounter>,
                         counter: Local<FpsCounter>,
                         q: Query<&mut Text, With<MFpsCounter>>| {
                fps_calc(update_delta, time, counter, q)
            })
            // .insert_resource(FpsCounter { update_delta: self.update_delta, ..default() })
            .add_systems(Startup, fpscounter_setup)

            .add_systems(Update, key_input)
            .add_systems(Update, fps_interact)
            .add_systems(Update, cursor_pos)
        ;
    }
}

fn fpscounter_setup(mut commands: Commands) {
    commands
        .spawn((
            TextBundle {
                text: Text::from_section("FPS: 9000", TextStyle{color:Color::RED,font_size:15.,..default()}),
                // style: Style { position_type: Absolute, left: Val::Px(0.),..default()},
                ..default()
            },
            Interaction::default(),
            MFpsCounter,
            MDebugText
        )).insert(Visibility::Hidden);
    commands
        .spawn((
            TextBundle {
                text: Text::from_section("cursor", TextStyle{color:Color::RED,font_size:15.,..default()}),
                style: Style { position_type: Absolute, right: Val::Percent(0.0), top: Val::Percent(0.0),..default()},
                ..default()
            },
            MCursorPos,
            MDebugText
        )).insert(Visibility::Hidden);
}

fn fps_calc(
    update_delta: u128,
    time: Res<Time>,
    // mut counter: ResMut<FpsCounter>,
    mut counter: Local<FpsCounter>,
    mut q: Query<&mut Text, With<MFpsCounter>>
) {
    if let Some(mut txt) = q.iter_mut().next() {
        let now = time.elapsed_wrapped().as_millis();
        if now - counter.last_update > update_delta {
            let fps = 1000.0 * counter.frames as f32 / (now - counter.last_update) as f32;
            txt.sections[0].value = format!("FPS: {:.2}, frame time: {:.5} ms", fps, 1000.0 / fps);
            counter.last_update = now;
            counter.frames = 0;
        } else {
            counter.frames += 1;
        }
    }
}

fn fps_interact(
    mut q: Query<(&Interaction, &mut Text), (With<MFpsCounter>, Changed<Interaction>)>,
) {
    if let Some((int, mut text)) = q.iter_mut().next() {
        // println!("fps update txt {:?} st {:?}", int, style);
        match *int {
            Interaction::Hovered => {
                text.sections[0].style.font_size = 45.0;
            }
            Interaction::None => {
                text.sections[0].style.font_size = 15.0;
            },
            _ => ()
        }
    }
}

fn cursor_pos(
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform, &OrthographicProjection)>,
    mut q: Query<&mut Text, With<MCursorPos>>
) {
    let (camera, cam_transform, projection) = camera.single();
    let Some(cursor_pos) = window.single().cursor_position() else {
        return;
    };

    let Some(pos) = camera.viewport_to_world_2d(cam_transform, cursor_pos) else {
        error!("unable to transform cursor pos to world pos");
        return;
    };
    // println!("mouse {:?} {:?} {:?}", window.single().cursor_position(), pos, projection);
    let Some(mut txt) = q.iter_mut().next() else {
        return;
    };

    txt.sections[0].value = format!("Wnd: ({},{}), Wld: ({},{}) Prj: ({},{})x({},{})",
                                    cursor_pos.x, cursor_pos.y,
                                    pos.x as i32, pos.y as i32,
                                    projection.area.min.x, projection.area.min.y, projection.area.max.x, projection.area.max.y);
}

fn key_input(
    input: Res<ButtonInput<KeyCode>>,
    mut qvisibility: Query<&mut Visibility, With<MDebugText>>
) {
    if input.any_pressed([KeyCode::AltLeft, KeyCode::AltRight])
        && input.just_pressed(KeyCode::KeyF) {
        for mut v in qvisibility.iter_mut() {
            *v = match *v {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden
            }
        }
    }
}