use bevy::prelude::*;

const TEXT_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const BUTTON_BG_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const BUTTON_HOVER_BG_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
struct PlayPauseButton;

#[derive(Component)]
struct NoiseButton;

#[derive(Component)]
struct ClearButton;

#[derive(Resource, Debug)]
pub struct Config {
    pub playing: bool,
}

pub struct AddNoiseEvent;

pub struct ClearEvent;

pub struct MenuPlugin {}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(Config { playing: false })
            .add_event::<AddNoiseEvent>()
            .add_event::<ClearEvent>()
            .add_system(play_pause_system)
            .add_system(add_noise_button_system)
            .add_system(clear_button_system);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/agave-r.ttf");
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(80.0), Val::Px(30.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BUTTON_BG_COLOR.into(),
                ..default()
            },
            PlayPauseButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: TEXT_COLOR,
                },
            ));
        });

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(80.0), Val::Px(30.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10.0),
                        top: Val::Px(50.0),
                        ..default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BUTTON_BG_COLOR.into(),
                ..default()
            },
            NoiseButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Noise",
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: TEXT_COLOR,
                },
            ));
        });

    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(80.0), Val::Px(30.0)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(10.0),
                        top: Val::Px(90.0),
                        ..default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BUTTON_BG_COLOR.into(),
                ..default()
            },
            ClearButton,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Clear",
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: TEXT_COLOR,
                },
            ));
        });
}

fn play_pause_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<PlayPauseButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut config: ResMut<Config>,
) {
    for (interaction, mut color, children) in &mut query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                let currently_playing = config.playing;
                if currently_playing {
                    text.sections[0].value = "Play".to_string();
                    *config = Config { playing: false };
                } else {
                    text.sections[0].value = "Pause".to_string();
                    *config = Config { playing: true };
                }
            }
            Interaction::Hovered => *color = BUTTON_HOVER_BG_COLOR.into(),
            Interaction::None => *color = BUTTON_BG_COLOR.into(),
        }
    }
}

fn add_noise_button_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NoiseButton>),
    >,
    mut ev_add_noise: EventWriter<AddNoiseEvent>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Clicked => {
                ev_add_noise.send(AddNoiseEvent);
            }
            Interaction::Hovered => *color = BUTTON_HOVER_BG_COLOR.into(),
            Interaction::None => *color = BUTTON_BG_COLOR.into(),
        }
    }
}

fn clear_button_system(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ClearButton>),
    >,
    mut ev_clear: EventWriter<ClearEvent>,
) {
    for (interaction, mut color) in &mut query {
        match *interaction {
            Interaction::Clicked => {
                ev_clear.send(ClearEvent);
            }
            Interaction::Hovered => *color = BUTTON_HOVER_BG_COLOR.into(),
            Interaction::None => *color = BUTTON_BG_COLOR.into(),
        }
    }
}
