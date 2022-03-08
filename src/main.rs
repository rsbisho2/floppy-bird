use tetra::graphics::mesh::{GeometryBuilder, Mesh, ShapeStyle};
use tetra::graphics::{self, Color, Rectangle};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use tetra::graphics::text::{Text,Font};
use rand::Rng;
use std::time;
use tetra::input::{self, Key};

mod bird;
use crate::bird::Bird;


#[derive(PartialEq)]
enum Orientation{
    Top,
    Bottom
}

struct GameState{
    bird: Bird,
    last_update: time::Instant,
    last_jump: time::Instant,
    obstacles: Vec<Rectangle>,
    last_obstacle: time::Instant,
    game_over: bool,
    score: u16,
    last_score: time::Instant,
    high_score: u16
}
/* 
impl ToString for Rectangle{
    fn to_string(&self) -> String {
        return "x:"+self.x + " y:" + self.y 
        + " w:" + self.width + " h:" + self.height
    }
}
*/
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
        let last_score = time::Instant::now();

        let obstacles: Vec<Rectangle> = Vec::new();

        let game_over: bool = false;

        let score: u16 = 0;

        let high_score = 0;

        Ok(GameState{bird, last_update,last_jump, obstacles, last_obstacle, game_over, score, last_score, high_score})
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
                let obs_height = rng.gen_range(200.0 .. 800.0);
                self.obstacles.push(Rectangle{
                    x: 1280.0,
                    y:
                        match rng.gen_bool(0.5) {
                            true => 1280.0 - obs_height,
                            false => 0.0
                        }, 
                    height: obs_height,
                    width: 120.0
                })
            }

            if self.last_score.elapsed().as_millis() > 250 {
                self.score = self.score + 1;
                self.last_score = time::Instant::now();

                if self.score > self.high_score {
                    self.high_score = self.score;
                }
            }

            for obs in self.obstacles.iter_mut(){
                obs.x -= self.last_update.elapsed().as_millis() as f32 * 0.1;

            }

            self.obstacles.retain(|r| r.x>0.1);

            // check for bird out of bounds
            if self.bird.position.1 < 0.0 || self.bird.position.1 > 1280.0{
                self.game_over = true;
            }

            if self.obstacles.iter().any(|f| f.contains_point(Vec2::new(self.bird.position.0, 1280.0-self.bird.position.1))){
                self.game_over = true;
            }

            

        }

        if input::is_key_down(ctx, Key::Space) &&
                self.game_over{
                    self.obstacles.clear();
                    self.score = 0;
                    self.bird.position.1 = 640.0;    
                    self.game_over = false;
                    self.last_jump = time::Instant::now();
                    self.last_obstacle = time::Instant::now();
                    self.last_score = time::Instant::now();
                    self.bird.velocity = 0.0;
                    
                }

        self.last_update = time::Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {

        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.1));

        self.bird.draw(ctx);
        

        

        for obs in self.obstacles.iter_mut(){
            let obs_sprite : Mesh = GeometryBuilder::new()
                .set_color(Color::rgb(0.392, 0.584, 0.929))
                .rectangle(ShapeStyle::Fill, Rectangle::new(0.0, 0.0, obs.width, obs.height))?
            .build_mesh(ctx)?;


            obs_sprite.draw(ctx, Vec2::new(
                obs.x,
                obs.y));
        }

        // Score text
        let mut score_text = Text::new("Score: ".to_owned() + &u16::to_string(&self.score),
        Font::vector(ctx, "./fonts/OpenSans-Regular.ttf", 32.0)?);
        score_text.draw(ctx, Vec2::new(25.0,25.0));

        let mut high_score_text = Text::new("High Score: ".to_owned() + &u16::to_string(&self.high_score).to_owned(),
        Font::vector(ctx, "./fonts/OpenSans-Regular.ttf", 32.0)?);
        high_score_text.draw(ctx, Vec2::new(25.0,60.0));

        let mut game_over_text = Text::new("Game over, man!",
            Font::vector(ctx, "./fonts/OpenSans-Regular.ttf", 32.0)?);
        if self.game_over {
            game_over_text.draw(ctx, Vec2::new(300.0,300.0));

        }
        /*
        let mut obs_loc:f32 = 40.0;
        for obs in self.obstacles.iter(){
            let mut obs_text = Text::new(obs.to_string(),
            Font::vector(ctx, "./fonts/OpenSans-Regular.ttf", 32.0)?);
            obs_loc = obs_loc + 40.0;
            obs_text.draw(ctx, Vec2::new(25.0,obs_loc));

        }
        */
        let mut bird_pos_text = Text::new(self.bird.position.0.to_string() + ", " + &self.bird.position.1.to_string() ,
        Font::vector(ctx, "./fonts/OpenSans-Regular.ttf", 32.0)?);
        bird_pos_text.draw(ctx, Vec2::new(1000.0,40.0));

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Floppy Bird", 1280, 1280)
        .build()?
        .run(GameState::new)
}