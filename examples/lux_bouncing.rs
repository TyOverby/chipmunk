extern crate chipmunk;
extern crate lux;

use lux::prelude::*;
use lux::game::*;

use chipmunk::space::Space;
use chipmunk::body::Body;
use chipmunk::shape::Shape;
use chipmunk::util::*;

struct MyGame {
    space: Space,
    ball_body: Body
}

impl Game for MyGame {
    fn update(&mut self, dt: f32, window: &mut Window, _events: &mut EventIterator) -> LuxResult<()> {
        self.space.step(dt as f64);
        Ok(())
    }

    fn render(&mut self, lag: f32, _window: &mut Window, frame: &mut Frame) -> LuxResult<()> {
        let (x, y) = self.ball_body.position();
        frame.circle(50.0, y as f32 * 10.0, 10.0).fill();
        println!("{}, {}", x, y);
        Ok(())
    }

    fn show_fps(&self, window: &Window) -> bool {
        window.is_key_pressed(' ')
    }
}

fn main() {
    let gravity = (0.0, -100.0);
    let floor_friction = 1.0;
    let ball_friction = 0.7;
    let ball_radius = 5.0;
    let ball_mass = 1.0;
    let ball_pos = (0.0, 100.0);
    let ball_moment = moment_of_circle(ball_mass, ball_radius, 0.0);
    let floor_start = (-20.0, 0.0);
    let floor_end = (20.0, 0.0);
    let floor_radius = 0.0;
    let zero = (0.0, 0.0);
    let time_step = 1.0 / 60.0;

    // The space contains everything in the simulation.
    let mut space = Space::new();
    space.set_gravity(gravity.0, gravity.1);

    // Set up a floor for our ball to bounce off of.
    let mut floor_body = Body::new_static();
    let mut floor_shape = Shape::new_segment(
        &mut floor_body, floor_start, floor_end, floor_radius);

    floor_shape.set_friction(floor_friction);
    space.add_body(&mut floor_body);
    space.add_shape(&mut floor_shape);


    // Add a bouncing ball to the scene.
    let mut ball_body = Body::new(ball_mass, ball_moment);
    let mut ball_shape = Shape::new_circle(&mut ball_body, ball_radius, zero);

    ball_body.set_position(ball_pos.0, ball_pos.1);
    ball_shape.set_friction(ball_friction);

    space.add_body(&mut ball_body);
    space.add_shape(&mut ball_shape);

    let game = MyGame {
        space: space,
        ball_body: ball_body
    };

    game.run_until_end().unwrap();
}

