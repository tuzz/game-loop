use wasm_bindgen::{prelude::*, JsCast};
use web_sys::window;
use super::*;

pub fn game_loop<G, U, R>(game: G, updates_per_second: u32, max_frame_time: f64, update: U, render: R)
    where G: 'static,
          U: FnMut(&mut GameLoop<G, Time>) + 'static,
          R: FnMut(&mut GameLoop<G, Time>) + 'static,
{
    let game_loop = GameLoop::new(game, updates_per_second, max_frame_time);

    animation_frame(game_loop, update, render);
}

pub fn animation_frame<G, U, R>(mut g: GameLoop<G, Time>, mut update: U, mut render: R)
    where G: 'static,
          U: FnMut(&mut GameLoop<G, Time>) + 'static,
          R: FnMut(&mut GameLoop<G, Time>) + 'static,
{
    if g.next_frame(&mut update, &mut render) {
        let next_frame = move || animation_frame(g, update, render);
        let closure = Closure::once_into_js(next_frame);
        let js_func = closure.as_ref().unchecked_ref();

        window().unwrap().request_animation_frame(js_func).unwrap();
    }
}

#[derive(Copy, Clone)]
pub struct Time(f64);

impl TimeTrait for Time {
    fn now() -> Self {
        Self(window().unwrap().performance().unwrap().now())
    }

    fn sub(&self, other: &Self) -> f64 {
        (self.0 - other.0) / 1000.0
    }
}
