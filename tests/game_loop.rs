use game_loop::game_loop;
use std::thread::sleep;
use std::time::Duration;

const GAME: &'static str = "fake game";

#[test]
fn it_can_exit_the_game_loop_from_the_update_or_render_closure() {
    game_loop(GAME, 100, 1.0, |g| { g.exit(); }, |_| {});
    game_loop(GAME, 100, 1.0, |_| {}, |g| { g.exit(); });
}

#[test]
fn it_returns_the_control_struct_after_the_game_loop_exits() {
    let control = game_loop(GAME, 100, 1.0, |g| { g.exit(); }, |_| {});

    assert_eq!(control.exit_next_iteration, true);
}

#[test]
fn it_provides_game_to_the_closures() {
    game_loop(GAME, 100, 1.0, |g| {
        assert_eq!(g.game, "fake game");
        g.exit();
    }, |_| {});
}

#[test]
fn it_provides_updates_per_second_to_the_closures() {
    game_loop(GAME, 100, 1.0, |g| {
        assert_eq!(g.updates_per_second, 100);
        g.exit();
    }, |_| {});
}

#[test]
fn it_provides_max_frame_time_to_the_closures() {
    game_loop(GAME, 100, 1.0, |g| {
        assert_eq!(g.max_frame_time, 1.0);
        g.exit();
    }, |_| {});
}

#[test]
fn it_provides_fixed_time_step_to_the_closures() {
    game_loop(GAME, 100, 1.0, |g| {
        assert_eq!(g.fixed_time_step(), 0.01);
        g.exit();
    }, |_| {});
}

#[test]
fn it_provides_number_of_updates_to_the_closures() {
    let mut i = 0;

    game_loop(GAME, 100, 1.0, move |g| {
        if i == 0 { assert_eq!(g.number_of_updates(), 0); }
        if i == 1 { assert_eq!(g.number_of_updates(), 1); }
        if i == 2 { assert_eq!(g.number_of_updates(), 2); }
        if i == 3 { g.exit(); }

        i += 1;
    }, |_| {});
}

#[test]
fn it_provides_number_of_renders_to_the_closures() {
    let mut i = 0;

    game_loop(GAME, 100, 1.0, |_| {}, move |g| {
        if i == 0 { assert_eq!(g.number_of_renders(), 0); }
        if i == 1 { assert_eq!(g.number_of_renders(), 1); }
        if i == 2 { assert_eq!(g.number_of_renders(), 2); }
        if i == 3 { g.exit(); }

        i += 1;
    });
}


#[test]
fn it_provides_running_time_to_the_closures() {
    let mut i = 0;

    game_loop(GAME, 100, 1.0, move |g| {
        if i == 0 { approx_eq(g.running_time(), 0.01); }
        if i == 1 { approx_eq(g.running_time(), 0.02); }
        if i == 2 { approx_eq(g.running_time(), 0.03); }
        if i == 3 { g.exit(); }

        i += 1;
    }, |_| {});
}

#[test]
fn it_provides_accumulated_time_to_the_closures() {
    let mut i = 0;

    game_loop(GAME, 100, 1.0, move |g| {
        if i == 0 { approx_eq(g.accumulated_time(), 0.01); }
        if i == 1 { approx_eq(g.accumulated_time(), 0.01); }
        if i == 2 { approx_eq(g.accumulated_time(), 0.01); }
        if i == 3 { g.exit(); }

        i += 1;
    }, |_| {});
}

#[test]
fn it_calls_the_update_function_according_to_updates_per_second() {
    let control = game_loop(GAME, 100, 1.0, |g| {
        if g.running_time() > 0.1 { g.exit(); }
    }, |_| {});

    assert_eq!(control.number_of_updates(), 10);
}

#[test]
fn it_calls_the_render_function_as_quickly_as_possible() {
    let control = game_loop(GAME, 100, 1.0, |g| {
        if g.running_time() > 0.1 { g.exit(); }
    }, |_| {});

    assert!(control.number_of_renders() > 1000);
}

#[test]
fn it_limits_the_maximum_frame_time_which_reduces_the_number_of_updates() {
    let mut i = 0;

    let g = game_loop(GAME, 100, 0.1, |_| {}, move |g| {
        if i == 0 { sleep(Duration::from_secs_f64(0.2)) };
        if i == 1 { g.exit() };

        i += 1;
    });

    assert_eq!(g.number_of_updates(), 10); // Instead of 20
}

#[test]
fn it_provides_blending_factor_so_that_render_can_interpolate_between_frames() {
    game_loop(GAME, 100, 1.0, |_| {}, |g| {
        assert!(g.blending_factor() > 0.0);
        assert!(g.blending_factor() < 0.0001);

        g.exit();
    });
}

#[test]
fn it_can_re_measure_how_much_time_has_accumulated_mid_way_through_a_render() {
    game_loop(GAME, 100, 1.0, |_| {}, |g| {
        sleep(Duration::from_secs_f64(0.2));
        g.re_accumulate();

        approx_eq(g.running_time(), 0.2);
        approx_eq(g.accumulated_time(), 0.2);

        assert!(g.blending_factor() > 19.);
        assert!(g.blending_factor() < 21.);

        g.exit();
    });
}

fn approx_eq(actual: f64, expected: f64) {
    let delta = (actual - expected).abs();

    assert!(delta < 0.01, "{} is not approximately {}", actual, expected);
}
