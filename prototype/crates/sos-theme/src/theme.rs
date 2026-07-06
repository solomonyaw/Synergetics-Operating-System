use egui::{CornerRadius, Stroke, Style, Visuals};

use crate::colors::SosColors;
use crate::spacing::Spacing;

/// Applies the global Synergetics visual theme.
pub fn apply_theme(ctx: &egui::Context) {
    let mut style: Style = (*ctx.style()).clone();

    style.visuals = Visuals::dark();

    style.visuals.panel_fill = SosColors::PANEL;
    style.visuals.window_fill = SosColors::BACKGROUND;
    style.visuals.extreme_bg_color = SosColors::BACKGROUND;
    style.visuals.faint_bg_color = SosColors::PANEL_ALT;

    style.visuals.window_corner_radius =
        CornerRadius::same(Spacing::WINDOW_RADIUS);

    style.visuals.widgets.noninteractive.bg_stroke =
        Stroke::new(1.0, SosColors::BORDER);

    style.visuals.selection.bg_fill = SosColors::ACCENT;

    ctx.set_style(style);
}