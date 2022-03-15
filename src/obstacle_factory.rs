use std::time;

use tetra::{Context, graphics::Rectangle};

use crate::obstacle::Obstacle;


pub struct ObstacleFactory{
    last_obstacle: time::Instant,
    pub difficulty: u8,
    start_time: time::Instant
}

impl ObstacleFactory{
    pub fn update(&mut self, obstacles: &mut Vec<Obstacle>, ctx: &mut Context){
        if self.last_obstacle.elapsed().as_millis() > 3000 - 150 * self.difficulty.min(20) as u128 {
            //Obstacle::add_random_obstacle(obstacles, ctx);

            let position_center : f32 = 100.0 * f32::cos(self.start_time.elapsed().as_secs_f32() * 0.63) + 1280.0 / 2.0;

            let opening_width : f32 = 100.0;

            Obstacle::add_obstacle(obstacles, ctx, Rectangle::new(1280.0, 0.0, 120.0, position_center - opening_width));
            Obstacle::add_obstacle(obstacles, ctx, Rectangle::new(1280.0, position_center + opening_width,  120.0, 1280.0 - position_center + opening_width));

            self.last_obstacle = time::Instant::now();
        }

    }

    pub fn new()->ObstacleFactory{
        ObstacleFactory { last_obstacle: time::Instant::now(), difficulty:1, start_time : time::Instant::now() }
    }

}