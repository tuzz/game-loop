#[cfg(not(target_arch = "wasm32"))] mod default_target;
#[cfg(not(target_arch = "wasm32"))] pub use default_target::game_loop;

#[cfg(target_arch = "wasm32")] mod wasm_target;
#[cfg(target_arch = "wasm32")] pub use wasm_target::game_loop;

pub struct GameLoop<G, T: TimeTrait> {
    pub game: G,
    pub updates_per_second: u32,
    pub exit_next_iteration: bool,

    fixed_time_step: f64,
    number_of_updates: u32,
    number_of_renders: u32,
    running_time: f64,
    accumulated_time: f64,
    blending_factor: f64,
    previous_instant: T,
    current_instant: T,
}

impl<G, T: TimeTrait> GameLoop<G, T> {
    pub fn new(game: G, updates_per_second: u32) -> Self {
        Self {
            game,
            updates_per_second,
            exit_next_iteration: false,

            fixed_time_step: 1.0 / updates_per_second as f64,
            number_of_updates: 0,
            number_of_renders: 0,
            running_time: 0.0,
            accumulated_time: 0.0,
            blending_factor: 0.0,
            previous_instant: T::now(),
            current_instant: T::now(),
        }
    }

    pub fn next_frame<U, R>(&mut self, mut update: U, mut render: R) -> bool
        where U: FnMut(&mut GameLoop<G, T>),
              R: FnMut(&mut GameLoop<G, T>),
    {
        let mut g = self;

        if g.exit_next_iteration {
            return false;
        }

        g.current_instant = T::now();

        let elapsed = g.current_instant.sub(&g.previous_instant);

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

    pub fn exit(&mut self) {
        self.exit_next_iteration = true;
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

    pub fn running_time(&self) -> f64 {
        self.running_time
    }

    pub fn accumulated_time(&self) -> f64 {
        self.accumulated_time
    }

    pub fn blending_factor(&self) -> f64 {
        self.blending_factor
    }
}

pub trait TimeTrait : Copy {
    fn now() -> Self;
    fn sub(&self, other: &Self) -> f64;
}
