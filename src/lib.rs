pub mod snake_game{

    pub const CELL_COUNT: f64 = 12.0;
    pub const WINDOW_SIZE: f64 = 480.0;

    pub fn get_cell_size() -> f64 {WINDOW_SIZE/CELL_COUNT}


    pub enum Direction{
        Left,
        Right,
        Top,
        Down,
    }

    impl Direction {
        pub fn value(&self) -> [f64;2]{
            match self {
                Direction::Top => [0.0, -1.0],
                Direction::Down => [0.0, 1.0],
                Direction::Left => [-1.0, 0.0],
                Direction::Right => [1.0, 0.0],
            }
        }
    }


    pub struct Snake{
        pub scale: f64,
        pub position: [f64;2],
        pub body: Vec<[f64;2]>,
        pub color: [f32;4],
        pub direction: Direction
    }

    impl Snake {
        pub fn new() -> Snake{
            Snake {
                scale: get_cell_size(),
                position: [0.0,0.0],
                body: vec![[0.0,0.0]],
                color: [0.297, 1.0, 0.489, 1.0],
                direction: Direction::Right
            }
        }

        pub fn draw(&self, c: piston_window::Context, g: &mut piston_window::G2d,){
            piston_window::rectangle(self.color, [self.position[0], self.position[1],self.scale, self.scale], c.transform, g)
        }

        pub fn change_direction(&mut self, key: piston::Key){
            match key {
                piston::Key::A => self.direction = Direction::Left,
                piston::Key::D => self.direction = Direction::Right,
                piston::Key::W => self.direction = Direction::Top,
                piston::Key::S => self.direction = Direction::Down,
                _=>{}
            }
        }

        pub fn update(&mut self){
            self.position = [ (self.direction.value()[0]*get_cell_size()) + self.position[0], (self.direction.value()[1]*get_cell_size()) + self.position[1]];
            //self.position = self.direction.value() * get_cell_size();
        }
    }

}