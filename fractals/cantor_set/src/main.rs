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

fn draw_cantor(draw: &nannou::Draw, x: f32, mut y: f32, len: f32) {
    if len < 0.33 { // break infinite recursive loop
        return;
    }

    draw.line()
        .start(pt2(x, y))
        .end(pt2(x + len, y))
        .stroke_weight(10.0)
        .color(BLACK);
    
    y += 30.0; // space between rows
    
    draw_cantor(&draw, x, y, len / 3.0);
    draw_cantor(&draw, x + (len * (2.0 / 3.0)), y, len / 3.0);
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let boundary = app.window_rect();
    let width = boundary.w();
    let height = boundary.h();

    draw.background().color(WHITE);
    draw_cantor(
        &draw,
        - (width / 2.0), // center between width (i.e. w = 20, x'=-10, x"=10 -> fills screen)
        - (height / 2.0 ) + 10.0, // start at bottom + stroke weight
        width
    );
    draw.to_frame(app, &frame).unwrap();
}
