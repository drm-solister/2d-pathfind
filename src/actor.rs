use ggez::graphics::{self, Mesh, DrawParam, Color};
use ggez::{Context, GameResult};
use mint::*;

use crate::map;
use crate::{MyGame, TileState};


pub struct Actor {
    pub pos: Point2<f32>,
    vel: Vector2<f32>,
    pub view_rad: f32,
    pub ray_num: u16,
    actor_mesh: Mesh,
    range_mesh: Mesh,
    rad: f32,
}

impl Actor {
    pub fn new(ctx: &mut Context, pos: Point2<f32>, view_rad: f32, ray_num: u16, rad: f32) -> Actor {
        let actor_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2{ x: 0.0, y: 0.0 },
            rad,
            0.1,
            Color::BLUE
        ).unwrap();

        let range_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::stroke(2.0),
            Point2{ x: 0.0, y: 0.0 },
            view_rad,
            0.1,
            Color::RED
        ).unwrap();

        Actor {
            pos,
            vel: Vector2{x: 0.0, y: 0.0},
            view_rad,
            ray_num,
            actor_mesh,
            range_mesh,
            rad,
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.actor_mesh, DrawParam::default().dest(self.pos)).unwrap();
        Ok(())
    }

    pub fn draw_range(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.range_mesh, DrawParam::default().dest(self.pos)).unwrap();
        Ok(())
    }

    pub fn update_pos(&mut self, vel: Vector2<f32>, map: &mut map::Map) {

        let speed = 1;
        //let theta = vel.y/vel.x;
        // let theta = vel.y.atan2(vel.x);
        // let theta = vel.x.atan2(vel.y);

        let magnitude = f32::sqrt( f32::powf(vel.x, 2.0) + f32::powf(vel.y, 2.0) );
        println!("magnitude: {:?}", magnitude);

        self.pos.x += vel.x;
        self.pos.y += vel.y;

        if self.pos.x < 0.0 || self.pos.x > map.tile_size * map.dimensions.0 as f32 {
            self.pos.x -= vel.x;
        }

        if self.pos.y < 0.0 || self.pos.y > map.tile_size * map.dimensions.1 as f32 {
            self.pos.y -= vel.y;
        }

        // check if new position is valid
        let overlapping_tiles = self.overlaps_with(map);

        for tile in overlapping_tiles {
            if let map::TileState::Full(_) = map.tile_states[tile] {
                // collision
                //println!("collided");
                self.pos.x -= vel.x;
                self.pos.y -= vel.y;
                return;
            }
        }
    }

    // what tiles does the actor overlap
    pub fn overlaps_with(&mut self, map: &mut map::Map) -> Vec<usize> {
        let mut tiles = Vec::new();
        
        // the areas of interest are the points where a circumscribed square would contact the shape of the actor (for now the actor is a circle)
        let mut indicies_to_check = Vec::new();
        // this sucks
        indicies_to_check.push( map.index_from_mouse(Point2{x: self.pos.x+self.rad/2.0, y: self.pos.y}));
        indicies_to_check.push( map.index_from_mouse(Point2{x: self.pos.x-self.rad/2.0, y: self.pos.y}));
        indicies_to_check.push( map.index_from_mouse(Point2{x: self.pos.x, y: self.pos.y+self.rad/2.0}));
        indicies_to_check.push( map.index_from_mouse(Point2{x: self.pos.x, y: self.pos.y-self.rad/2.0}));
        
        for index in indicies_to_check {
            if !tiles.contains(&index) { tiles.push(index); }
        }

        return tiles;
    }

}