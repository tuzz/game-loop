pub trait TimeTrait : Copy {
    fn now() -> Self;
    fn sub(&self, other: &Self) -> f64;
    fn supports_sleep() -> bool;
    fn sleep(seconds: f64);
}

pub use time::*;

#[cfg(not(target_arch = "wasm32"))]
mod time {
    use super::*;
    use std::time::{Instant, Duration};
    use std::thread::sleep;

    #[derive(Copy, Clone)]
    pub struct Time(Instant);

    impl TimeTrait for Time {
        fn now() -> Self {
            Self(Instant::now())
        }

        fn sub(&self, other: &Self) -> f64 {
            self.0.duration_since(other.0).as_secs_f64()
        }

        fn supports_sleep() -> bool {
            true
        }

        fn sleep(seconds: f64) {
            sleep(Duration::from_secs_f64(seconds));
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod time {
    use super::*;
    use web_sys::window;

    #[derive(Copy, Clone)]
    pub struct Time(f64);

    impl TimeTrait for Time {
        fn now() -> Self {
            Self(window().unwrap().performance().unwrap().now() / 1000.)
        }

        fn sub(&self, other: &Self) -> f64 {
            self.0 - other.0
        }

        fn supports_sleep() -> bool {
            false
        }

        fn sleep(_seconds: f64) {
            unimplemented!("Not supported for WASM.");
        }
    }
}
