use macroquad::prelude::*;

const GRAVITY: f32 = 1.2;

#[macroquad::main("BallBounce")]
async fn main() {
    let mut balls = vec![
        Ball::new(Circle::new(50.0, 50.0, 50.0), Vec2::new(20.0, 51.0)),
        Ball::new(Circle::new(250.0, 250.0, 50.0), Vec2::new(-12.0, -2.0)),
    ];
    loop {
        update_velocity(&mut balls);
        update_pos(&mut balls);
        draw(&balls).await;
        print(&balls);
    }
}

#[derive(Debug)]
struct Ball {
    cir: Circle,
    vel: Vec2,
}

impl Ball {
    fn new(cir: Circle, vel: Vec2) -> Self {
        Ball { cir, vel }
    }
}

fn update_velocity(balls: &mut [Ball]) {
    for i in 0..balls.len() {
        // Ball collision
        for j in (i + 1)..balls.len() {
            if balls[i].cir.overlaps(&balls[j].cir) {
                // balls[i].vel.x *= 0.5;
                // balls[i].vel.y *= 0.5;
                // balls[j].vel.x *= 0.5;
                // balls[j].vel.y *= 0.5;
                //
                // balls[i].vel.x += balls[j].vel.x;
                // balls[i].vel.y += balls[j].vel.x;
                //
                // balls[j].vel.x += balls[i].vel.x;
                // balls[j].vel.y += balls[i].vel.x;
            }
        }

        // Wall collision
        // top
        if balls[i].cir.contains(&Vec2::new(balls[i].cir.x, 0.0)) {
            balls[i].vel.y = 0.90 * balls[i].vel.y.abs();
        }
        // bottom
        if balls[i]
            .cir
            .contains(&Vec2::new(balls[i].cir.x, screen_height()))
        {
            balls[i].vel.y = -0.70 * balls[i].vel.y.abs();
            // drag
            balls[i].vel.x *= 0.99;
        } else {
            // gravity
            balls[i].vel.y += GRAVITY;
        }
        // left
        if balls[i].cir.contains(&Vec2::new(0.0, balls[i].cir.y)) {
            balls[i].vel.x = 0.90 * balls[i].vel.x.abs();
        }
        // right
        if balls[i]
            .cir
            .contains(&Vec2::new(screen_width(), balls[i].cir.y))
        {
            balls[i].vel.x = -0.90 * balls[i].vel.x.abs();
        }
    }
}

fn update_pos(balls: &mut [Ball]) {
    for ball in balls {
        ball.cir = ball.cir.offset(ball.vel);
    }
}

async fn draw(balls: &[Ball]) {
    clear_background(BLACK);

    for ball in balls {
        draw_circle(ball.cir.x, ball.cir.y, ball.cir.r, BLUE);
    }
    next_frame().await;
}

fn print(balls: &[Ball]) {
    println!("{:?}", balls[0]);
}
