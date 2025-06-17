use macroquad::prelude::*;
use std::f32::consts::PI;

const GRAVITY: f32 = 0.0;
// const GRAVITY: f32 = 1.1;
const ROTATION: f32 = PI / 200.;

#[macroquad::main("BallBounce")]
async fn main() {
    let x_center = screen_width() / 2.;
    let y_center = screen_height() / 2.;
    let hex_radius = 100.;

    let mut balls: Vec<_> = (0..0)
        .map(|_| {
            let cir_x = rand::gen_range(-hex_radius, hex_radius) + x_center;
            let cir_y = rand::gen_range(-hex_radius, hex_radius) + y_center;
            let vel_x = rand::gen_range(-10., 10.);
            let vel_y = rand::gen_range(-10., 10.);

            Ball::new(Circle::new(cir_x, cir_y, 10.), Vec2::new(vel_x, vel_y))
        })
        .collect();

    balls.push(Ball::new(
        Circle::new(x_center - 100., y_center + 100., 10.),
        Vec2::new(0., 0.),
    ));

    let mut hexagon = RegularHexagon::new(Vec2::new(x_center, y_center), hex_radius);

    loop {
        ball_movement(&mut balls);
        hex_movement(&mut hexagon);

        ball_collisions(&mut balls);
        hex_collisions(&mut balls, &hexagon);

        apply_gravity(&mut balls);

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
    // radius: f32,
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
            // radius,
        }
    }

    fn rotate(&mut self, radians: f32) {
        let angle = Vec2::from_angle(radians);

        for i in 0..self.vertices.len() {
            // self.vertices[i] = angle.rotate(self.vertices[i] - self.center) + self.center;
        }
    }

    fn lines(&self) -> [(Vec2, Vec2); 6] {
        let mut lines: [(Vec2, Vec2); 6] = Default::default();

        for (i, v1) in self.vertices.into_iter().enumerate() {
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
                let collision_vec = balls[j].cir.point() - balls[i].cir.point();
                let angle = collision_vec.normalize_or(Vec2::X);

                // Add in some extra to account for rounding errors
                let overlap_length =
                    balls[i].cir.r + balls[j].cir.r - collision_vec.length() + 1.0e-3;

                // Shift them away from each other
                balls[i].cir = balls[i].cir.offset(-angle * overlap_length * 0.5);
                balls[j].cir = balls[j].cir.offset(angle * overlap_length * 0.5);
                assert!(
                    !balls[i].cir.overlaps(&balls[j].cir),
                    "Balls still overlap:\n{:?}\n{:?}\nDistance: {}",
                    balls[i],
                    balls[j],
                    balls[i].cir.point().distance(balls[j].cir.point())
                );

                // Update velocities due to collision
                let b1_old_vel = balls[i].vel;
                let b2_old_vel = balls[j].vel;

                // Elastic collision with the same mass means all we need is to swap
                // the component of their velocities that is parallel to the collision
                let b1_new_vel = b2_old_vel.project_onto_normalized(angle)
                    + b1_old_vel.reject_from_normalized(angle);
                let b2_new_vel = b1_old_vel.project_onto_normalized(angle)
                    + b2_old_vel.reject_from_normalized(angle);

                // total energy of old should equal total energy of new
                let old_energy = b1_old_vel.length_squared() + b2_old_vel.length_squared();
                let new_energy = b1_new_vel.length_squared() + b2_new_vel.length_squared();
                assert!(
                    (old_energy - new_energy).abs() <= 1.0e-3,
                    "Energy not conserved: {old_energy} {new_energy}"
                );

                balls[i].vel = b1_new_vel;
                balls[j].vel = b2_new_vel;
            }
        }
    }
}

fn hex_collisions(balls: &mut [Ball], hex: &RegularHexagon) {
    for ball in balls {
        let ball_center = ball.cir.point();
        let (start, end) = hex.lines().iter().fold(
            (Vec2::INFINITY, Vec2::INFINITY),
            |(min_start, min_end), (start, end)| {
                let cur =
                    min_start.distance_squared(ball_center) + min_end.distance_squared(ball_center);
                let new = start.distance_squared(ball_center) + end.distance_squared(ball_center);
                if new < cur {
                    (*start, *end)
                } else {
                    (min_start, min_end)
                }
            },
        );

        let midpoint = start.midpoint(end);
        let midpoint_vec = midpoint - hex.center;
        let ball_vec = ball.cir.point() - hex.center;
        let ball_proj = ball_vec.project_onto(midpoint_vec);

        if midpoint_vec.length_squared() < ball_proj.length_squared() {}
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
