use egui::*;

pub fn show(ctx: &Context) {

    CentralPanel::default().show(ctx, |ui| {

        ui.vertical_centered(|ui| {

            ui.add_space(80.0);

            ui.heading("Synergetics Operating System");

            ui.label("Prototype v0.1");

            ui.separator();

            ui.label("Topology-based Operating System");

            ui.add_space(20.0);

            ui.group(|ui| {

                ui.heading("System Coherence");

                ui.add(
                    ProgressBar::new(0.86)
                        .show_percentage()
                );

                ui.separator();

                ui.label("Topology Nodes : 124");

                ui.label("Emergence Score : 91%");

                ui.label("Optimization : Active");

            });

        });

    });

}