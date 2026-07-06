use egui::*;
use crate::state::ShellState;

pub fn show(
    ctx: &Context,
    state: &mut ShellState,
) {
    TopBottomPanel::top("topbar").show(ctx, |ui| {

        ui.horizontal(|ui| {

            ui.heading("SOS");

            ui.separator();

            ui.label(format!(
                "Workspace {}",
                state.active_workspace
            ));

            ui.separator();

            ui.label("Search");

        });

    });
}