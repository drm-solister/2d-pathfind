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
        for i in 0..dimensions.0*dimensions.1 {
            if i%10 == 0 && i%30 != 0{
                tile_states[i as usize] = TileState::Full(false);
            }
        }

        Map {
            dimensions,
            tile_size,
            tile_states,
        }

    }
}

impl Map {

    pub fn index_from_mouse(&mut self, mouse: Point2<f32>) -> usize {
        let mut x = (mouse.x - mouse.x % self.tile_size) / self.tile_size;
        let mut y = (mouse.y - mouse.y % self.tile_size) / self.tile_size;

        let x = x.clamp(0.0, self.dimensions.0 as f32 -1.0); // why tf is x and y switched ??
        let y = y.clamp(0.0, self.dimensions.1 as f32 -1.0);

        let index = y as u16 * self.dimensions.0 + x as u16;
        return index as usize
    }

    // this one doesnt take my_game but only the parts it needs
    pub fn index_from_mouse2(&mut self, tile_size: f32, dimensions: (u16, u16), mouse: Point2<f32>) -> usize {
        let mut x = (mouse.x - mouse.x % tile_size) / tile_size;
        let mut y = (mouse.y - mouse.y % tile_size) / tile_size;

        let x = x.clamp(0.0, dimensions.0 as f32 -1.0); // why tf is x and y switched ??
        let y = y.clamp(0.0, dimensions.1 as f32 -1.0);

        let index = y as u16 * dimensions.0 + x as u16;
        return index as usize
    }

    pub fn coords_from_mouse(&mut self, mouse: Point2<f32>) -> Point2<u16> {
        let mut x = (mouse.x - mouse.x % self.tile_size) / self.tile_size;
        let mut y = (mouse.y - mouse.y % self.tile_size) / self.tile_size;

        let x = x.clamp(0.0, self.dimensions.0 as f32 -1.0); // why tf is x and y switched ??
        let y = y.clamp(0.0, self.dimensions.1 as f32 -1.0);

        return Point2{x: x as u16, y: y as u16}
    }

    // --- UNUSED ---
    // find all the tiles on a line (based on endpoints) that are visible and return them in a vector. flood-fill algorithm
    // pub fn find_visible_on_line(my_game: &mut MyGame, origin: Point2<f32>, endpoint: Point2<f32>) {
        
    //     // start with the pixel containing the origin
    //     // for all surrounding pixels p...
    //     // if distance between p and line is less than the diagonal, it is visible

    //     let og_coord = coords_from_mouse(my_game, origin);
    //     let ep_coord = coords_from_mouse(my_game, endpoint);
    //     let px_line = PxLine { origin, endpoint };

    //     let mut indicies_seen_this_pass: Vec<usize> = Vec::new();
    //     flood_and_find(my_game.dimensions, &mut my_game.tile_states, &px_line, og_coord, &mut indicies_seen_this_pass);

    //     return;

    // }

    // // --- UNUSED ---
    // // hard coded values in here: horizontal dimension, pixel size
    // fn flood_and_find(dimensions: (u16, u16), tile_states: &mut Vec<TileState>, px_line: &PxLine, point: Point2<u16>, indicies_seen_this_pass: &mut Vec<usize>) {
    //     // hard coded because i dont have access to this data but i should
    //     // if the flood finds a full block, dont continue
    //     // dont retrace your steps or youll loop forever
    //     // println!("{:?} is on the line, checking surrounding coordinates", point);
    //     if point.x == dimensions.0 || point.y == dimensions.1 { return; }   // reached the end of the grid

    //     let index = (point.x + point.y*30) as usize;
    //     if indicies_seen_this_pass.contains(&index) { return; } // already "saw" this square
    //     indicies_seen_this_pass.push(index);

    //     match tile_states[index] {
    //         TileState::Full(ref mut x) => {
    //             *x = true;
    //             return;
    //         },
    //         TileState::Empty(ref mut x) => {
    //             *x = true;
    //         },
    //     }

    //     //for coords in origin.surrounding
    //     for i in -1i16..=1i16 {
    //         for j in -1i16..=1i16 {
    //             // hard coding to get the surrounding pixels, kinda stupid but maybe somewhat necessary
    //             if i == 0 && j == 0 { continue; }
    //             if point.x == 0 && i == -1 { continue; }
    //             if point.y == 0 && j == -1 { continue; }
    //             if point.y == dimensions.1 && j == 1 { continue; }
    //             if point.x == dimensions.0 && i == 1 { continue; }

    //             // println!("point.x: {:?}", point.x);
    //             let new_point: Point2<u16> = Point2{x: (point.x as i16+i) as u16, y: (point.y as i16+j) as u16};

    //             let center_of_target_px = Point2{x: new_point.x as f32 * 9.0, y: new_point.y as f32 * 9.0};
    //             if distance_from_line(px_line.origin, px_line.endpoint, center_of_target_px) < 6.0 {
    //                 flood_and_find(dimensions, tile_states, px_line, new_point, indicies_seen_this_pass);
    //             }
    //         }
    //     }

    // }

    // // --- UNUSED ---
    // // helper function for visible_on_line()
    // // distance from a point to a line (based on endpoints)
    // pub fn distance_from_line(origin: Point2<f32>, endpoint: Point2<f32>, point: Point2<f32>) -> f32 {
    //     let dy = endpoint.y - origin.y;
    //     let dx = endpoint.x - origin.x;

    //     if dy == 0.0 {
    //         return (origin.y - point.y).abs();
    //     } else if dx == 0.0 {
    //         return (origin.x - point.x).abs();
    //     }

    //     let a = dy/dx;

    //     let b = -1.0;

    //     let c = origin.y - origin.x * a;

    //     let distance = (a*point.x + b*point.y + c).abs() / f32::sqrt(a*a + b*b);
    //     //println!("distance: {:?}", distance);
    //     return distance;
    // }

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

            dda(self, actor.pos, ray_endpoint);

            theta += inc;
        }
    }

}

// line represented by pixels on the screen
struct PxLine {
    origin: Point2<f32>,
    endpoint: Point2<f32>,
}

pub fn dda(map: &mut Map, origin: Point2<f32>, endpoint: Point2<f32>) {
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

    for i in 0..=steps as u16 {
        let index = map.index_from_mouse2(map.tile_size, map.dimensions, Point2{ x, y});

        match map.tile_states[index] {
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