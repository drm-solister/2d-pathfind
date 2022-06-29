#![allow(dead_code, unused_imports)]
use ggez;
use ggez::graphics::{self, Mesh, Color, DrawParam, Image, spritebatch::SpriteBatch, MeshBuilder};
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::KeyCode;

// use glam::*;
use mint::*;

mod img_tile;
mod map;
mod actor;

fn main() {

    let dimensions: (u16, u16) = (30, 25);
    let tile_size = 20.0;

    let width = dimensions.0 as f32 * tile_size;
    let height = dimensions.1 as f32 * tile_size;

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("2d test", "rj")
        .window_setup(
            ggez::conf::WindowSetup {
                title: "testing !".to_owned(),
                vsync: false,
                ..ggez::conf::WindowSetup::default()
            }
        )
        .window_mode(
            ggez::conf::WindowMode {
                width,
                height,
                ..ggez::conf::WindowMode::default()
            }
        )
        .build()
        .expect("could not create ggez context");
    

    let my_game = MyGame::new(&mut ctx, dimensions, tile_size);

    event::run(ctx, event_loop, my_game);
}

pub struct MyGame {
    pub map: map::Map,
    pub spritesheet: SpriteSheet,
    pub actor: actor::Actor,
}

impl MyGame {
    pub fn new(ctx: &mut Context, dimensions: (u16, u16), tile_size: f32) -> MyGame {

        // let dimensions: (u16, u16) = (100, 100);
        // let tile_size = 10.0;

        let mut tile_states = vec![TileState::Empty(false); (dimensions.0 as u32 *dimensions.1 as u32) as usize];
        for i in 0..dimensions.0*dimensions.1 {
            if i%10 == 0 && i%30 != 0{
                tile_states[i as usize] = TileState::Full(false);
            }
        }

        MyGame {
            map: map::Map::new(dimensions, tile_size),
            spritesheet: SpriteSheet {
                image: img_tile::new(ctx, tile_size as u16),
                empty: (0.0, 0.5),
                full: (0.0, 0.0),
            },
            actor: actor::Actor::new(ctx, Point2{x: 100.0, y: 100.0}, 50.0, 10),
        }

    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        let mouse_pos = ggez::input::mouse::position(ctx);
        // let pressed = ggez::input::mouse::button_pressed(ctx, ggez::input::mouse::MouseButton::Left);

        //let origin = Point2{x: 10.0, y: 10.0};
        // map::draw_line(ctx, origin, mouse_pos);

        // worse
        //map::find_visible_on_line(self, origin, mouse_pos);

        // better - highlights all the pixels that the line crosses from origin to mouse
        //map::Map::dda(self, origin, mouse_pos);

        // --- handle inputs ---
        let mut v_x = 0.0;
        let mut v_y = 0.0;
        let keys_pressed = ggez::input::keyboard::pressed_keys(ctx);

        for key in keys_pressed.iter() {
            match key {
                KeyCode::W => v_y -= 1.0,
                KeyCode::A => v_x -= 1.0,
                KeyCode::S => v_y += 1.0,
                KeyCode::D => v_x += 1.0,
                _ => (),
            }
        }

        self.actor.update_pos(Vector2{x: v_x, y: v_y});
        // need tile_states, tile_size, dimensions
        self.map.look_around(&mut self.actor);

        // --- fps counter ---
        if ggez::timer::ticks(&ctx) % 50 == 0 {
            println!("fps: {:?}", ggez::timer::fps(&ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        graphics::clear(ctx, Color::WHITE);

        // this creates a spritebatch which sources everything from a single image, which is generated in img_tile.rs
        // based on the TileState that corresponds which each tile, the tile will be hollow or full and have whatever
        // color is desired. (empty tiles are hollow, obstructions are filled)

        let mut batch = SpriteBatch::new(self.spritesheet.image.clone());

        for i in 0..self.map.dimensions.0 {
            for j in 0..self.map.dimensions.1 {
                
                let mut param = DrawParam::default().dest( mint::Point2{ x: i as f32*(self.map.tile_size+0.0), y: j as f32*(self.map.tile_size+0.0)} );

                let mut alpha = 255;
                match self.map.tile_states[(i+self.map.dimensions.0*j) as usize] {
                    map::TileState::Full(known) => {
                        if !known { alpha = 125 }
                        param = param
                            .src( graphics::Rect{ x: 0.0, y:0.0, w:1.0, h:0.5} )
                            .color(Color::from_rgba(9,17,51,alpha));
                    },
                    map::TileState::Empty(ref mut known) => {
                        if !*known { alpha = 125 }
                        param = param
                            .src( graphics::Rect{ x: 0.0, y:0.5, w:1.0, h:0.5} )
                            .color(Color::from_rgba(124,174,195,alpha));
                        //*known = false; // tiles dont stay "known"
                    },
                    _ => (),
                }

                batch.add(param);
            }
        }
        graphics::draw(ctx, &batch, DrawParam::default())?;


        // for debugging, this draws a line
        // let mouse_pos = ggez::input::mouse::position(ctx);
        // let origin = Point2{x: 10.0, y: 10.0};
        // let target = Point2{x: 300.0, y: 400.0};
        // map::draw_line(ctx, origin, mouse_pos);

        self.actor.draw(ctx);
        self.actor.draw_range(ctx);
        
        graphics::present(ctx)

    }
}

// add more for different states
#[derive(Clone, Copy, Debug)]
pub enum TileState {
    Empty(bool),
    Full(bool),
}

// when i make a sprite sheet this struct should have one image and several other fields describing where the bounds of each
// seperate sprite/tile idk the term is located
pub struct SpriteSheet {
    image: Image,
    empty: (f32, f32),
    full: (f32, f32),
}


// attempts to make this fast:
// first:
// create only one mesh and draw it in a different place every time

// second:
// create iamges and create a sprite batch to draw once a frame

// third: (not yet explored, may not be needed)
// meshbuilder?


// current sprite batch approach might have to cut it, at least for now.

// todo
// [x] - make the screen size fit the tile map
// add the actor code back in 
// [x] - get back on finding out how to draw lines on a discrete grid - rudimentary implementation in place
//      is DDA faster than flood fill for my needs?
//      probably. implement DDA or breshzanms or however you spell that
//      but also i may just have slight math mistakes in my algorithm
//      but it literally uses inverse square roots but slow
//      [x] - DDA implemented
// [ ] - make movement speed independent of framerate (when a human is controlling it)
// [ ] - collision
// [ ] - add back in clicking to add full tiles
// [ ] - make sprite for actor and target
// [ ] - refactor everything to make it easier to read
// [ ] - put text on the screen for debugging
// [x] - test performance on a better computer

// could still only update the tiles i need to update and speed up the drawing massively
// an easy thing to do would be to only update tiles in a radius around the actor

// i think its bad to be exporting things from main to sub modules. the data needed by the sub module should be able to be passed to 
// it or contained in it from the start

// possibly use lazy static to keep a copy of my_game in each module
// major problem in the way that data is shared between the modules.
// the look_around() function in actor is looking jankier by the second
// some functions just take my_game but others cant
// can my_game not own actor but just a reference to it? but then what would the lifetime of it be.