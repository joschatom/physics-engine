mod comp;
mod state;
mod sys;

use macroquad::{prelude as mqp, color::LIGHTGRAY, shapes::{draw_circle, draw_rectangle, draw_poly, draw_circle_lines, draw_rectangle_lines, draw_line}};
use specs::{prelude as ecs, WorldExt as _, Join as _, Builder as _, DispatcherBuilder};
use sys::DispatcherBuilderExt as _;

#[macroquad::main("Physics Engine")]
async fn main() {
    let mut world = ecs::World::new();
    let mut dispatcher = 
        DispatcherBuilder::new()
        .register_systems()
        .build();

    world.register::<comp::Pos>();
    world.register::<comp::Vel>();
    world.register::<comp::Body>();

    world.create_entity()
        .with(comp::Pos (vek::Vec2 { x: 100.0, y: 100.0}))
        .with(comp::Body::Circle(20.0))
        .with(comp::Vel(vek::Vec2 { x: 2.0, y: 1.0 }))
        .build();

    world.create_entity()
        .with(comp::Pos (vek::Vec2 { x: 150.0, y: 150.0}))
        .with(comp::Body::RectangleOutline(200.0, 400.0, 5.0))
        .build();

    'game: loop {
        'update: {
            dispatcher.dispatch(&world);
            break 'update;
        };

        let pos_storage = world.read_storage::<comp::Pos>();
        let body_storage = world.read_storage::<comp::Body>();

        'draw: for (pos, body) in (&pos_storage, &body_storage).join(){
            match body{
                comp::Body::Circle(r) => draw_circle(pos.0.x, pos.0.y, r.clone(), LIGHTGRAY),
                comp::Body::CircleOutline(r, t) => draw_circle_lines(pos.0.x, pos.0.y, r.clone(), t.clone(), LIGHTGRAY),
                comp::Body::Rectangle(w, h) => draw_rectangle(pos.0.x, pos.0.y, w.clone(), h.clone(), LIGHTGRAY),
                comp::Body::RectangleOutline(w, h, t) => draw_rectangle_lines(pos.0.x, pos.0.y, w.clone(), h.clone(), t.clone(), LIGHTGRAY),
                comp::Body::Line(target, t) => draw_line(pos.0.x, pos.0.y, target.x, target.y, t.clone(), LIGHTGRAY),
                _ => todo!()
            }
            continue 'draw;
        };

        mqp::next_frame().await;
    }


}
