use crate::*;

pub use helper::*;

#[cfg(all(
    not(target_arch = "wasm32"),
    not(feature = "winit"),
    not(feature = "tao")
))]
mod helper {
    use super::*;

    pub fn game_loop<G, U, R>(
        game: G,
        updates_per_second: u32,
        max_frame_time: f64,
        mut update: U,
        mut render: R,
    ) -> GameLoop<G, Time, ()>
    where
        U: FnMut(&mut GameLoop<G, Time, ()>),
        R: FnMut(&mut GameLoop<G, Time, ()>),
    {
        let mut game_loop = GameLoop::new(game, updates_per_second, max_frame_time, ());

        while game_loop.next_frame(&mut update, &mut render) {}

        game_loop
    }
}

#[cfg(all(target_arch = "wasm32", not(feature = "winit")))]
mod helper {
    use super::*;
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;
    use web_sys::window;

    pub fn game_loop<G, U, R>(
        game: G,
        updates_per_second: u32,
        max_frame_time: f64,
        update: U,
        render: R,
    ) where
        G: 'static,
        U: FnMut(&mut GameLoop<G, Time, ()>) + 'static,
        R: FnMut(&mut GameLoop<G, Time, ()>) + 'static,
    {
        let game_loop = GameLoop::new(game, updates_per_second, max_frame_time, ());

        animation_frame(game_loop, update, render);
    }

    fn animation_frame<G, U, R>(mut g: GameLoop<G, Time, ()>, mut update: U, mut render: R)
    where
        G: 'static,
        U: FnMut(&mut GameLoop<G, Time, ()>) + 'static,
        R: FnMut(&mut GameLoop<G, Time, ()>) + 'static,
    {
        if g.next_frame(&mut update, &mut render) {
            let next_frame = move || animation_frame(g, update, render);
            let closure = Closure::once_into_js(next_frame);
            let js_func = closure.as_ref().unchecked_ref();

            window().unwrap().request_animation_frame(js_func).unwrap();
        }
    }
}

#[cfg(feature = "winit")]
mod helper {
    use super::*;
    use std::sync::Arc;
    use winit::event::{Event, WindowEvent};
    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::window::Window;

    pub use winit;

    pub fn game_loop<G, U, R, H, T>(
        event_loop: EventLoop<T>,
        window: Arc<Window>,
        game: G,
        updates_per_second: u32,
        max_frame_time: f64,
        mut update: U,
        mut render: R,
        mut handler: H,
    ) -> Result<(), impl std::error::Error>
    where
        G: 'static,
        U: FnMut(&mut GameLoop<G, Time, Arc<Window>>) + 'static,
        R: FnMut(&mut GameLoop<G, Time, Arc<Window>>) + 'static,
        H: FnMut(&mut GameLoop<G, Time, Arc<Window>>, &Event<T>) + 'static,
        T: 'static,
    {
        let mut game_loop = GameLoop::new(game, updates_per_second, max_frame_time, window.clone());

        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            // Forward events to existing handlers.
            handler(&mut game_loop, &event);

            match event {
                Event::WindowEvent { event, window_id } if window_id == window.id() => {
                    match event {
                        WindowEvent::Occluded(occluded) => game_loop.window_occluded = occluded,
                        WindowEvent::RedrawRequested => {
                            if !game_loop.next_frame(&mut update, &mut render) {
                                elwt.exit();
                            }
                        }
                        _ => (),
                    }
                }
                Event::AboutToWait => {
                    game_loop.window.request_redraw();
                }
                _ => (),
            }
        })
    }
}

#[cfg(feature = "tao")]
mod helper {
    use super::*;
    use std::sync::Arc;
    use tao::event::Event;
    use tao::event_loop::{ControlFlow, EventLoop};
    use tao::window::Window;

    pub use tao;

    pub fn game_loop<G, U, R, H, T>(
        event_loop: EventLoop<T>,
        window: Arc<Window>,
        game: G,
        updates_per_second: u32,
        max_frame_time: f64,
        mut update: U,
        mut render: R,
        mut handler: H,
    ) -> !
    where
        G: 'static,
        U: FnMut(&mut GameLoop<G, Time, Arc<Window>>) + 'static,
        R: FnMut(&mut GameLoop<G, Time, Arc<Window>>) + 'static,
        H: FnMut(&mut GameLoop<G, Time, Arc<Window>>, &Event<'_, T>) + 'static,
        T: 'static,
    {
        let mut game_loop = GameLoop::new(game, updates_per_second, max_frame_time, window);

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            // Forward events to existing handlers.
            handler(&mut game_loop, &event);

            match event {
                Event::RedrawRequested(_) => {
                    if !game_loop.next_frame(&mut update, &mut render) {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                Event::MainEventsCleared => {
                    game_loop.window.request_redraw();
                }
                _ => {}
            }
        })
    }
}
