extern crate chipmunk;

use chipmunk::space::Space;
use chipmunk::body::Body;
use chipmunk::shape::Shape;
use chipmunk::util::*;

/*
 *
    let gravity = cpv(0.0, -100.0);
    let zero = cpv(0.0, 0.0);

    let space = cpSpaceNew();
    cpSpaceSetGravity(space, gravity);
    let ground = cpSegmentShapeNew((*space).staticBody,
                                   cpv(-20.0, 5.0),
                                   cpv(20.0, -5.0),
                                   0.0);
  cpShapeSetFriction(ground, 1.0);
  cpSpaceAddShape(space, ground);

  let radius = 5.0;
  let mass = 1.0;

  let moment = cpMomentForCircle(mass, 0.0, radius, zero);

  let ballbody = cpSpaceAddBody(space, cpBodyNew(mass, moment));
  cpBodySetPosition(ballbody, cpv(0.0, 15.0));

  let ballShape = cpSpaceAddShape(space, cpCircleShapeNew(ballbody, radius, zero));
  cpShapeSetFriction(ballShape, 0.7);

  let timeStep = 1.0 / 60.0;
  for i in 0 .. 60 {
      let time = timeStep * i as f32;
      let pos = cpBodyGetPosition(ballbody);
      let vel = cpBodyGetVelocity(ballbody);
      println!("Time: {:?}, Pos: {:?}, Vel: {:?}", time, pos, vel);
      cpSpaceStep(space, timeStep as f64);
  }
  */

fn main() {
    let gravity = (0.0, -100.0);
    let floor_friction = 1.0;
    let ball_friction = 0.7;
    let ball_radius = 5.0;
    let ball_mass = 1.0;
    let ball_pos = (0.0, 15.0);
    let ball_moment = moment_of_circle(ball_mass, ball_radius, 0.0);
    let floor_start = (-20.0, 5.0);
    let floor_end = (20.0, -5.0);
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

    let mut y_coords = vec![];


    // Run the simulation!
    for i in 0 .. 60 {
        let time = time_step * (i as f64);
        let pos = ball_body.position();
        let vel = ball_body.velocity();
        println!("t: {:?}, p: {:?}, v: {:?}", time, pos, vel);
        println!("{:?}", space.gravity());
        space.step(time_step);
        y_coords.push(pos.1);
    }

    let min = y_coords.iter().cloned().map(|a| a as i32).min_max().into_option().unwrap().0;

    for coord in y_coords {
        let coord = ((coord - min as f64) * 10.0) as usize;
        let s: String = ::std::iter::repeat(' ').take(coord).collect();
        println!("{}#", s);
    }

}

