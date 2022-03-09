use std::time;

use rand::Rng;
use tetra::{Context, graphics::{self, Color, Texture, ImageData, Rectangle, DrawParams, text::Text}, math::Vec2};

pub struct Background{
    speed:f32,
    layers: Vec<BackgroundLayer>,
    cloud_texture: Texture,
    clouds: Vec<Cloud>,
    last_cloud: time::Instant
}

impl Background{
    pub fn new(ctx: &mut Context, speed:f32)->Background{
        let sprite_sheet: ImageData = ImageData::from_file("./gfx/1.png").unwrap();

        let layer1_texture : Texture = sprite_sheet.region(Rectangle::new(42,28,40,31)).to_texture(ctx).unwrap();
        let layer2_texture : Texture = sprite_sheet.region(Rectangle::new(83,28,40,31)).to_texture(ctx).unwrap();
        let layer3_texture : Texture = sprite_sheet.region(Rectangle::new(124,28,40,31)).to_texture(ctx).unwrap();

        let layer1 = BackgroundLayer::new(layer1_texture,60.0,0.05);
        let layer2 = BackgroundLayer::new(layer2_texture,60.0,0.1);
        let layer3 = BackgroundLayer::new(layer3_texture,60.0,0.2);

        let cloud_texture : Texture = sprite_sheet.region(Rectangle::new(70,1,16,10)).to_texture(ctx).unwrap();

        let mut layers: Vec<BackgroundLayer> = Vec::new();
        layers.push(layer1);
        layers.push(layer2);
        layers.push(layer3);

        let mut clouds :Vec<Cloud> = Vec::new();

        let  cloud = Cloud::new(cloud_texture.clone());
        clouds.push(cloud);

        Background{
            speed:speed,
            layers: layers,
            cloud_texture: cloud_texture,
            clouds: clouds,
            last_cloud: time::Instant::now()
        }
    }

    pub fn draw(&mut self, ctx: &mut Context){
        graphics::clear(ctx, Color::rgb(0.050, 0.398, 0.915));

        for layer in self.layers.iter(){
            layer.draw(ctx)
        }

        for cloud in self.clouds.iter(){
            cloud.draw(ctx);
        }

    }

    pub fn update(&mut self){
        let mut rng = rand::thread_rng();
        if self.last_cloud.elapsed().as_millis() > 2500 {
            self.clouds.push(Cloud::new(self.cloud_texture.clone()));
            self.last_cloud = time::Instant::now();
        }
        for layer in self.layers.iter_mut(){
            layer.update();
        }
        for cloud in self.clouds.iter_mut(){
            cloud.update();
        }

    }
}

struct BackgroundLayer{
    texture: Texture,
    position: f32,
    speed: f32,
    last_update: time::Instant
}
impl BackgroundLayer{
    fn new(texture: Texture, position: f32, speed: f32)->BackgroundLayer{
        BackgroundLayer { texture: texture, position: position, speed: speed,
            last_update: time::Instant::now() }
    }

    fn draw(&self, ctx: &mut Context){
        let mut params = DrawParams::new();
        params.position = Vec2::new(self.position, 0.0);
        params.scale = Vec2::new(1280.0/40.0, 1280.0/31.0);

        self.texture.draw(ctx, params);

        let mut params = DrawParams::new();
        params.position = Vec2::new(self.position-1280.0, 0.0);
        params.scale = Vec2::new(1280.0/40.0, 1280.0/31.0);

        self.texture.draw(ctx, params);
        
        let mut params = DrawParams::new();
        params.position = Vec2::new(self.position+1280.0, 0.0);
        params.scale = Vec2::new(1280.0/40.0, 1280.0/31.0);
        
        self.texture.draw(ctx, params);
    }

    fn update(&mut self){
        self.position -= self.last_update.elapsed().as_millis() as f32 * self.speed; 
        self.last_update= time::Instant::now();

        if self.position < 0.0 {
            self.position += 1280.0
        }
    }
}

struct Cloud {
    position: f32,
    height: f32,
    speed: f32,
    last_update : time::Instant,
    cloud_texture: Texture
}

impl Cloud{
    fn update(&mut self){
        self.position -= self.last_update.elapsed().as_millis() as f32 * self.speed; 
        self.last_update = time::Instant::now();
    }

    fn draw(&self, ctx: &mut Context){
        let mut params = DrawParams::new();
        params.position = Vec2::new(self.position, self.height);
        params.scale = Vec2::new(12.0, 12.0);

        self.cloud_texture.draw(ctx, params);
        

    }

    fn new(cloud_texture: Texture) -> Cloud{
        let mut rng = rand::thread_rng();
        let position : f32 = 1300.0;
        let speed: f32 = rng.gen_range(0.01.. 0.9);
        let height : f32 = rng.gen_range(40.0..400.0);
        Cloud { position: position, height: height, speed: speed, last_update: time::Instant::now(), cloud_texture }

    }
}