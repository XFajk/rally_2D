use macroquad::prelude::*;

pub enum Shapes {
    Circle,
    Rectangle,
    Hexagon,
}

pub struct ShapeParticles {
    pub gravity: f32,
    pub objects: Vec<(Vec2, Vec2, f32, f32, Color)>,
    pub shape_type: Shapes
}

impl ShapeParticles {

    pub fn new(shape_type: Shapes, gravity: f32) -> Self {
        Self { gravity, shape_type, objects: Vec::new() }
    }

    pub fn add(&mut self, pos: Vec2, angle: f32, speed: f32, size: f32, decrement: f32, c: Color) {
        let vel: Vec2 = Vec2::new(angle.to_radians().cos()*speed, angle.to_radians().sin()*speed);
        self.objects.insert(0, (pos, vel, size, decrement, c));
    }

    pub fn draw(&mut self, dt: f32, operation: fn(x: &mut (Vec2, Vec2, f32, f32, Color), d: f32)) {
        match self.shape_type {
            Shapes::Circle => {
                let mut index: usize = 0;
                for _i in 0..self.objects.len() {
                    let particle: &mut (Vec2, Vec2, f32, f32, Color) = &mut self.objects[index];

                    // logic part
                    particle.0.x += particle.1.x * dt;
                    particle.0.y += particle.1.y * dt;
                    particle.2 -= particle.3 * dt;
                    operation(particle, dt);

                    // drawing part
                    draw_circle(particle.0.x, particle.0.y, particle.2, particle.4);

                    // removing the particle form the self.objects vector
                    if particle.2 <= 0.0 {
                        self.objects.pop();
                        index -= 1;
                    }
                    index += 1;
                }
            }
            Shapes::Rectangle => {
                let mut index: usize = 0;
                for _i in 0..self.objects.len() {
                    let particle: &mut (Vec2, Vec2, f32, f32, Color) = &mut self.objects[index];

                    // logic part
                    particle.0.x += particle.1.x * dt;
                    particle.0.y += particle.1.y * dt;
                    particle.2 -= particle.3 * dt;
                    operation(particle, dt);

                    // drawing part
                    draw_rectangle(particle.0.x, particle.0.y, particle.2, particle.2, particle.4);

                    // removing the particle form the self.objects vector
                    if particle.2 <= 0.0 {
                        self.objects.pop();
                        index -= 1;
                    }
                    index += 1;
                }
            }
            Shapes::Hexagon => {
                let mut index: usize = 0;
                for _i in 0..self.objects.len() - 1 {
                    let particle: &mut (Vec2, Vec2, f32, f32, Color) = &mut self.objects[index];

                    // logic part
                    particle.0.x += particle.1.x * dt;
                    particle.0.y += particle.1.y * dt;
                    particle.2 -= particle.3 * dt;
                    operation(particle, dt);

                    // drawing part
                    draw_hexagon(
                        particle.0.x, particle.0.y, particle.2, 0.0,
                        true, Color::new(0.0, 0.0, 0.0, 0.0),
                        particle.4
                    );

                    // removing the particle form the self.objects vector
                    if particle.2 <= 0.0 {
                        self.objects.pop();
                        index -= 1;
                    }
                    index -= 1;
                }
            }
        }
    }
}

pub fn default_operation(_x: &mut (Vec2, Vec2, f32, f32, Color), _dt: f32) {}