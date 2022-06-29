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
    range_mesh: Mesh
}

impl Actor {
    pub fn new(ctx: &mut Context, pos: Point2<f32>, view_rad: f32, ray_num: u16) -> Actor {
        let actor_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2{ x: 0.0, y: 0.0 },
            5.0,
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
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        graphics::draw(ctx, &self.actor_mesh, DrawParam::default().dest(self.pos));
    }

    pub fn draw_range(&self, ctx: &mut Context) {
        graphics::draw(ctx, &self.range_mesh, DrawParam::default().dest(self.pos));
    }

    pub fn update_pos(&mut self, vel: Vector2<f32>) {
        self.pos.x += vel.x;
        self.pos.y += vel.y;
    }

    // pub fn look_around(&mut self, map: &mut map::Map) {
    //     let inc = std::f32::consts::PI / self.ray_num as f32;

    //     let mut theta = 0.0;
    //     while theta < 2.0 * std::f32::consts::PI {
    //         let dx = f32::cos(theta)*self.view_rad;
    //         let dy = f32::sin(theta)*self.view_rad;

    //         let ray_endpoint = Point2{ x: self.pos.x + dx, y: self.pos.y + dy};

    //         map.dda(&mut map.tile_states, map.tile_size, map.dimensions, self.pos, ray_endpoint);

    //         theta += inc;
    //     }
    // }

    // ahahahhhaaaaaa this also needs my_game
    // pub fn overlaps_with(&mut self, ) -> Vec<u16>

}