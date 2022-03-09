use std::time;

use tetra::{Context, graphics::{self, Color, Texture, ImageData, Rectangle, DrawParams}, math::Vec2};

pub struct Background{
    speed:f32,
    layers: Vec<BackgroundLayer>
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

        let mut layers: Vec<BackgroundLayer> = Vec::new();
        layers.push(layer1);
        layers.push(layer2);
        layers.push(layer3);

        Background{
            speed:speed,
            layers: layers
        }
    }

    pub fn draw(&mut self, ctx: &mut Context){
        graphics::clear(ctx, Color::rgb(0.050, 0.398, 0.915));

        for layer in self.layers.iter(){
            layer.draw(ctx)
        }

    }

    pub fn update(&mut self){
        for layer in self.layers.iter_mut(){
            layer.update();
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