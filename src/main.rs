use macroquad::logging as log;
use macroquad::prelude::*;
//use macroquad::texture;
//use macroquad::{
//    audio::{self, PlaySoundParams, Sound},
//    experimental::coroutines::{start_coroutine, Coroutine},
//};
use quad_rand as qrand;

const PROJECTION_WIDTH: f32 = 1600.0;
const PROJECTION_HEIGHT: f32 = 900.0;
const MAX_BUNNIES: usize = 500000; // 500K bunnies limit
const MAX_BATCH_ELEMENTS: usize = 8192;

#[derive(Clone)]
struct Bunny {
    position: Vec2,
    speed: Vec2,
    color: Color,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Scary Maze Demo".to_string(),
        window_width: 1600,
        window_height: 900,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    log::debug!("Start game");

    let camera = macroquad::camera::Camera2D::from_display_rect(macroquad::math::Rect::new(
        0.0,
        0.0,
        PROJECTION_WIDTH,
        PROJECTION_HEIGHT,
    ));
    macroquad::camera::set_camera(&camera);

    let mut bunnies = vec![Bunny { position: Vec2::new(0.0, 0.0), speed: Vec2::new(0.0, 0.0), color: WHITE }; MAX_BUNNIES];
    let mut bunnies_count = 0;
    let tex_bunny = load_texture("resources/wabbit_alpha.png").await.unwrap();

//     loop {
//         clear_background(GRAY);
//         draw_circle(PROJECTION_WIDTH/2f32, PROJECTION_HEIGHT/2f32, 4f32, RED);
//         next_frame().await;
//     }
    loop {
        if is_mouse_button_down(MouseButton::Left) {
            for _ in 0..100 {
                if bunnies_count < MAX_BUNNIES {
                    bunnies[bunnies_count].position = mouse_position().into();
                    bunnies[bunnies_count].speed = Vec2::new(qrand::gen_range(-250, 250) as f32 /60.0, qrand::gen_range(-250, 250) as f32 /60.0);
                    bunnies[bunnies_count].color = Color::from_rgba(qrand::gen_range(50, 240).into(), qrand::gen_range(80, 240).into(), qrand::gen_range(100, 240).into(), 255);
                    bunnies_count += 1;
                }
            }
        }

        for i in 0..bunnies_count {
            bunnies[i].position.x += bunnies[i].speed.x;
            bunnies[i].position.y += bunnies[i].speed.y;

            if (bunnies[i].position.x + tex_bunny.width() as f32 / 2.0) > PROJECTION_WIDTH as f32 || (bunnies[i].position.x + tex_bunny.width() as f32 / 2.0) < 0.0 {
                bunnies[i].speed.x *= -1.0;
            }
            if (bunnies[i].position.y + tex_bunny.height() as f32 / 2.0) > PROJECTION_HEIGHT as f32 || (bunnies[i].position.y + tex_bunny.height() as f32 / 2.0 - 40.0) < 0.0 {
                bunnies[i].speed.y *= -1.0;
            }
        }

        clear_background(WHITE);

        for i in 0..bunnies_count {
            draw_texture_ex(tex_bunny, bunnies[i].position.x, bunnies[i].position.y, bunnies[i].color, DrawTextureParams::default());
        }

        draw_rectangle(0f32, 0f32, PROJECTION_WIDTH as f32, 30f32, BLACK);
        draw_text(format!("bunnies: {}", bunnies_count).as_str(), 120.0, 20.0, 30.0, GREEN);
        draw_text(&format!("batched draw calls: {}", 1 + bunnies_count/MAX_BATCH_ELEMENTS).as_str(), 320.0, 20.0, 30.0, MAROON);
        draw_text(format!("FPS: {}", get_fps()).as_str(), 10.0, 20.0, 30.0, BLUE);


        next_frame().await;
    }
}
