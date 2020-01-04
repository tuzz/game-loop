use game_loop::game_loop;

const GAME: &'static str = "fake game";

#[test]
fn it_can_exit_the_game_loop_from_the_update_or_render_closure() {
    game_loop(GAME, 100, |g| { g.exit(); }, |_| {});
    game_loop(GAME, 100, |_| {}, |g| { g.exit(); });
}

#[test]
fn it_returns_the_control_struct_after_the_game_loop_exits() {
    let control = game_loop(GAME, 100, |g| { g.exit(); }, |_| {});

    assert_eq!(control.exit_next_iteration, true);
}

#[test]
fn it_provides_game_to_the_closures() {
    game_loop(GAME, 100, |g| {
        assert_eq!(g.game, "fake game");
        g.exit();
    }, |_| {});
}

#[test]
fn it_provides_updates_per_second_to_the_closures() {
    game_loop(GAME, 100, |g| {
        assert_eq!(g.updates_per_second, 100);
        g.exit();
    }, |_| {});
}

#[test]
fn it_provides_fixed_time_step_to_the_closures() {
    game_loop(GAME, 100, |g| {
        assert_eq!(g.fixed_time_step(), 0.01);
        g.exit();
    }, |_| {});
}

#[test]
fn it_provides_number_of_updates_to_the_closures() {
    let mut i = 0;

    game_loop(GAME, 100, |g| {
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

    game_loop(GAME, 100, |_| {}, |g| {
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

    game_loop(GAME, 100, |g| {
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

    game_loop(GAME, 100, |g| {
        if i == 0 { approx_eq(g.accumulated_time(), 0.01); }
        if i == 1 { approx_eq(g.accumulated_time(), 0.01); }
        if i == 2 { approx_eq(g.accumulated_time(), 0.01); }
        if i == 3 { g.exit(); }

        i += 1;
    }, |_| {});
}

#[test]
fn it_calls_the_update_function_according_to_updates_per_second() {
    let control = game_loop(GAME, 100, |g| {
        if g.running_time() > 0.1 { g.exit(); }
    }, |_| {});

    assert_eq!(control.number_of_updates(), 10);
}

#[test]
fn it_calls_the_render_function_as_quickly_as_possible() {
    let control = game_loop(GAME, 100, |g| {
        if g.running_time() > 0.1 { g.exit(); }
    }, |_| {});

    assert!(control.number_of_renders() > 1000);
}

#[test]
fn it_provides_blending_factor_so_that_render_can_interpolate_between_frames() {
    game_loop(GAME, 100, |_| {}, |g| {
        assert!(g.blending_factor() > 0.0);
        assert!(g.blending_factor() < 0.0001);

        g.exit();
    });
}

fn approx_eq(actual: f64, expected: f64) {
    let delta = (actual - expected).abs();

    assert!(delta < 0.005, "{} is not approximately {}", actual, expected);
}
