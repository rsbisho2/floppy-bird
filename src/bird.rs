
use std::time;

use tetra::{graphics::{mesh::{Mesh, GeometryBuilder, ShapeStyle}, Color}, Context, math::Vec2};

pub struct Bird{
    pub position: (f32, f32),
    pub velocity: f32,
    pub acceleration: f32,
    pub bird_sprite: Mesh,
    last_jump: time::Instant,
    last_update: time::Instant
}

impl Bird {
    pub fn draw (&self, ctx: &mut Context){
        self.bird_sprite.draw(ctx, Vec2::new(self.position.0 as f32,1280.0 - self.position.1 as f32));
    }

    pub fn jump(&mut self){

        if self.last_jump.elapsed().as_millis() > 500 as u128 {
            self.velocity += 1.0;
            self.velocity = self.velocity.max(0.001);
            self.last_jump = time::Instant::now();
        }
        
    }

    pub fn update(&mut self){
        self.position.1 += self.velocity * self.last_update.elapsed().as_millis() as f32;
        self.velocity += self.acceleration * self.last_update.elapsed().as_millis() as f32;
        self.last_update = time::Instant::now();
    }

    pub fn new(ctx: &mut Context) -> Bird{
        Bird { 
            position: (360.0,640.0), 
            velocity: (0.0), 
            acceleration: -0.0008,
            bird_sprite: (GeometryBuilder::new()
            // Background
            .set_color(Color::rgb(1.0, 1.0, 0.0))
            .circle(ShapeStyle::Fill, Vec2::zero(), 16.0).unwrap()
            .build_mesh(ctx)).unwrap(),
            last_jump: time::Instant::now(),
            last_update: time::Instant::now() }
        
    }
}
