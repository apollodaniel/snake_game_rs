use snake_game_piston::snake_game;


fn main() {
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::
        new("Snake Game", (snake_game::WINDOW_SIZE,snake_game::WINDOW_SIZE))
        .resizable(false)
        .exit_on_esc(true)
        .vsync(true)
        .build()

        .expect("Error building game window!");
    let mut snake = snake_game::Snake::new();

    while let Some(event) = window.next() {
        window.draw_2d(&event, |c, g, _gf|{
            piston_window::clear([0.0,0.0,0.0,1.0], g);

            snake.draw(c, g);
        });
    }

    snake.update();
}
