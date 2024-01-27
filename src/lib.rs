pub mod snake_game{

    pub const CELL_COUNT: f64 = 12.0;
    pub const WINDOW_SIZE: f64 = 480.0;

    pub fn get_cell_size() -> f64 {WINDOW_SIZE/CELL_COUNT}

    pub struct Snake{
        pub scale: f64,
        pub x: f64,
        pub y: f64,
        pub body: Vec<[f64;2]>,
        pub color: [f32;4]
    }

    impl Snake {
        pub fn new() -> Snake{
            Snake {
                scale: get_cell_size(),
                x: 0.0,
                y: 0.0,
                body: vec![[0.0,0.0]],
                color: [0.297, 1.0, 0.489, 1.0]
            }
        }

        pub fn draw(&self, c: piston_window::Context, g: &mut piston_window::G2d,){
            piston_window::rectangle(self.color, [self.x, self.y,self.scale, self.scale], c.transform, g)
        }

        pub fn update(&mut self){

        }
    }

}