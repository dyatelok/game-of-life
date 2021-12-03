extern crate find_folder;
extern crate piston_window;

use piston_window::*;

struct Game {
    window: PistonWindow,
    columns: u32,
    rows: u32,
    square_side: u32,
    grid: Vec<Vec<bool>>,
}

impl Game {
    fn new(rows: u32, columns: u32, square_side: u32, grid: Vec<Vec<bool>>) -> Game {
        let g = Game {
            window: WindowSettings::new("snake", [rows * square_side, columns * square_side])
                .exit_on_esc(true)
                .graphics_api(OpenGL::V3_2)
                .build()
                .unwrap(),
            columns: columns,
            rows: rows,
            square_side: square_side,
            grid: grid,
        };
        g
    }
    fn run(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        println!("{:?}", assets);
        let mut glyphs = self
            .window
            .load_font(assets.join("FiraSans-Regular.ttf"))
            .unwrap();

        //self.window.set_lazy(true);
        while let Some(e) = self.window.next() {
            //Rendering picture on screen
            self.window.draw_2d(&e, |c, g, device| {
                clear([0.0, 0.0, 0.0, 1.0], g);
                for i in 0..self.rows {
                    for j in 0..self.columns {
                        if self.grid[i as usize][j as usize] == true {
                            let x: f64 =
                                (self.square_side * j) as f64 + self.square_side as f64 * 0.1;
                            let y: f64 =
                                (self.square_side * i) as f64 + self.square_side as f64 * 0.1;

                            let xs: f64 = self.square_side as f64 * 0.8;
                            let ys: f64 = self.square_side as f64 * 0.8;
                            let size: [f64; 4] = [x, y, xs, ys];
                            Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw(
                                size,
                                &c.draw_state,
                                c.transform,
                                g,
                            );
                        }
                    }
                }
                /*let transform = c.transform.trans(10.0, 100.0);
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32).draw(
                    "game of life!",
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                ).unwrap();*/

                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            });
            //Getting button presses
            if let Event::Input(input, _) = e {
                if let Input::Button(button_args) = input {
                    if let Button::Keyboard(key) = button_args.button {
                        match (key, button_args.state) {
                            (Key::Space, ButtonState::Press) => {
                                self.update();
                                //println!("{}", "pepe");
                            }
                            _ => {}
                        }
                        /*if key == Key::Space && button_args.state == ButtonState::Press {
                            println!("{}","You pressed space");
                        }
                        // Hold down a key, and see the message repeated in your terminal.
                        println!("Key event: {:?} {:?}", key, button_args.state);*/
                    }
                }
            }
            //Updating game state
        }
    }
    fn update(&mut self) {
        let mut grid: Vec<Vec<bool>> = vec![vec![false; self.columns as usize]; self.rows as usize];
        for i in 0..self.rows {
            for j in 0..self.columns {
                let mut around: u32 = 0;
                if self.grid[((i + self.rows - 1) % self.rows) as usize][((j + self.columns - 1) % self.columns) as usize] == true {around += 1;}
                if self.grid[((i + self.rows - 1) % self.rows) as usize][((j + self.columns + 0) % self.columns) as usize] == true {around += 1;}
                if self.grid[((i + self.rows - 1) % self.rows) as usize][((j + self.columns + 1) % self.columns) as usize] == true {around += 1;}
                if self.grid[((i + self.rows + 0) % self.rows) as usize][((j + self.columns - 1) % self.columns) as usize] == true {around += 1;}
                if self.grid[((i + self.rows + 0) % self.rows) as usize][((j + self.columns + 1) % self.columns) as usize] == true {around += 1;}
                if self.grid[((i + self.rows + 1) % self.rows) as usize][((j + self.columns - 1) % self.columns) as usize] == true {around += 1;}
                if self.grid[((i + self.rows + 1) % self.rows) as usize][((j + self.columns + 0) % self.columns) as usize] == true {around += 1;}
                if self.grid[((i + self.rows + 1) % self.rows) as usize][((j + self.columns + 1) % self.columns) as usize] == true {around += 1;}

                if self.grid[((i + self.rows + 0) % self.rows) as usize][((j + self.columns + 0) % self.columns) as usize] == true {
                    if around < 2  {grid[i as usize][j as usize] = false;} else 
                    if around == 2 {grid[i as usize][j as usize] = true; } else 
                    if around == 3 {grid[i as usize][j as usize] = true; } else 
                    if around > 3  {grid[i as usize][j as usize] = false;}
                } else {
                    if around == 3 {grid[i as usize][j as usize] = true; } else
                                   {grid[i as usize][j as usize] = false;}
                }
            }
        //println!("{}","Dima Lox!");
        }
        self.grid = grid;
    }
}
fn main() {
    let rows: u32 = 40;
    let columns: u32 = 40;
    let square_size: u32 = 20;
    let mut grid: Vec<Vec<bool>> = vec![vec![false; columns as usize]; rows as usize];
    grid[0+1][1+1] = true;
    grid[1+1][2+1] = true;
    grid[2+1][0+1] = true;
    grid[2+1][1+1] = true;
    grid[2+1][2+1] = true;
    /*grid[1][1] = true;
    grid[1][2] = true;
    grid[2][1] = true;
    grid[2][2] = true;*/
    let mut game = Game::new(rows, columns, square_size, grid);
    game.run();
}
