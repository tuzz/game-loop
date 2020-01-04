use std::time::Instant;

pub fn game_loop<U, R>(updates_per_second: u32, mut update: U, mut render: R) -> Control
    where U: FnMut(&mut Control), R: FnMut(&mut Control)
{
    let mut c = Control::new(updates_per_second);

    loop {
        if c.exit_next_iteration {
            break;
        }

        c.current_instant = Instant::now();

        let duration = c.current_instant.duration_since(c.previous_instant);
        let elapsed = duration.as_secs_f64();

        c.running_time += elapsed;
        c.accumulated_time += elapsed;

        while c.accumulated_time >= c.fixed_time_step {
            update(&mut c);

            c.accumulated_time -= c.fixed_time_step;
            c.number_of_updates += 1;
        }

        c.blending_factor = c.accumulated_time / c.fixed_time_step;

        render(&mut c);

        c.number_of_renders += 1;
        c.previous_instant = c.current_instant;
    }

    return c;
}

pub struct Control {
    pub updates_per_second: u32,
    pub exit_next_iteration: bool,

    fixed_time_step: f64,
    number_of_updates: u32,
    number_of_renders: u32,
    running_time: f64,
    accumulated_time: f64,
    blending_factor: f64,
    previous_instant: Instant,
    current_instant: Instant,
}

impl Control {
    pub fn new(updates_per_second: u32) -> Self {
        Self {
            updates_per_second,
            exit_next_iteration: false,

            fixed_time_step: 1.0 / updates_per_second as f64,
            number_of_updates: 0,
            number_of_renders: 0,
            running_time: 0.0,
            accumulated_time: 0.0,
            blending_factor: 0.0,
            previous_instant: Instant::now(),
            current_instant: Instant::now(),
        }
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
