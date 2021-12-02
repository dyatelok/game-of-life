extern crate piston_window;
extern crate find_folder;

use piston_window::*;

struct Game{
    window : PistonWindow,
}

impl Game{
    fn new(width : u32, height : u32) -> Game{
        let g = Game{
            window : WindowSettings::new("snake",[width, height]).exit_on_esc(true).graphics_api(OpenGL::V3_2).build().unwrap(),
        };
        g
    }
    fn run(&mut self, columns : u32, rows :u32) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        println!("{:?}", assets);
        let mut glyphs = self.window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

        //self.window.set_lazy(true);
        while let Some(e) = self.window.next() {
            //Rendering picture on screen
            self.window.draw_2d(&e, |c, g, device| {
                let transform = c.transform.trans(10.0, 100.0);
                
                clear([0.0, 0.0, 0.0, 1.0], g);
                for i in (1 as i32)..(2 as i32) {
                    for j in (1 as i32)..(2 as i32) {
                        let size : [f64; 4] = [ 0.0 + 100.0 * (i as f64), 0.0 + 100.0 * (j as f64), 100.0 + 100.0 * (i as f64), 100.0 + 100.0 * (j as f64)];
                        Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw( size, &c.draw_state, c.transform, g);
                    }
                }
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32).draw(
                    "game of life!",
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                ).unwrap();
                /*text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32).draw(
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
                        match ( key, button_args.state) {
                            ( Key::Space, ButtonState::Press) => {println!("{}","pepe");}
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

    }
}
fn main() {
    let columns : u32 = 10;
    let rows : u32 = 10;
    let square_size : u32 = 40;
    let WIDTH : u32 = columns*square_size;
    let HEIGHT : u32 = rows*square_size;

    let mut game = Game::new( WIDTH, HEIGHT);
    game.run( columns, rows);
}



