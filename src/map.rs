use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, Color, DrawParam, Image};
use mint::*;
use crate::{MyGame};

use crate::actor;

// pub fn index_from_mouse(my_game: &mut MyGame, coords: Point2<f32>) -> usize {

//     let index = y as u16 * self.dim_x + x as u16; // i thought x and y should be swapped on this line, may cause problems
//     return index as usize
// }

pub struct Map {
    pub dimensions: (u16, u16),
    pub tile_size: f32,
    pub tile_states: Vec<TileState>,
}

#[derive(Clone, Copy, Debug)]
pub enum TileState {
    Empty(bool),
    Full(bool),
}

impl Map {
    pub fn new(dimensions: (u16, u16), tile_size: f32) -> Self {

        let mut tile_states = vec![TileState::Empty(false); (dimensions.0 as u32 *dimensions.1 as u32) as usize];
        // for i in 0..dimensions.0*dimensions.1 {
        //     if i%10 == 0 && i%30 != 0{
        //         tile_states[i as usize] = TileState::Full(false);
        //     }
        // }

        Map {
            dimensions,
            tile_size,
            tile_states,
        }

    }

    pub fn index_from_mouse(&mut self, mouse: Point2<f32>) -> usize {
        let mut x = (mouse.x - mouse.x % self.tile_size) / self.tile_size;
        let mut y = (mouse.y - mouse.y % self.tile_size) / self.tile_size;

        x = x.clamp(0.0, self.dimensions.0 as f32 -1.0); // why tf is x and y switched ??
        y = y.clamp(0.0, self.dimensions.1 as f32 -1.0);

        let index = y as u16 * self.dimensions.0 + x as u16;
        return index as usize
    }

    pub fn coords_from_mouse(&mut self, mouse: Point2<f32>) -> Point2<u16> {
        let mut x = (mouse.x - mouse.x % self.tile_size) / self.tile_size;
        let mut y = (mouse.y - mouse.y % self.tile_size) / self.tile_size;

        x = x.clamp(0.0, self.dimensions.0 as f32 -1.0); // why tf is x and y switched ??
        y = y.clamp(0.0, self.dimensions.1 as f32 -1.0);

        return Point2{x: x as u16, y: y as u16}
    }

    pub fn point_to_point_dist() {
        todo!()
    }

    pub fn draw_line(ctx: &mut Context, origin: Point2<f32>, endpoint: Point2<f32>) -> GameResult<()> {
        let line = graphics::Mesh::new_line(ctx, &[origin, endpoint], 3.0, Color::RED).unwrap();
        graphics::draw(ctx, &line, DrawParam::default())
    }


    pub fn look_around(&mut self, actor: &mut actor::Actor) {
        let inc = std::f32::consts::PI / actor.ray_num as f32;

        let mut theta = 0.0;
        while theta < 2.0 * std::f32::consts::PI {
            let dx = f32::cos(theta)*actor.view_rad;
            let dy = f32::sin(theta)*actor.view_rad;

            let ray_endpoint = Point2{ x: actor.pos.x + dx, y: actor.pos.y + dy};

            self.dda(actor.pos, ray_endpoint);

            theta += inc;
        }
    }

    pub fn dda(&mut self, origin: Point2<f32>, endpoint: Point2<f32>) {
        let dx = endpoint.x - origin.x;
        let dy = endpoint.y - origin.y;
    
        let steps;
        if dx.abs() > dy.abs() {
            steps = dx.abs();
        } else {
            steps = dy.abs();
        }
    
        let x_inc = dx / steps;
        let y_inc = dy / steps;
    
        let mut x = origin.x;
        let mut y = origin.y;
    
        for _ in 0..=steps as u16 {
            let index = self.index_from_mouse(Point2{ x, y });
    
            match self.tile_states[index] {
                TileState::Full(ref mut x) => {
                    *x = true;
                    return;
                }
                TileState::Empty(ref mut x) => *x = true,
            }
    
            x += x_inc;
            y += y_inc;
        }
    }

}

// line represented by pixels on the screen
struct PxLine {
    origin: Point2<f32>,
    endpoint: Point2<f32>,
}

