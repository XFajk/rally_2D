use macroquad::prelude::*;
use crate::effects::{ShapeParticles, Shapes, default_operation};
use crate::rand::*;

pub struct Car {
    pub vel: f32,
    pub max_vel: f32,
    pub acceleration: f32,
    pub angle: f32,
    pub direction: f32,
    pub max_direction: f32,
    pub particles: ShapeParticles,

    pub pos: Vec2
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
            particles: ShapeParticles::new(Shapes::Circle, 0.0)
        }
    }

    pub fn update(&mut self, dt: f32) {

        // movement
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

        if self.vel != 0.0 {
            self.angle += ((self.direction - self.angle) / 10.0)*dt;
        }
        if self.vel < 0.1 && -0.1 < self.vel {
            self.vel = 0.0;
        }

        // rotation
        if is_key_down(KeyCode::A) && !(self.direction < self.angle-self.max_direction)  {
            self.direction -= 5.0*dt;
        }
        if is_key_down(KeyCode::D) && !(self.direction > self.angle+self.max_direction) {
            self.direction += 5.0*dt
        }

        self.pos += Vec2::new(
            self.direction.to_radians().cos()*self.vel,
            self.direction.to_radians().sin()*self.vel
        ) * dt;

    }

    pub fn draw(&mut self, dt: f32) {
        // [math.cos(math.radians(angle)) * speed, math.sin(math.radians(angle)) * speed]
        let points: [Vec2; 4] = [
            Vec2::new(self.pos.x+(self.angle+30.0).to_radians().cos()*30.0, self.pos.y+(self.angle+30.0).to_radians().sin()*30.0),
            Vec2::new(self.pos.x+(self.angle-30.0).to_radians().cos()*30.0, self.pos.y+(self.angle-30.0).to_radians().sin()*30.0),
            Vec2::new(self.pos.x+(self.angle+135.0).to_radians().cos()*30.0, self.pos.y+(self.angle+135.0).to_radians().sin()*30.0),
            Vec2::new(self.pos.x+(self.angle-135.0).to_radians().cos()*30.0, self.pos.y+(self.angle-135.0).to_radians().sin()*30.0)
        ];

        if self.vel > 0.0 || self.vel < 0.0 {
            self.particles.add(
                self.pos,
                gen_range(self.angle - 180.0 - 40.0, self.angle - 180.0 + 40.0),
                gen_range(1.0, 3.0),
                gen_range(10.0, 12.0),
                0.2,
                GRAY
            );
        }

        self.particles.draw(dt, default_operation);

        draw_triangle(points[0], points[1], points[2], Color::new(1.0, 0.0, 0.0, 1.0));
        draw_triangle(points[3], points[1], points[2], Color::new(1.0, 0.0, 0.0, 1.0));
        draw_line(self.pos.x, self.pos.y, self.pos.x+(self.angle).to_radians().cos()*50.0, self.pos.y+(self.angle).to_radians().sin()*50.0, 3.0, BLUE);
        draw_line(self.pos.x, self.pos.y, self.pos.x+(self.direction).to_radians().cos()*50.0, self.pos.y+(self.direction).to_radians().sin()*50.0, 3.0, GREEN);
    }
}