use crate::*;

pub use helper::*;

#[cfg(not(target_arch = "wasm32"))]
mod helper {
    use super::*;

    pub fn game_loop<G, U, R>(game: G, updates_per_second: u32, max_frame_time: f64, mut update: U, mut render: R) -> GameLoop<G, Time>
        where U: FnMut(&mut GameLoop<G, Time>),
              R: FnMut(&mut GameLoop<G, Time>),
    {
        let mut game_loop = GameLoop::new(game, updates_per_second, max_frame_time);

        while game_loop.next_frame(&mut update, &mut render) {}

        game_loop
    }
}

#[cfg(target_arch = "wasm32")]
mod helper {
    use super::*;
    use web_sys::window;

    pub fn game_loop<G, U, R>(game: G, updates_per_second: u32, max_frame_time: f64, update: U, render: R)
        where G: 'static,
              U: FnMut(&mut GameLoop<G, Time>) + 'static,
              R: FnMut(&mut GameLoop<G, Time>) + 'static,
    {
        let game_loop = GameLoop::new(game, updates_per_second, max_frame_time);

        animation_frame(game_loop, update, render);
    }

    fn animation_frame<G, U, R>(mut g: GameLoop<G, Time>, mut update: U, mut render: R)
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
}
