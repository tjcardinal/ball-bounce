use macroquad::prelude::*;
use std::f32::consts::PI;

const GRAVITY: f32 = 1.1;
const ROTATION: f32 = PI / 200.;

#[macroquad::main("BallBounce")]
async fn main() {
    let x_center = screen_width() / 2.;
    let y_center = screen_height() / 2.;
    let hex_radius = 100.;

    let mut balls: Vec<_> = (0..10)
        .map(|_| {
            let cir_x = rand::gen_range(-hex_radius, hex_radius) + x_center;
            let cir_y = rand::gen_range(-hex_radius, hex_radius) + y_center;
            let vel_x = rand::gen_range(-10., 10.);
            let vel_y = rand::gen_range(-10., 10.);

            Ball::new(Circle::new(cir_x, cir_y, 10.), Vec2::new(vel_x, vel_y))
        })
        .collect();

    let mut hexagon = RegularHexagon::new(Vec2::new(x_center, y_center), hex_radius);

    loop {
        ball_movement(&mut balls);
        hex_movement(&mut hexagon);

        ball_collisions(&mut balls);
        hex_collisions(&mut balls, &hexagon);

        apply_gravity(&mut balls);

        draw(&balls, &hexagon).await;

        // println!("{} {}", balls[3].cir.point(), balls[3].vel);

        // fix overlaps
        // for each each ball
        //      for each other ball
        //          if they overlap
        //          move them away from each other's center the required distance by the ratio of their velocity
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

    fn lines(&self) -> [(Vec2, Vec2); 6] {
        let mut lines: [(Vec2, Vec2); 6] = Default::default();

        for i in 0..self.vertices.len() {
            let v1 = self.vertices[i];
            let v2 = if i == 5 {
                self.vertices[0]
            } else {
                self.vertices[i + 1]
            };
            lines[i] = (v1, v2);
        }

        lines
    }
}

fn ball_movement(balls: &mut [Ball]) {
    for ball in balls {
        ball.cir = ball.cir.offset(ball.vel);
    }
}

fn hex_movement(hex: &mut RegularHexagon) {
    hex.rotate(ROTATION);
}

fn ball_collisions(balls: &mut [Ball]) {
    for i in 0..balls.len() {
        for j in (i + 1)..balls.len() {
            if balls[i].cir.overlaps(&balls[j].cir) {
                let vec_between_circles = balls[j].cir.point() - balls[i].cir.point();
                let overlap_ratio =
                    1. - vec_between_circles.length() / (balls[i].cir.r + balls[j].cir.r);
                let offset = if overlap_ratio == 1.0 {
                    Vec2::ONE
                } else {
                    vec_between_circles * overlap_ratio
                };

                // Shift them away from each other
                balls[i].cir = balls[i].cir.offset(offset * -0.5);
                balls[j].cir = balls[j].cir.offset(offset * 0.5);

                // Update velocities
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

fn hex_collisions(balls: &mut [Ball], hex: &RegularHexagon) {
    for ball in balls {
        let dist_from_center = (ball.cir.point() - hex.center).length();
        if hex.radius <= dist_from_center {
            // Shift it towards center of hex
            ball.cir
                .move_to(hex.center.move_towards(ball.cir.point(), hex.radius));

            // Refract its velocity
            ball.vel *= -0.9;

            // Apply friction?
        }
    }
}

fn apply_gravity(balls: &mut [Ball]) {
    for ball in balls {
        ball.vel.y += GRAVITY;
    }
}

async fn draw(balls: &[Ball], hexagon: &RegularHexagon) {
    clear_background(BLACK);

    for ball in balls {
        draw_circle(ball.cir.x, ball.cir.y, ball.cir.r, BLUE);
    }

    for line in hexagon.lines() {
        draw_line(line.0.x, line.0.y, line.1.x, line.1.y, 5., RED);
    }
    next_frame().await;
}
