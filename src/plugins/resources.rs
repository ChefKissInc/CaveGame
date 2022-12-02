use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

pub struct GameResourcePlugin;

#[derive(Resource)]
pub struct GameResources {
    pub font: Handle<Font>,
    pub block_textures: Handle<Image>,
}

impl Plugin for GameResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(crate::AppState::Splash, load_assets)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(crate::AppState::Splash)
                    .into(),
            )
            .add_system(
                loading_screen
                    .run_in_state(crate::AppState::Splash)
                    .after(ProgressSystemLabel::Tracking),
            );
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let font = asset_server.load("fonts/Iosevka NF.ttf");
    let block_textures = asset_server.load("textures/blocks/grass_block_top.ktx2");
    loading.add(&font);
    loading.add(&block_textures);
    commands.insert_resource(GameResources {
        font,
        block_textures,
    });
}

fn loading_screen(mut egui_context: ResMut<EguiContext>, counter: Res<ProgressCounter>) {
    egui::Window::new("loading_screen")
        .title_bar(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.heading("ChefKiss Inc");
                ui.add(
                    egui::ProgressBar::new(counter.progress().into())
                        .animate(true)
                        .show_percentage(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    ui.label(format!(
                        "Loading {}/{}",
                        counter.progress().done,
                        counter.progress().total
                    ));
                });
            });
        });
}
