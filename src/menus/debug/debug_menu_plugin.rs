use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_inspector_egui::bevy_egui::{EguiContext, EguiPlugin};
use bevy_inspector_egui::bevy_inspector::hierarchy::SelectedEntities;
use bevy_inspector_egui::quick::{self};
use bevy_inspector_egui::{self, quick::StateInspectorPlugin};
use bevy_inspector_egui::{egui, DefaultInspectorConfigPlugin};

use crate::app::states::ClientAppState;
use crate::net::states::{ClientGameState, ServerConnectionState};

#[derive(Component)]
pub struct CustomInspectorPlugin;

impl Plugin for CustomInspectorPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(StateInspectorPlugin::<ClientAppState>::default())
        //     .add_plugins(StateInspectorPlugin::<ClientGameState>::default())
        //     .add_plugins(StateInspectorPlugin::<ServerConnectionState>::default())
        //     .add_plugins(quick::WorldInspectorPlugin::new());
        app.add_plugins(EguiPlugin)
            .add_plugins(DefaultInspectorConfigPlugin)
            .add_systems(
                Update,
                inspector_ui.run_if(input_toggle_active(true, KeyCode::Escape)),
            );
    }
}

fn inspector_ui(world: &mut World, mut selected_entities: Local<SelectedEntities>) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();
    egui::SidePanel::left("hierarchy")
        .default_width(200.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Hierarchy");

                bevy_inspector_egui::bevy_inspector::hierarchy::hierarchy_ui(
                    world,
                    ui,
                    &mut selected_entities,
                );

                ui.label("Press escape to toggle UI");
                ui.allocate_space(ui.available_size());
            });
        });

    egui::TopBottomPanel::bottom("states")
        .default_height(200.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("States");

                egui::CollapsingHeader::new("ClientAppState").show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_state::<ClientAppState>(world, ui);
                });

                egui::CollapsingHeader::new("ClientGameState").show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_state::<ClientGameState>(world, ui);
                });
                egui::CollapsingHeader::new("ServerConnectionState").show(ui, |ui| {
                    bevy_inspector_egui::bevy_inspector::ui_for_state::<ServerConnectionState>(
                        world, ui,
                    );
                });

                ui.allocate_space(ui.available_size());
            })
        });

    egui::SidePanel::right("inspector")
        .default_width(250.0)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.heading("Inspector");

                match selected_entities.as_slice() {
                    &[entity] => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entity(world, entity, ui);
                    }
                    entities => {
                        bevy_inspector_egui::bevy_inspector::ui_for_entities_shared_components(
                            world, entities, ui,
                        );
                    }
                }

                ui.allocate_space(ui.available_size());
            });
        });
}
