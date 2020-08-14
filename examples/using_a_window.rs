use game_loop::game_loop;

// For convenience, game_loop re-exports winit so you don't need to add it as
// an additional dependency of your crate. It uses a slightly older version than
// latest since I couldn't get the latest version to compile on my mac.

use game_loop::winit::event::{Event, WindowEvent};
use game_loop::winit::event_loop::{EventLoop, ControlFlow};
use game_loop::winit::window::{Window, WindowBuilder};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let game = Game::new();

    let g = game_loop(event_loop, window, game, 240, 0.1, |g| {
        g.game.your_update_function();
    }, |g| {
        g.game.your_render_function(&g.window);
    }, |g, event| {
        if !g.game.your_window_handler(event) { g.exit(); }
    });
}

#[derive(Default)]
struct Game {
    counter: u32,
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn your_update_function(&mut self) {
        self.counter += 1;
    }

    pub fn your_render_function(&self, window: &Window) {
        window.set_title(&format!("Counter: {}", self.counter));
    }

    // A very simple handler that returns false when CloseRequested is detected.
    pub fn your_window_handler(&self, event: Event<()>) -> bool {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    return false;
                },
                _ => {},
            },
            _ => {},
        }

        true
    }
}
