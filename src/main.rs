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

mod background;
use crate::background::Background;

mod obstacle;
use crate::obstacle::Obstacle;

struct GameState{
    bird: Bird,
    last_update: time::Instant,
    obstacles: Vec<Obstacle>,
    last_obstacle: time::Instant,
    game_over: bool,
    score: u16,
    last_score: time::Instant,
    high_score: u16,
    background: Background
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        let bird:Bird = Bird::new(ctx);

        let last_update = time::Instant::now();
        let last_obstacle = time::Instant::now();
        let last_score = time::Instant::now();

        let obstacles: Vec<Obstacle> = Vec::new();

        let game_over: bool = false;

        let score: u16 = 0;

        let high_score = 0;

        let background = Background::new(ctx, 1.0);

        Ok(GameState{bird, last_update, obstacles, last_obstacle, game_over, score, last_score, high_score, background})
    }

}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        let mut rng = rand::thread_rng();

        if !self.game_over{

            self.bird.update();
            self.background.update();

            if input::is_key_down(ctx, Key::Space){
                
                self.bird.jump();
            }

            if self.last_obstacle.elapsed().as_secs() > 3 {
                Obstacle::add_obstacle(&mut self.obstacles);
                self.last_obstacle = time::Instant::now();
            }

            if self.last_score.elapsed().as_millis() > 250 {
                self.score = self.score + 1;
                self.last_score = time::Instant::now();

                if self.score > self.high_score {
                    self.high_score = self.score;
                }
            }

            for obs in self.obstacles.iter_mut(){
                obs.update();

            }

            self.obstacles.retain(|r| r.rect.x>0.1);

            // check for bird out of bounds
            if self.bird.position.1 < 0.0 || self.bird.position.1 > 1280.0{
                self.game_over = true;
            }

            if self.obstacles.iter().any(|f| f.rect.contains_point(Vec2::new(self.bird.position.0, 1280.0-self.bird.position.1))){
                self.game_over = true;
            }

            

        }

        if input::is_key_down(ctx, Key::Space) &&
                self.game_over{
                    self.obstacles.clear();
                    self.score = 0;
                    self.bird.position.1 = 640.0;    
                    self.game_over = false;
                    self.last_obstacle = time::Instant::now();
                    self.last_score = time::Instant::now();
                    self.bird.velocity = 0.0;
                    
                }

        self.last_update = time::Instant::now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {

        self.background.draw(ctx);

        self.bird.draw(ctx);

        for obs in self.obstacles.iter_mut(){
            obs.draw(ctx);
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

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Floppy Bird", 1280, 1280)
        .build()?
        .run(GameState::new)
}