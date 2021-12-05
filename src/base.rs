use crate::*;

pub struct GameLoop<G, T: TimeTrait, W> {
    pub game: G,
    pub updates_per_second: u32,
    pub max_frame_time: f64,
    pub exit_next_iteration: bool,
    pub window: W,

    fixed_time_step: f64,
    number_of_updates: u32,
    number_of_renders: u32,
    last_frame_time: f64,
    running_time: f64,
    accumulated_time: f64,
    blending_factor: f64,
    previous_instant: T,
    current_instant: T,
}

impl<G, T: TimeTrait, W> GameLoop<G, T, W> {
    pub fn new(game: G, updates_per_second: u32, max_frame_time: f64, window: W) -> Self {
        Self {
            game,
            updates_per_second,
            max_frame_time,
            window,
            exit_next_iteration: false,

            fixed_time_step: 1.0 / updates_per_second as f64,
            number_of_updates: 0,
            number_of_renders: 0,
            running_time: 0.0,
            accumulated_time: 0.0,
            blending_factor: 0.0,
            previous_instant: T::now(),
            current_instant: T::now(),
            last_frame_time: 0.0,
        }
    }

    pub fn next_frame<U, R>(&mut self, mut update: U, mut render: R) -> bool
        where U: FnMut(&mut GameLoop<G, T, W>),
              R: FnMut(&mut GameLoop<G, T, W>),
    {
        let mut g = self;

        if g.exit_next_iteration {
            return false;
        }

        g.current_instant = T::now();

        let mut elapsed = g.current_instant.sub(&g.previous_instant);

        if elapsed > g.max_frame_time {
            elapsed = g.max_frame_time;
        }

        g.last_frame_time = elapsed;
        g.running_time += elapsed;
        g.accumulated_time += elapsed;

        while g.accumulated_time >= g.fixed_time_step {
            update(&mut g);

            g.accumulated_time -= g.fixed_time_step;
            g.number_of_updates += 1;
        }

        g.blending_factor = g.accumulated_time / g.fixed_time_step;

        render(&mut g);

        g.number_of_renders += 1;
        g.previous_instant = g.current_instant;

        return true;
    }

    pub fn re_accumulate(&mut self) {
        let mut g = self;

        g.current_instant = T::now();

        let prev_elapsed = g.last_frame_time;
        let new_elapsed = g.current_instant.sub(&g.previous_instant);

        let delta = new_elapsed - prev_elapsed;

        // We don't update g.last_frame_time since this additional time in the
        // render function is considered part of the current frame.

        g.running_time += delta;
        g.accumulated_time += delta;

        g.blending_factor = g.accumulated_time / g.fixed_time_step;
    }

    pub fn exit(&mut self) {
        self.exit_next_iteration = true;
    }

    pub fn set_updates_per_second(&mut self, new_updates_per_second: u32) {
        self.updates_per_second = new_updates_per_second;
        self.fixed_time_step = 1.0 / new_updates_per_second as f64;
    }

    pub fn fixed_time_step(&self) -> f64 {
        self.fixed_time_step
    }

    pub fn number_of_updates(&self) -> u32 {
        self.number_of_updates
    }

    pub fn number_of_renders(&self) -> u32 {
        self.number_of_renders
    }

    pub fn last_frame_time(&self) -> f64 {
        self.last_frame_time
    }

    pub fn running_time(&self) -> f64 {
        self.running_time
    }

    pub fn accumulated_time(&self) -> f64 {
        self.accumulated_time
    }

    pub fn blending_factor(&self) -> f64 {
        self.blending_factor
    }

    pub fn previous_instant(&self) -> T {
        self.previous_instant
    }

    pub fn current_instant(&self) -> T {
        self.current_instant
    }
}
