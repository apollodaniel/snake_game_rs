use piston::{EventLoop, PressEvent, UpdateEvent};
use snake_game_piston::snake_game::{self, Apple};


fn main() {
    let mut window: piston_window::PistonWindow = piston_window::WindowSettings::
        new("Snake Game", (snake_game::WINDOW_SIZE,snake_game::WINDOW_SIZE))
        .resizable(false)
        .exit_on_esc(true)
        .build()
        .expect("Error building game window!");
    
    let frame_rate = 3.0;
    

    window.set_max_fps(frame_rate as u64 * 2);

    let mut snake = snake_game::Snake::new();

    let mut last_update = std::time::Instant::now();

    let mut apple = Apple::new();

    while let Some(event) = &window.next() {

        if let Some(_) = event.update_args() {
            let elapsed = last_update.elapsed();
            if elapsed >= std::time::Duration::from_secs_f64(1.0 / frame_rate) {
                last_update = std::time::Instant::now();

                snake.update(&mut apple);
            }
        }


        window.draw_2d(event, |c, g, _gf|{
            piston_window::clear([0.0,0.0,0.0,1.0], g);

            apple.draw(c, g);
            snake.draw(c, g);
        });

        event.press(|f|{
            if let piston::Button::Keyboard(key) = f {
                snake.change_direction(key);
            }
        });

    }

}
