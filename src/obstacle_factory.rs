use std::time;

use tetra::Context;

use crate::obstacle::Obstacle;


pub struct ObstacleFactory{
    last_obstacle: time::Instant
}

impl ObstacleFactory{
    pub fn update(&mut self, obstacles: &mut Vec<Obstacle>, ctx: &mut Context){
        if self.last_obstacle.elapsed().as_millis() > 3000 {
            Obstacle::add_obstacle(obstacles, ctx);
            self.last_obstacle = time::Instant::now();
        }


    }

    pub fn new()->ObstacleFactory{
        ObstacleFactory { last_obstacle: time::Instant::now() }
    }

}