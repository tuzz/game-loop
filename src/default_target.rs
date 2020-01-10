use std::time::Instant;
use super::*;

pub fn game_loop<G, U, R>(game: G, updates_per_second: u32, max_frame_time: f64, mut update: U, mut render: R) -> GameLoop<G, Time>
    where U: FnMut(&mut GameLoop<G, Time>),
          R: FnMut(&mut GameLoop<G, Time>),
{
    let mut game_loop = GameLoop::new(game, updates_per_second, max_frame_time);

    while game_loop.next_frame(&mut update, &mut render) {}

    return game_loop;
}

#[derive(Copy, Clone)]
pub struct Time(Instant);

impl TimeTrait for Time {
    fn now() -> Self {
        Self(Instant::now())
    }

    fn sub(&self, other: &Self) -> f64 {
        self.0.duration_since(other.0).as_secs_f64()
    }
}
