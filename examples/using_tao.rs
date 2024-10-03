use game_loop::game_loop;

// For convenience, game_loop re-exports tao so you don't need to add it as
// an additional dependency of your crate.

use game_loop::tao::event::{Event, WindowEvent};
use game_loop::tao::event_loop::EventLoop;
use game_loop::tao::window::{Window, WindowBuilder};
use muda::{Menu, MenuEvent, Submenu};
use std::sync::Arc;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let menu = Menu::new();
    #[cfg(target_os = "windows")]
    {
        use game_loop::tao::platform::windows::WindowExtWindows as _;
        let file_menu = Submenu::new("&File", true);
        menu.append(&file_menu).unwrap();
        file_menu.append(&muda::PredefinedMenuItem::quit(None)).unwrap();
        menu.init_for_hwnd(window.hwnd() as _).unwrap();
    }
    #[cfg(target_os = "linux")]
    {
        use game_loop::tao::platform::unix::WindowExtUnix as _;
        let file_menu = Submenu::new("File", true);
        menu.append(&file_menu).unwrap();
        file_menu.append(&MenuItem::with_id("quit", "Quit", true, None)).unwrap();
        menu.init_for_gtk_window(window.gtk_window(), window.default_vbox()).unwrap();
    }
    #[cfg(target_os = "macos")]
    {
        let app_menu = Submenu::new("App", true);
        menu.append(&app_menu).unwrap();
        app_menu.append(&muda::PredefinedMenuItem::quit(None)).unwrap();
        menu.init_for_nsapp();
    }

    let window = Arc::new(window);

    let game = Game::new();

    game_loop(event_loop, window, game, 240, 0.1, |g| {
        g.game.your_update_function();
    }, |g| {
        g.game.your_render_function(&g.window);
    }, |g, event| {
        if !g.game.your_window_handler(event) { g.exit(); }
    });
}

#[derive(Default)]
struct Game {
    num_updates: u32,
    num_renders: u32,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn your_update_function(&mut self) {
        self.num_updates += 1;
    }

    pub fn your_render_function(&mut self, window: &Window) {
        self.num_renders += 1;
        window.set_title(&format!("num_updates: {}, num_renders: {}", self.num_updates, self.num_renders));
    }

    // A very simple handler that returns false when CloseRequested is detected.
    pub fn your_window_handler(&self, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    return false;
                }
                _ => {}
            },
            _ => {}
        }

        if let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id.0 == "quit" {
                return false;
            }
        }

        true
    }
}
