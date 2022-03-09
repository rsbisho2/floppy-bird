use std::time;

use rand::Rng;
use tetra::{graphics::{Rectangle, mesh::*, Color}, Context, math::Vec2};

pub struct Obstacle{
    pub rect: Rectangle,
    last_update: time::Instant
}

impl Obstacle{
    fn new()->Obstacle{
        let mut rng = rand::thread_rng();
        
        let obs_height = rng.gen_range(200.0 .. 800.0);
        Obstacle { rect:(Rectangle{
            x: 1280.0,
            y:
                match rng.gen_bool(0.5) {
                    true => 1280.0 - obs_height,
                    false => 0.0
                }, 
            height: obs_height,
            width: 120.0
        }),
        last_update: time::Instant::now()}
    }

    pub fn add_obstacle(obstacles: &mut Vec<Obstacle>){
        obstacles.push(Obstacle::new());
    }

    pub fn update(&mut self){
        self.rect.x -= self.last_update.elapsed().as_millis() as f32 * 0.1;
        self.last_update = time::Instant::now();
    }

    pub fn draw(&self, ctx:&mut Context){
        let obs_sprite : Mesh = GeometryBuilder::new()
                .set_color(Color::rgb(0.392, 0.584, 0.929))
                .rectangle(ShapeStyle::Fill, Rectangle::new(0.0, 0.0, self.rect.width, self.rect.height)).unwrap()
            .build_mesh(ctx).unwrap();


            obs_sprite.draw(ctx, Vec2::new(
                self.rect.x,
                self.rect.y));
    }
}