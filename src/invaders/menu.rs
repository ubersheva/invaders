use bevy::prelude::*;
use crate::invaders::{InvadersGame, InvaderState};
use crate::MainState;

#[derive(Component)]
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EventClose>()

            .add_systems(OnEnter(InvaderState::Menu), show_menu)
            .add_systems(OnExit(InvaderState::Menu), destroy_menu)
            .add_systems(Update, interact_key.run_if(in_state(InvaderState::Menu)))
            .add_systems(Update, interact_menu.run_if(in_state(InvaderState::Menu)))
            .add_systems(Update, hover_menu.run_if(in_state(InvaderState::Menu)))
            .add_systems(Update, close_menu.run_if(in_state(InvaderState::Menu)))
        ;
    }
}

#[derive(Component)]
struct MMenu;

#[derive(Component)]
struct MMenuQuit;

#[derive(Component)]
struct MMenuClose;

#[derive(Event)]
struct EventClose;

fn show_menu(
    mut commands: Commands,
    game: Res<InvadersGame>,
    assets: Res<AssetServer>
) {
    let Some(font) = assets.get_handle("eight-bit-dragon.otf") else {
        error!("menu font not loaded");
        return;
    };
    let color = Color::rgb_u8(81, 130, 155);
    commands.spawn((
        NodeBundle {
            style: Style {
                left: Val::Percent(0.0),
                top: Val::Percent(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: BackgroundColor(Color::rgba_u8(234, 223, 180, 172)),
            ..default()
        },
        MMenu,
        ));
    commands.spawn((
        TextBundle {
            text: Text::from_section("Quit", TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color,
            }),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(70.0),
                top: Val::Percent(90.0),
                ..default()
            },
            ..default()
        },
        Interaction::default(),
        MMenu,
        MMenuQuit,
    ));
    if !game.win && !game.gameover {
        commands.spawn((
            TextBundle {
                text: Text::from_section("Close", TextStyle {
                    font: font.clone(),
                    font_size: 30.0,
                    color,
                }),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(20.0),
                    top: Val::Percent(90.0),
                    ..default()
                },
                ..default()
            },
            Interaction::default(),
            MMenu,
            MMenuClose,
        ));
    }
    if game.win {
        commands.spawn((
            TextBundle {
                text: Text::from_section("You Win!", TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::rgb_u8(211, 118, 118),
                }),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(35.0),
                    top: Val::Percent(20.0),
                    ..default()
                },
                ..default()
            },
            MMenu,
        ));
    } else if game.gameover {
        commands.spawn((
            TextBundle {
                text: Text::from_section("Game Over!", TextStyle {
                    font: font.clone(),
                    font_size: 50.0,
                    color: Color::rgb_u8(211, 118, 118),
                }),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(35.0),
                    top: Val::Percent(20.0),
                    ..default()
                },
                ..default()
            },
            MMenu,
        ));
    }
    commands.spawn((
        TextBundle {
            text: Text::from_section(format!("Score: {}", game.score), TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color,
            }),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(40.0),
                top: Val::Percent(30.0),
                ..default()
            },
            ..default()
        },
        MMenu,
    ));
    commands.spawn((
        TextBundle {
            text: Text::from_section(format!("Time: {:.1}", game.time), TextStyle {
                font: font.clone(),
                font_size: 30.0,
                color,
            }),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(40.0),
                top: Val::Percent(35.0),
                ..default()
            },
            ..default()
        },
        MMenu,
    ));
}

fn interact_key(
    input: Res<ButtonInput<KeyCode>>,
    mut event_close: EventWriter<EventClose>,
) {
    if input.just_pressed(KeyCode::Escape) {
        event_close.send(EventClose);
    }
}

fn interact_menu(
    qclose: Query<&Interaction, (With<MMenuClose>, Changed<Interaction>)>,
    qquit: Query<&Interaction, (With<MMenuQuit>, Changed<Interaction>)>,
    mut event_close: EventWriter<EventClose>,
    mut main_state: ResMut<NextState<MainState>>,
    mut state: ResMut<NextState<InvaderState>>
) {
    if let Ok(Interaction::Pressed) = qclose.get_single() {
        event_close.send(EventClose);
    }
    if let Ok(Interaction::Pressed) = qquit.get_single() {
        state.set(InvaderState::None);
        main_state.set(MainState::MainMenu);
    }
}

fn close_menu(
    event: EventReader<EventClose>,
    mut state: ResMut<NextState<InvaderState>>,
) {
    if !event.is_empty() {
        state.set(InvaderState::Game);
    }
}

fn destroy_menu(
    mut commands: Commands,
    qmenu: Query<Entity, With<MMenu>>,
) {
    for e in qmenu.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn hover_menu(
    mut qmenu: Query<(&Interaction, &mut BackgroundColor), (With<MMenu>, Changed<Interaction>)>
) {
    for (int, mut color) in qmenu.iter_mut() {
        match int {
            Interaction::Hovered => { *color = BackgroundColor(Color::rgba_u8(155, 176, 193, 96)); }
            Interaction::Pressed => {}
            Interaction::None => { *color = BackgroundColor(Color::NONE); }
        }
    }

}