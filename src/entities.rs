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
    pub back_direction: f32,
    pub max_direction: f32,
    pub particles: ShapeParticles,

    pub pos: Vec2,

    angle_step: f32,
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
            back_direction: angle-180.0,
            max_direction: 30.0,
            particles: ShapeParticles::new(Shapes::Hexagon, 0.0),
            particle_direction: angle-180.0,
            breaking_speed: 0.1,
            car_angle: angle,
            angle_step: 5.0,
            car_state: CarState::NotBreaking,
        }
    }

    pub fn update(&mut self, dt: f32) {

        // movement and rotation
        match self.car_state {
            CarState::NotBreaking => {
                // movement forward
                if is_key_down(KeyCode::W) && self.vel < self.max_vel {                
                    self.vel += self.acceleration * dt;
                }
                else if !is_key_down(KeyCode::W) && self.vel > 0.1 {
                    self.vel -= self.acceleration*dt;
                    self.back_direction = self.angle-180.0;
                }

                // movement backward
                if is_key_down(KeyCode::S) && self.vel > -self.max_vel {
                    self.vel -= self.acceleration * dt;
                }
                else if !is_key_down(KeyCode::S) && self.vel < -0.1 {
                    self.vel += self.acceleration*dt;
                    self.direction = self.angle;
                }

                // rotation
                if is_key_down(KeyCode::A) && !(self.direction < self.angle-self.max_direction)  {
                    self.direction -= self.angle_step*dt;
                }
                else if is_key_down(KeyCode::A) && (self.direction < self.angle-self.max_direction-self.angle_step) {
                    self.direction = self.angle-self.max_direction;
                }

                if is_key_down(KeyCode::D) && !(self.direction > self.angle+self.max_direction) {
                    self.direction += self.angle_step*dt;
                }
                else if is_key_down(KeyCode::D) && (self.direction > self.angle+self.max_direction+self.angle_step) {
                    self.direction = self.angle+self.max_direction;
                }

                if is_key_down(KeyCode::A) && !(self.back_direction > self.angle-180.0+self.max_direction)  {
                    self.back_direction += self.angle_step*dt;
                }
                else if is_key_down(KeyCode::A) && (self.back_direction > self.angle-180.0+self.max_direction+self.angle_step) {
                    self.back_direction = self.angle-180.0+self.max_direction
                }

                if is_key_down(KeyCode::D) && !(self.back_direction < self.angle-180.0-self.max_direction) {
                    self.back_direction -= self.angle_step*dt;
                }
                else if is_key_down(KeyCode::D) && (self.back_direction < self.angle-180.0-self.max_direction-self.angle_step) {
                    self.back_direction = self.angle-180.0-self.max_direction;
                }

                self.particle_direction = self.angle-180.0;


            }
            _ => {}
        }

        // rotates the angle
        if self.vel > 0.0 {
            self.angle += (self.direction - self.angle)/10.0*dt;
        }
        else if self.vel < 0.0 {
            self.angle += ((self.back_direction+180.0) - self.angle)/10.0*dt;
        }
        if self.vel < 0.1 && -0.1 < self.vel {
            self.vel = 0.0;
        }

        // breaking
        if is_key_down(KeyCode::Space) {
            if self.vel > 0.0 {
                self.vel -= self.breaking_speed * dt;
            }
            else if self.vel < 0.0 {
                self.vel += self.breaking_speed * dt;
            }
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
                if self.vel >= 0.1 {
                    self.pos += Vec2::new(
                        self.direction.to_radians().cos()*self.vel,
                        self.direction.to_radians().sin()*self.vel
                    ) * dt;
                }
                else if self.vel < 0.0 {
                    self.pos += Vec2::new(
                        self.back_direction.to_radians().cos()*(self.vel*-1.0),
                        self.back_direction.to_radians().sin()*(self.vel*-1.0)
                    ) * dt;
                }
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
                self.vel = self.vel;
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

        // caping the angles
        if self.angle > 360.0 {
            self.angle -= 360.0;
            self.direction -= 360.0;
            self.back_direction -= 360.0;
        }
        else if self.angle < -360.0 {
            self.angle += 360.0;
            self.direction += 360.0;
            self.back_direction += 360.0;
        }

        if self.car_angle > 360.0 {
            self.car_angle -= 360.0;
        }
        else if self.back_direction < -360.0 {
            self.car_angle += 360.0;
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

        let mut front_wheel_points: [[Vec2; 4]; 2] = [[Vec2::new(self.pos.x, self.pos.y); 4]; 2]; 
        let mut back_wheel_points: [[Vec2; 4]; 2] = [[Vec2::new(self.pos.x, self.pos.y); 4]; 2];
        

        self.particles.draw(dt, default_operation);

        // draw the shadow of a car
        draw_triangle(points[0]+3.0, points[1]+3.0, points[2]+3.0, BLACK);
        draw_triangle(points[3]+3.0, points[1]+3.0, points[2]+3.0, BLACK);

        // draw wheels
        {
            let mut index: usize = 0;
            let offset: f32 = 2.0;
            for wheel in front_wheel_points.iter_mut() {
                *wheel = [
                    Vec2::new(points[index].x+(self.direction+30.0).to_radians().cos()*8.0, points[index].y+(self.direction+30.0).to_radians().sin()*8.0),
                    Vec2::new(points[index].x+(self.direction-30.0).to_radians().cos()*8.0, points[index].y+(self.direction-30.0).to_radians().sin()*8.0),
                    Vec2::new(points[index].x+(self.direction+160.0).to_radians().cos()*8.0, points[index].y+(self.direction+160.0).to_radians().sin()*8.0),
                    Vec2::new(points[index].x+(self.direction-160.0).to_radians().cos()*8.0, points[index].y+(self.direction-160.0).to_radians().sin()*8.0)
                        
                ];
                draw_triangle(wheel[0]+3.0+offset, wheel[1]+3.0+offset, wheel[2]+3.0+offset, BLACK);
                draw_triangle(wheel[3]+3.0+offset, wheel[1]+3.0+offset, wheel[2]+3.0+offset, BLACK);
                draw_triangle(wheel[0]+offset, wheel[1]+offset, wheel[2]+offset, GRAY);
                draw_triangle(wheel[3]+offset, wheel[1]+offset, wheel[2]+offset, GRAY);
                index += 1;
            }
        }
        {
            let mut index: usize = 2;
            let offset: f32 = 2.0;
            for wheel in back_wheel_points.iter_mut() {
                *wheel = [
                    Vec2::new(points[index].x+(self.angle+30.0).to_radians().cos()*8.0, points[index].y+(self.angle+30.0).to_radians().sin()*8.0),
                    Vec2::new(points[index].x+(self.angle-30.0).to_radians().cos()*8.0, points[index].y+(self.angle-30.0).to_radians().sin()*8.0),
                    Vec2::new(points[index].x+(self.angle+160.0).to_radians().cos()*8.0, points[index].y+(self.angle+160.0).to_radians().sin()*8.0),
                    Vec2::new(points[index].x+(self.angle-160.0).to_radians().cos()*8.0, points[index].y+(self.angle-160.0).to_radians().sin()*8.0)
                ];
                draw_triangle(wheel[0]+3.0+offset, wheel[1]+3.0+offset, wheel[2]+3.0+offset, BLACK);
                draw_triangle(wheel[3]+3.0+offset, wheel[1]+3.0+offset, wheel[2]+3.0+offset, BLACK);
                draw_triangle(wheel[0]+offset, wheel[1]+offset, wheel[2]+offset, GRAY);
                draw_triangle(wheel[3]+offset, wheel[1]+offset, wheel[2]+offset, GRAY);
                index += 1;
            }
        }

        // draw the car
        draw_triangle(points[0], points[1], points[2], Color::new(1.0, 0.0, 0.0, 1.0));
        draw_triangle(points[3], points[1], points[2], Color::new(1.0, 0.0, 0.0, 1.0));
        
        // draw all the different angles
        // draw_line(self.pos.x, self.pos.y, self.pos.x+self.back_direction.to_radians().cos()*50.0, self.pos.y+self.back_direction.to_radians().sin()*50.0, 3.0, BLUE);
        // draw_line(self.pos.x, self.pos.y, self.pos.x+(self.angle).to_radians().cos()*50.0, self.pos.y+(self.angle).to_radians().sin()*50.0, 3.0, BLACK);
        // draw_line(self.pos.x, self.pos.y, self.pos.x+(self.direction).to_radians().cos()*50.0, self.pos.y+(self.direction).to_radians().sin()*50.0, 3.0, GREEN);
    }
}
