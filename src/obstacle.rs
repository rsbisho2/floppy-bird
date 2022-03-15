use std::time;

use rand::Rng;
use tetra::{graphics::{Rectangle, mesh::*, Color, ImageData, Texture, DrawParams, NineSlice}, Context, math::{Vec2, Rect}};

pub struct Obstacle{
    pub rect: Rectangle,
    last_update: time::Instant,
    obstacle_texture: Texture
}

impl Obstacle{
    fn new(ctx: &mut Context)->Obstacle{
        let mut rng = rand::thread_rng();

        let sprite_sheet: ImageData = ImageData::from_file("./gfx/1.png").unwrap();

        let obstacle_sprite = sprite_sheet.region(Rectangle::new(3,9,12,3));

        let obstacle_texture = obstacle_sprite.to_texture(ctx).unwrap();
        
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
        last_update: time::Instant::now(),
        obstacle_texture}
    }

    pub fn add_random_obstacle(obstacles: &mut Vec<Obstacle>, ctx: &mut Context){
        obstacles.push(Obstacle::new(ctx));
    }

    pub fn add_obstacle(obstacles: &mut Vec<Obstacle>, ctx: &mut Context, rect: Rectangle){
        let sprite_sheet: ImageData = ImageData::from_file("./gfx/1.png").unwrap();

        let obstacle_sprite = sprite_sheet.region(Rectangle::new(3,9,12,3));

        let obstacle_texture = obstacle_sprite.to_texture(ctx).unwrap();

        let obs = Obstacle{
            rect:rect,
            last_update: time::Instant::now(),
            obstacle_texture
        };

        obstacles.push(obs);
    }

    pub fn update(&mut self){
        self.rect.x -= self.last_update.elapsed().as_millis() as f32 * 0.3;
        self.last_update = time::Instant::now();
    }

    pub fn draw(&self, ctx:&mut Context){

        let mut params: DrawParams = DrawParams::new();
        params.position = Vec2::new(self.rect.x, self.rect.y);

        let config = NineSlice::with_border(Rectangle { x: 0.0, y: 0.0, width: 12.0, height: 3.0 }, 2.0);

        self.obstacle_texture.draw_nine_slice(ctx, &config, self.rect.width, self.rect.height, params);
                
    }
}