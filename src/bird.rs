
use tetra::{graphics::mesh::Mesh, Context, math::Vec2};

pub struct Bird{
    pub position: (f32, f32),
    pub velocity: f32,
    pub acceleration: f32,
    pub bird_sprite: Mesh
}

impl Bird {
    pub fn draw (&self, ctx: &mut Context){
        self.bird_sprite.draw(ctx, Vec2::new(self.position.0 as f32,1280.0 - self.position.1 as f32));
    }
}



