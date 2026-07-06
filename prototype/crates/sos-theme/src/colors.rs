use egui::Color32;

/// Global color palette for the Synergetics desktop.
pub struct SosColors;

impl SosColors {
    pub const BACKGROUND: Color32 = Color32::from_rgb(18, 22, 30);
    pub const PANEL: Color32 = Color32::from_rgb(28, 34, 44);
    pub const PANEL_ALT: Color32 = Color32::from_rgb(36, 43, 54);

    pub const ACCENT: Color32 = Color32::from_rgb(0, 180, 255);
    pub const SUCCESS: Color32 = Color32::from_rgb(70, 210, 120);
    pub const WARNING: Color32 = Color32::from_rgb(255, 185, 70);
    pub const ERROR: Color32 = Color32::from_rgb(220, 70, 70);

    pub const TEXT: Color32 = Color32::from_rgb(240, 240, 240);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(170, 170, 170);

    pub const BORDER: Color32 = Color32::from_rgb(65, 75, 90);
}