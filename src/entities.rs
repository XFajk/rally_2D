use macroquad::prelude::*;
use crate::effects::{ShapeParticles, Shapes, default_operation};
use crate::rand::*;

enum CarState {
    NotBreaking,
    GoingToBreak,
    Breaking,
}

pub struct Car {
    pub vel: f32,
    pub max_vel: f32,
    pub acceleration: f32,
    pub angle: f32,
    pub direction: f32,
    pub max_direction: f32,
    pub particles: ShapeParticles,

    pub pos: Vec2,

    car_state: CarState,
    breaking_speed: f32,
    particle_direction: f32,
    car_angle: f32
}

impl Car {
    pub fn new(max_vel: f32, acceleration: f32, angle: f32, pos: Vec2) -> Self {
        Self {
            vel: 0.0,
            max_vel,
            acceleration,
            angle,
            pos,
            direction: angle,
            max_direction: 30.0,
            particles: ShapeParticles::new(Shapes::Hexagon, 0.0),
            particle_direction: angle-180.0,
            breaking_speed: 0.1,
            car_angle: angle,
            car_state: CarState::NotBreaking,
        }
    }

    pub fn update(&mut self, dt: f32) {

        // movement and rotation
        match self.car_state {
            CarState::NotBreaking => {
                if is_key_down(KeyCode::W) && self.vel < self.max_vel {                
                    self.vel += self.acceleration * dt;
                }
                else if !is_key_down(KeyCode::W) && self.vel > 0.1 {
                    self.vel -= self.acceleration*dt;
                }

                if is_key_down(KeyCode::S) && self.vel > -self.max_vel {
                    self.vel -= self.acceleration * dt;
                }
                else if !is_key_down(KeyCode::S) && self.vel < -0.1 {
                    self.vel += self.acceleration*dt;
                }
                if is_key_down(KeyCode::A) && !(self.direction < self.angle-self.max_direction)  {
                    self.direction -= 5.0*dt;
                }
                if is_key_down(KeyCode::D) && !(self.direction > self.angle+self.max_direction) {
                    self.direction += 5.0*dt;
                }
                self.particle_direction = self.angle-180.0;

            }
            _ => {}
        }

        if self.vel != 0.0 {
            self.angle += ((self.direction - self.angle) / 10.0)*dt;
        }
        if self.vel < 0.1 && -0.1 < self.vel {
            self.vel = 0.0;
        }

        // breaking 
        if is_key_down(KeyCode::Space) {
            self.vel -= self.breaking_speed*dt;
            match self.car_state {
                CarState::NotBreaking => {
                    self.car_state = CarState::GoingToBreak;
                }
                _ => {}
            }
        }
        else if !is_key_down(KeyCode::Space) || self.vel == 0.0 {
            self.car_state = CarState::NotBreaking;
        }
        
        match self.car_state {
            CarState::NotBreaking => {
                self.car_angle = self.angle;
                 self.pos += Vec2::new(
                    self.direction.to_radians().cos()*self.vel,
                    self.direction.to_radians().sin()*self.vel
                ) * dt;
                if self.vel > 0.0 || self.vel < 0.0 {
                     self.particles.add(
                        self.pos,
                        gen_range(self.particle_direction - 15.0, self.particle_direction + 15.0),
                        gen_range(1.0, 3.0),
                        gen_range(8.0, 9.0),
                        0.5,
                        Color::new(0.7, 0.7, 0.7, 0.3)
                    );
               }

            }
            CarState::GoingToBreak => {
                self.car_angle = self.angle;
                self.car_state = CarState::Breaking;
            }
            CarState::Breaking => {
                self.vel = self.vel.abs();
                self.pos += Vec2::new(
                    self.car_angle.to_radians().cos()*self.vel,
                    self.car_angle.to_radians().sin()*self.vel
                ) * dt;
                if self.vel >= 4.0 {
                    for _i in 0..5 {
                        self.particles.add(
                            self.pos,
                            gen_range(self.particle_direction - 50.0, self.particle_direction + 50.0),
                            gen_range(1.0, 3.0),
                            gen_range(9.0, 10.0),
                            0.2,
                            Color::new(0.2, 0.2, 0.2, 0.6)
                        );
                    }
                }
            }
        }
    }

    pub fn draw(&mut self, dt: f32) {
        // [math.cos(math.radians(angle)) * speed, math.sin(math.radians(angle)) * speed]
        let points: [Vec2; 4] = [
            Vec2::new(self.pos.x+(self.angle+30.0).to_radians().cos()*30.0, self.pos.y+(self.angle+30.0).to_radians().sin()*30.0),
            Vec2::new(self.pos.x+(self.angle-30.0).to_radians().cos()*30.0, self.pos.y+(self.angle-30.0).to_radians().sin()*30.0),
            Vec2::new(self.pos.x+(self.angle+135.0).to_radians().cos()*30.0, self.pos.y+(self.angle+135.0).to_radians().sin()*30.0),
            Vec2::new(self.pos.x+(self.angle-135.0).to_radians().cos()*30.0, self.pos.y+(self.angle-135.0).to_radians().sin()*30.0)
        ];

        self.particles.draw(dt, default_operation);
        // draw the shadow of a car
        draw_triangle(points[0]+3.0, points[1]+3.0, points[2]+3.0, BLACK);
        draw_triangle(points[3]+3.0, points[1]+3.0, points[2]+3.0, BLACK);

        // draw the car
        draw_triangle(points[0], points[1], points[2], Color::new(1.0, 0.0, 0.0, 1.0));
        draw_triangle(points[3], points[1], points[2], Color::new(1.0, 0.0, 0.0, 1.0));
        
        // draw all the diffrent angles
        //draw_line(self.pos.x, self.pos.y, self.pos.x+(self.angle).to_radians().cos()*50.0, self.pos.y+(self.angle).to_radians().sin()*50.0, 3.0, BLUE);
        //draw_line(self.pos.x, self.pos.y, self.pos.x+(self.particle_direction).to_radians().cos()*50.0, self.pos.y+(self.particle_direction).to_radians().sin()*50.0, 3.0, BLACK);
        //draw_line(self.pos.x, self.pos.y, self.pos.x+(self.direction).to_radians().cos()*50.0, self.pos.y+(self.direction).to_radians().sin()*50.0, 3.0, GREEN);
    }
}
