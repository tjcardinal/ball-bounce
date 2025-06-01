use macroquad::prelude::*;
use std::f32::consts::PI;

const GRAVITY: f32 = 2.2;
const ROTATION: f32 = PI / 200.;

#[macroquad::main("BallBounce")]
async fn main() {
    let mut balls = vec![
        Ball::new(
            Circle::new(screen_width() / 2. - 100., screen_height() / 2. + 2., 10.),
            Vec2::new(1., 1.),
        ),
        Ball::new(
            Circle::new(screen_width() / 2. - 80., screen_height() / 2. + 4., 10.),
            Vec2::new(2., 2.),
        ),
        Ball::new(
            Circle::new(screen_width() / 2. - 60., screen_height() / 2. + 6., 10.),
            Vec2::new(-12., -5.),
        ),
    ];

    let mut hexagon =
        RegularHexagon::new(Vec2::new(screen_width() / 2., screen_height() / 2.), 100.);

    loop {
        update_ball_velocity(&mut balls, &hexagon);
        update_pos(&mut balls, &mut hexagon);
        // fix overlaps
        draw(&balls, &hexagon).await;
    }
}

#[derive(Debug)]
struct Ball {
    cir: Circle,
    vel: Vec2,
}

impl Ball {
    fn new(cir: Circle, vel: Vec2) -> Self {
        Self { cir, vel }
    }
}

#[derive(Debug)]
struct RegularHexagon {
    center: Vec2,
    vertices: [Vec2; 6],
    radius: f32,
}

impl RegularHexagon {
    fn new(center: Vec2, radius: f32) -> Self {
        let max_rads = PI * 2.;
        let vertices = [
            polar_to_cartesian(radius, max_rads * 1. / 6.) + center,
            polar_to_cartesian(radius, max_rads * 2. / 6.) + center,
            polar_to_cartesian(radius, max_rads * 3. / 6.) + center,
            polar_to_cartesian(radius, max_rads * 4. / 6.) + center,
            polar_to_cartesian(radius, max_rads * 5. / 6.) + center,
            polar_to_cartesian(radius, max_rads * 6. / 6.) + center,
        ];

        Self {
            center,
            vertices,
            radius,
        }
    }

    fn rotate(&mut self, radians: f32) {
        let angle = Vec2::from_angle(radians);

        for i in 0..self.vertices.len() {
            self.vertices[i] = angle.rotate(self.vertices[i] - self.center) + self.center;
        }
    }
}

fn update_ball_velocity(balls: &mut [Ball], hexagon: &RegularHexagon) {
    for i in 0..balls.len() {
        // Ball collision
        for j in (i + 1)..balls.len() {
            if balls[i].cir.overlaps(&balls[j].cir) {
                let jmod = (balls[j].cir.point() - balls[i].cir.point()).normalize()
                    * 0.5
                    * balls[i].vel.length();

                let imod = (balls[i].cir.point() - balls[j].cir.point()).normalize()
                    * 0.5
                    * balls[j].vel.length();

                let old_i = balls[i].vel;
                let old_j = balls[j].vel;

                balls[i].vel = (balls[i].vel - jmod) * 0.5 + imod;
                balls[j].vel = (balls[j].vel - imod) * 0.5 + jmod;
            }
        }

        // gravity
        balls[i].vel.y += GRAVITY;

        // Hex
        // for j in 0..6 {
        //     let new_direction = hexagon.center - balls[i].cir.point().normalize();
        //     let new_vel = balls[i].vel.abs() * 0.8 * new_direction;
        //
        // }
        //
        // test with it as a circle
        if hexagon.radius < (balls[i].cir.point() - hexagon.center).length() {
            let new_direction = (hexagon.center - balls[i].cir.point()).normalize();
            let new_vel = balls[i].vel.abs() * 0.9 * new_direction;
            balls[i].vel = new_vel;
        }
    }
}

fn update_pos(balls: &mut [Ball], hexagon: &mut RegularHexagon) {
    for ball in balls {
        ball.cir = ball.cir.offset(ball.vel);
    }

    hexagon.rotate(ROTATION);
}

async fn draw(balls: &[Ball], hexagon: &RegularHexagon) {
    clear_background(BLACK);

    for ball in balls {
        draw_circle(ball.cir.x, ball.cir.y, ball.cir.r, BLUE);
    }

    for i in 0..6 {
        let v1 = hexagon.vertices[i];
        let v2 = if i == 5 {
            hexagon.vertices[0]
        } else {
            hexagon.vertices[i + 1]
        };
        draw_line(v1.x, v1.y, v2.x, v2.y, 5., RED);
    }

    next_frame().await;
}
