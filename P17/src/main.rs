use macroquad::prelude::*;
mod bullet;
use crate::bullet::Bullet;

fn window_conf() -> Conf {
    Conf {
        window_title: "Lot pocisku".to_owned(),
        window_width: 1350,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut bullet = Bullet::new(45.0, 40.0);
    let mut trail: Vec<(f32, f32)> = vec![];

    // SKY

    let sky = load_texture("sky.png").await.unwrap();
    sky.set_filter(FilterMode::Nearest);

    // GRASS

    let grass_texture = load_texture("grass.png").await.unwrap();
    grass_texture.set_filter(FilterMode::Nearest);
    let tex_w = grass_texture.width();
    let tex_h = grass_texture.height();
    let target_h = 50.0;
    let target_w = tex_w * (target_h / tex_h);
    let tiles = (screen_width() / target_w).ceil() as i32 + 1;

    loop {
        clear_background(SKYBLUE);

        draw_texture_ex(
            &sky,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        for i in 0..tiles {
            draw_texture_ex(
                &grass_texture,
                i as f32 * target_w,
                screen_height() - target_h,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(target_w, target_h)),
                    ..Default::default()
                },
            );
        }

        let (x, y) = bullet.update();
        trail.push((x, y));

        for &(tx, ty) in &trail {
            draw_circle(tx, 750.0 - ty, 3.0, YELLOW);
        }

        draw_circle(x, 750.0 - y, 8.0, RED);

        next_frame().await;
    }
}
