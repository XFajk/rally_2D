mod entities;
mod effects;
mod utils;

use macroquad::prelude::*;

#[macroquad::main(window_conf())]
async fn main() {

    let zoom: f32 = 1.;
    let camera = Camera2D::from_display_rect(Rect::new(
        0.,
        0.,
        screen_width() / (zoom),
        screen_height() / (zoom),
    ));

    // Logic
    let mut debug: bool= false;

    // Assets

    // Entities
    let mut car: entities::Car = entities::Car::new(
        8.0, 0.1, 0.0, Vec2::new(400.0, 100.0)
    );

    'gameLoop: loop {

        let dt= get_frame_time()*60.0;

        car.update(dt);

        clear_background(WHITE);
        set_camera(&camera);

        car.draw(dt);

        if debug {
            draw_debug(vec![
                (String::from("FPS"), get_fps().to_string()),
                (String::from("ANGLE"), car.angle.to_string()),
                (String::from("DIRECTION"), car.direction.to_string()),
                (String::from("VELOCITY"), car.vel.to_string()),
                (String::from("PARTICLE_AMOUNT"), car.particles.objects.len().to_string()),
            ]);
        }
        next_frame().await;

        if is_key_pressed(KeyCode::Escape) {
            break 'gameLoop;
        }
        if is_key_pressed(KeyCode::F1) {
            if !debug {
                debug = true;
            } else {
                debug = false;
            }
        }
    }
    std::process::exit(0);
}

fn draw_debug(values: Vec<(String, String)>) {
    let mut i: f32 = 0.0;
    let mut x_offset: f32 = 5.;
    for val in values {
        i += 1.0;
        if i*12.0+2.0 > screen_height()-24.0 {
            x_offset += screen_width()-200.0;
        }
        let text: String = val.0 + ": " + val.1.as_str();
        draw_text(text.as_str(), x_offset, i*12.0+2.0, 20.0, BLACK)
    }
}

fn window_conf() -> Conf {
    Conf {
        window_resizable: false,
        window_title: "rally 2D".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

