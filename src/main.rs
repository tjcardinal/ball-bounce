use macroquad::prelude::*;

const GRAVITY: f32 = 2.2;

#[macroquad::main("BallBounce")]
async fn main() {
    let mut balls = vec![
        Ball::new(Circle::new(50.0, 50.0, 50.0), Vec2::new(0.0, 0.0)),
        Ball::new(Circle::new(50.0, 200.0, 50.0), Vec2::new(0.0, 0.0)),
        Ball::new(Circle::new(150.0, 50.0, 50.0), Vec2::new(-12.0, -2.0)),
        Ball::new(Circle::new(250.0, 50.0, 50.0), Vec2::new(-12.0, -2.0)),
        Ball::new(Circle::new(350.0, 50.0, 50.0), Vec2::new(-12.0, -2.0)),
    ];
    let mut walls = vec![
        Rect::new(0.4, 255.0, 50.0, 10.0),
        Rect::new(500.4, 255.0, 50.0, 10.0),
    ];

    loop {
        update_ball_velocity(&mut balls, &mut walls);
        update_pos(&mut balls, &mut walls);
        draw(&balls, &walls).await;
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

fn update_ball_velocity(balls: &mut [Ball], walls: &mut [Rect]) {
    for i in 0..balls.len() {
        // Edge collision
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

        // Wall collision
        for j in 0..walls.len() {
            if balls[i].cir.overlaps_rect(&walls[j]) {}
        }

        // Ball collision
        for j in (i + 1)..balls.len() {
            if balls[i].cir.overlaps(&balls[j].cir) {
                let jmod = (balls[j].cir.point() - balls[i].cir.point()).normalize()
                    * 0.5
                    * balls[i].vel.length();

                let imod = (balls[i].cir.point() - balls[j].cir.point()).normalize()
                    * 0.5
                    * balls[j].vel.length();

                balls[i].vel = (balls[i].vel - jmod) * 0.5 + imod;
                balls[j].vel = (balls[j].vel - imod) * 0.5 + jmod;
            }
        }
    }
}

fn update_pos(balls: &mut [Ball], walls: &mut [Rect]) {
    for ball in balls {
        ball.cir = ball.cir.offset(ball.vel);
    }

    for wall in walls {
        wall.offset(Vec2::new(0.00001, 0.00001));
    }
}

async fn draw(balls: &[Ball], walls: &[Rect]) {
    clear_background(BLACK);

    for ball in balls {
        draw_circle(ball.cir.x, ball.cir.y, ball.cir.r, BLUE);
    }

    for wall in walls {
        draw_rectangle(wall.x, wall.y, wall.w, wall.h, ORANGE);
    }

    next_frame().await;
}
