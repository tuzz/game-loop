use wasm_bindgen::prelude::*;
use game_loop::game_loop;

#[wasm_bindgen(start)]
pub fn main() {
    let game = Game::new();

    game_loop(game, 240, 0.1, |g| {
        g.game.your_update_function();
    }, |g| {
        g.game.your_render_function();
    });
}

struct Game {
    span: web_sys::Element,
    counter: u32,
}

impl Game {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let span = document.create_element("span").unwrap();

        body.append_child(&span).unwrap();

        Self { span, counter: 0 }
    }

    pub fn your_update_function(&mut self) {
        self.counter += 1;
    }

    pub fn your_render_function(&self) {
        self.span.set_inner_html(&format!("Counter: {}", self.counter));
    }
}
