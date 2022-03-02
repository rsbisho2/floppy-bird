use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::graphics::{self, Color, Rectangle};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use tetra::graphics::text::{Text,Font};
use rand::Rng;
use std::time;
use tetra::input::{self, Key};

struct Bird{
    position: (f32, f32),
    velocity: f32,
    acceleration: f32,
    bird_sprite: Mesh
}

#[derive(PartialEq)]
enum Orientation{
    Top,
    Bottom
}

struct Obstacle {
    position: f32,
    orientation: Orientation,
    height: f32

}
struct GameState{
    bird: Bird,
    last_update: time::Instant,
    last_jump: time::Instant,
    obstacles: Vec<Obstacle>,
    last_obstacle: time::Instant,
    game_over: bool
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        let bird:Bird = Bird { 
            position: (360.0,640.0), 
            velocity: (0.0), 
            acceleration: -0.0008,
            bird_sprite: (GeometryBuilder::new()
            // Background
            .set_color(Color::rgb(1.0, 1.0, 0.0))
            .circle(ShapeStyle::Fill, Vec2::zero(), 16.0)?
            .build_mesh(ctx)?) };

        let last_update = time::Instant::now();
        let last_jump = time::Instant::now();
        let last_obstacle = time::Instant::now();

        let obstacles: Vec<Obstacle> = Vec::new();

        let game_over: bool = false;

        Ok(GameState{bird, last_update,last_jump, obstacles, last_obstacle, game_over})
    }

}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let mut rng = rand::thread_rng();

        if !self.game_over{

            self.bird.position.1 += self.bird.velocity * self.last_update.elapsed().as_millis() as f32;
            self.bird.velocity += self.bird.acceleration * self.last_update.elapsed().as_millis() as f32;

            

            if input::is_key_down(ctx, Key::Space) &&
                self.last_jump.elapsed().as_millis() > 500.0 as u128{

                self.bird.velocity += 1.0;
                self.last_jump = time::Instant::now();
            }

            if self.last_obstacle.elapsed().as_secs() > 3 {
                self.last_obstacle = time::Instant::now();
                self.obstacles.push(Obstacle{
                    position: 1240.0,
                    orientation:
                        match rng.gen_bool(0.5) {
                            true => Orientation::Bottom,
                            false => Orientation::Top
                        }, 
                    height: rng.gen_range(200.0 .. 800.0)
                })
            }

            for obs in self.obstacles.iter_mut(){
                obs.position -= self.last_update.elapsed().as_millis() as f32 * 0.1;

            }

            self.obstacles.retain(|x| x.position>0.0);

            // check for bird out of bounds
            if self.bird.position.1 < 0.0 || self.bird.position.1 > 1280.0{
                self.game_over = true;
            }

        }
        self.last_update = time::Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {

        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.1));

        self.bird.bird_sprite.draw(ctx, Vec2::new(self.bird.position.0 as f32,1280.0 - self.bird.position.1 as f32));

        let mut rects : Vec<Rectangle> = Vec::new(); 
        for obs in self.obstacles.iter(){
            rects.push(Rectangle::new(obs.position,
                match obs.orientation {
                    Orientation::Bottom => 1280.0 - obs.height,
                    Orientation::Top => 0.0
                },
            120.0,
        obs.height));
        }

        if rects.iter().any(|f| f.contains_point(Vec2::new(self.bird.position.0, self.bird.position.1))){
            self.game_over = true;
        }

        for obs in self.obstacles.iter_mut(){
            let obs_sprite : Mesh = GeometryBuilder::new()
                .set_color(Color::rgb(0.392, 0.584, 0.929))
                .rectangle(ShapeStyle::Fill, Rectangle::new(0.0, 0.0, 120.0, obs.height))?
            .build_mesh(ctx)?;


            obs_sprite.draw(ctx, Vec2::new(
                obs.position,
                match obs.orientation {
                    Orientation::Bottom => 1280.0 - obs.height,
                    Orientation::Top => 0.0
                }));
        }

        let mut game_over_text = Text::new("Game over, man!",
            Font::vector(ctx, "./fonts/OpenSans-Regular.ttf", 32.0)?);
        if self.game_over {
            game_over_text.draw(ctx, Vec2::new(300.0,300.0));

        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Floppy Bird", 1280, 1280)
        .build()?
        .run(GameState::new)
}