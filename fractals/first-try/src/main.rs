use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

// fn draw_circle(draw: &nannou::Draw, x: f32, y: f32, radius: f32) {
//     draw.ellipse()
//         .x_y(x, y)
//         .radius(radius)
//         .stroke(BLACK)
//         .stroke_weight(2.0)
//         .no_fill();

//     if radius > 2.0 {
//         draw_circle(&draw, x + radius / 2.0, y, radius / 2.0);
//         draw_circle(&draw, x - radius / 2.0, y, radius / 2.0);
//     }
// }

fn draw_circle(draw: &nannou::Draw, x: f32, y: f32, radius: f32) {
    draw.ellipse()
        .x_y(x, y)
        .radius(radius)
        .stroke(BLACK)
        .stroke_weight(2.0)
        .no_fill();
    if radius > 8.0 {
        draw_circle(&draw, x + radius / 2.0, y, radius / 2.0);
        draw_circle(&draw, x - radius / 2.0, y, radius / 2.0);
        draw_circle(&draw, x, y + radius / 2.0, radius / 2.0);
        draw_circle(&draw, x, y - radius / 2.0, radius / 2.0);
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect();
    let width = boundary.w();
    let height = boundary.h();

    draw.background().color(WHITE);
    draw_circle(&draw, width / 20.0, height / 20.0, 500.0);
    draw.to_frame(app, &frame).unwrap();
}
