use bevy::prelude::*;

use crate::MainState;

#[derive(Default)]
pub struct MainMenuPlugin<T: States> {
    mystate: T,
}

impl<T: States+Copy> MainMenuPlugin<T> {
    pub fn for_state(state: T) -> Self {
        Self {
            mystate: state
        }
    }
}

impl<T: States+Copy> Plugin for MainMenuPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(self.mystate), mmenu_setup)
            .insert_resource(MMenuStyles {
                item_style: TextStyle {
                    font: Default::default(),
                    font_size: 45.0,
                    color: Color::rgb_u8(81, 130, 155),
                },
                hover_color: Color::rgba_u8(155, 176, 193, 96),
            })
            .add_systems(Update, mmenu_highlight.run_if(in_state(self.mystate)))
            .add_systems(Update, mmenu_invaders::<T>.run_if(in_state(self.mystate)))
            .add_systems(OnExit(self.mystate), mmenu_onexit)
        ;

        #[cfg(not(target_family = "wasm"))]
        app.add_systems(Update, mmenu_btn_quit.run_if(in_state(self.mystate)));
    }
}

#[derive(Component)]
struct MMenuItem;

#[derive(Component)]
struct MMenuItemInvaders;

#[derive(Component)]
struct MMenuItemQuit;


#[derive(Resource)]
struct MMenuStyles {
    item_style: TextStyle,
    hover_color: Color,
}

#[derive(Component)]
struct MMenuHoverText(String);

#[derive(Component)]
struct MMenuInfo;


fn mmenu_setup(mut commands: Commands, mut defaults: ResMut<MMenuStyles>, assets: Res<AssetServer>) {
    defaults.item_style.font = assets.load("eight-bit-dragon.otf");

    #[cfg(not(target_family = "wasm"))]
    commands.spawn((
        TextBundle {
            text: Text::from_section("Quit", defaults.item_style.clone()),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(10.0),
                top: Val::Percent(80.0),
                ..default()
            },
            ..default()
        },
        Interaction::default(),
        MMenuItem,
        MMenuItemQuit,
        MMenuHoverText("Quit Game".into()),
    ));
    commands.spawn((
        TextBundle {
            text: Text::from_section("Invaders", defaults.item_style.clone()),
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(10.0),
                top: Val::Percent(10.0),
                ..default()
            },
            ..default()
        },
        Interaction::default(),
        MMenuItem,
        MMenuItemInvaders,
        MMenuHoverText("Spave Invaders game.\n\nControls:\nA and D to move, LMB to shoot, Esc for pause.".into()),
    ));
    let mut info_style = defaults.item_style.clone();
    info_style.font_size = 30.0;
    commands.spawn((
        TextBundle {
            text: Text::from_section("", info_style),
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Percent(3.0),
                left: Val::Percent(50.0),
                top: Val::Percent(10.0),
                ..default()
            },
            ..default()
        },
        Interaction::default(),
        MMenuItem,
        MMenuInfo,
    ));
}

fn mmenu_invaders<T: States>(
    q: Query<&Interaction, (With<MMenuItemInvaders>, Changed<Interaction>)>,
    mut stchange: ResMut<NextState<MainState>>
) {
    for int in q.iter() {
        // println!("state: {:?}, int {:?}", st, int);
        if int == &Interaction::Pressed {
            stchange.set(MainState::Invaders)
        }
    }
}

fn mmenu_highlight(
    mut q: Query<(&Interaction, &mut BackgroundColor, &MMenuHoverText), (With<MMenuItem>, Changed<Interaction>)>,
    mut qinfo: Query<&mut Text, With<MMenuInfo>>,
    defaults: Res<MMenuStyles>
) {
    let mut info_text = qinfo.single_mut();
    for (int, mut bgcolor, hover_text) in q.iter_mut() {
        // println!("interaction {:?} text {:?}", int, txt);
        match int {
            Interaction::Hovered => {*bgcolor = BackgroundColor(defaults.hover_color); info_text.sections[0].value = hover_text.0.clone();},
            Interaction::Pressed => (),
            Interaction::None => {*bgcolor = BackgroundColor(Color::NONE); info_text.sections[0].value = "".into();},
        }
    }
}

#[cfg(not(target_family = "wasm"))]
fn mmenu_btn_quit(
    q: Query<&Interaction, (With<MMenuItemQuit>, Changed<Interaction>)>,
    mut exit_event: ResMut<Events<bevy::app::AppExit>>,
) {
    for int in q.iter() {
        match int {
            Interaction::Pressed => {println!("bye!");exit_event.send(bevy::app::AppExit);}
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn mmenu_onexit(mut commands: Commands, mut q: Query<Entity, With<MMenuItem>>) {
    // println!("mmenu onexit");
    for entity in q.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}