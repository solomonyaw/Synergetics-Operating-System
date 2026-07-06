#[derive(Default)]
pub struct ShellState {
    pub search_text: String,
    pub notifications: usize,
    pub active_workspace: usize,
    pub wallpaper: String,
}

impl ShellState {
    pub fn new() -> Self {
        Self {
            search_text: String::new(),
            notifications: 3,
            active_workspace: 1,
            wallpaper: String::from("assets/wallpaper.png"),
        }
    }
}