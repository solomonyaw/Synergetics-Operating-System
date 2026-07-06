use egui::*;

pub fn show(ctx: &Context) {

    SidePanel::left("sidebar")
        .default_width(90.0)
        .show(ctx, |ui| {

            ui.heading("Apps");

            ui.separator();

            ui.button("Files");

            ui.button("Terminal");

            ui.button("Topology");

            ui.button("Settings");

            ui.button("AI");

        });

}