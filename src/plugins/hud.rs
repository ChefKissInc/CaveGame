use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use iyes_loopless::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_enter_system(crate::AppState::InGame, hud_setup)
            .add_system(hud_system.run_in_state(crate::AppState::InGame));
    }
}

#[derive(Component)]
struct FpsText;

fn hud_setup(mut commands: Commands, res: Res<super::resources::GameResources>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "FPS: ",
                    TextStyle {
                        font: res.font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: res.font.clone(),
                    font_size: 20.0,
                    color: Color::GOLD,
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexEnd,
                margin: UiRect::all(Val::Percent(0.5)),
                ..default()
            }),
        )
        .insert(FpsText);

    commands.spawn_bundle(
        TextBundle::from_section(
            "+",
            TextStyle {
                font: res.font.clone(),
                font_size: 40.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::CENTER)
        .with_style(Style {
            align_self: AlignSelf::Center,
            margin: UiRect::all(Val::Auto),
            ..default()
        }),
    );
}

fn hud_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{}", average.ceil());
            }
        }
    }
}
