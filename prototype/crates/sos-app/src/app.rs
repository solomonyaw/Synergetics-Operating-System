use chrono::Local;
use eframe::egui;

pub struct SynergeticsApp {

    current_time: String,

}

impl SynergeticsApp {

    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {

        Self {

            current_time: String::new(),

        }

    }

}

impl eframe::App for SynergeticsApp {

    fn update(

        &mut self,

        ctx: &egui::Context,

        _frame: &mut eframe::Frame,

    ) {

        self.current_time =
            Local::now().format("%H:%M:%S").to_string();

        // Background
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.visuals_mut().dark_mode = true;

            ui.heading("Synergetics Operating System");

            ui.separator();

            ui.label("Prototype v0.1");

            ui.add_space(10.0);

            ui.label("Desktop Shell");

            ui.add_space(20.0);

            ui.label(format!(
                "Time: {}",
                self.current_time
            ));

        });

    }

}