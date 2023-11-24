#![deny(unsafe_code)]
#![recursion_limit = "128"]
#![allow(unused_imports, unused_labels)]

pub mod comp;
pub mod consts;
pub mod physics;
pub mod state;
pub mod sys;

use comp::builder_ext::EntityBuilderExt as _;
use macroquad::{
    color::{LIGHTGRAY, RED},
    prelude as mqp,
    shapes::{draw_circle, draw_circle_lines, draw_line, draw_rectangle, draw_rectangle_lines},
    text::draw_text,
};
use physics::Physics;
use specs::{Builder as _, DispatcherBuilder, Join as _, WorldExt as _};
use sys::DispatcherBuilderExt as _;
use vek::Vec2;

#[macroquad::main("Physics Engine")]
async fn main() {
    let mut world = specs::World::new();
    let dispatcher_builder = DispatcherBuilder::new().register_systems();
    let mut dispatcher = dispatcher_builder.build();

    world.register::<comp::Pos>();
    world.register::<comp::PrevPos>();
    world.register::<comp::Vel>();
    world.register::<comp::SimVel>();
    world.register::<comp::Body>();
    world.register::<comp::ZIdx>();
    world.register::<comp::Mass>();

    /*  world
    .create_entity()
    .with_position(vek::Vec2 { x: 150.0, y: 150.0 })
    .with(comp::Body::RectangleOutline(200.0, 400.0, 5.0))
    .build();*/

    'game: loop {
        'update: {
            dispatcher.dispatch(&world);
            world.maintain();

            if macroquad::input::is_mouse_button_down(mqp::MouseButton::Left) {
                world
                    .create_entity()
                    .with_position(Vec2 {
                        x: macroquad::rand::gen_range(
                            10.0,
                            macroquad::window::screen_width() - 10.0,
                        ),
                        y: -20.0,
                    })
                    .with_physics(Physics::Default)
                    .with(comp::Body::Circle(20.0))
                    .build();
            }

            break 'update;
        };

        let pos_storage = world.read_storage::<comp::Pos>();
        let body_storage = world.read_storage::<comp::Body>();

        'draw: for (pos, body) in (&pos_storage, &body_storage).join() {
            match body {
                comp::Body::Circle(r) => draw_circle(pos.0.x, pos.0.y, r.clone(), LIGHTGRAY),
                comp::Body::CircleOutline(r, t) => {
                    draw_circle_lines(pos.0.x, pos.0.y, r.clone(), t.clone(), LIGHTGRAY)
                }
                comp::Body::Rectangle(w, h) => {
                    draw_rectangle(pos.0.x, pos.0.y, w.clone(), h.clone(), LIGHTGRAY)
                }
                comp::Body::RectangleOutline(w, h, t) => draw_rectangle_lines(
                    pos.0.x,
                    pos.0.y,
                    w.clone(),
                    h.clone(),
                    t.clone(),
                    LIGHTGRAY,
                ),
                comp::Body::Line(target, t) => {
                    draw_line(pos.0.x, pos.0.y, target.x, target.y, t.clone(), LIGHTGRAY)
                }
                _ => todo!(),
            }
        }

        draw_text(
            format!(
                "Entity Count: {} | FPS: {:2} | dt: {:2} | Time: {:2}",
                world.entities().join().count(),
                macroquad::time::get_fps(),
                macroquad::time::get_frame_time(),
                macroquad::time::get_time()
            )
            .as_str(),
            10.0,
            25.0,
            40.0,
            RED,
        );

        mqp::next_frame().await;
    }
}
