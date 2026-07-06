use crate::{
    desktop,
    dock,
    sidebar,
    state::ShellState,
    topbar,
};

pub struct DesktopShell {
    pub state: ShellState,
}

impl DesktopShell {
    pub fn new() -> Self {
        Self {
            state: ShellState::new(),
        }
    }

    pub fn ui(
        &mut self,
        ctx: &egui::Context,
    ) {

        topbar::show(ctx, &mut self.state);

        sidebar::show(ctx);

        dock::show(ctx);

        desktop::show(ctx);
    }
}