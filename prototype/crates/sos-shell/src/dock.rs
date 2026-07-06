use egui::*;

pub fn show(ctx: &Context) {

    TopBottomPanel::bottom("dock")
        .show(ctx, |ui| {

            ui.horizontal_centered(|ui| {

                ui.button("📁");

                ui.button("🖥");

                ui.button("🧠");

                ui.button("⚙");

                ui.button("📊");

            });

        });

}