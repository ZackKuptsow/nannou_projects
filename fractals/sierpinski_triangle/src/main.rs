use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn draw_sierpinkski(draw: &nannou::Draw, start: Point2, top: Point2, end: Point2, depth: u32) {
    if depth == 0 {
        draw.tri().points(start, end, top).color(BLACK);
    } else {
        let mid_start_top = (start + top) / 2.0;
        let mid_top_end = (top + end) / 2.0;
        let mid_end_start = (end + start) / 2.0;

        draw_sierpinkski(draw, start, mid_start_top, mid_end_start, depth - 1);
        draw_sierpinkski(draw, mid_start_top, top, mid_top_end, depth - 1);
        draw_sierpinkski(draw, mid_end_start, mid_top_end, end, depth - 1);
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let boundary = app.window_rect();
    let middle = boundary.xy();

    let start = pt2(middle.x - boundary.w() / 2.0, middle.y - boundary.h() / 2.0);
    let end = pt2(middle.x + boundary.w() / 2.0, middle.y - boundary.h() / 2.0);

    let length = boundary.w();
    let height = boundary.h();

    let depth = 3;

    // let height = (length * (3.0f32).sqrt()) / 2.0;
    let top = pt2(middle.x, middle.y + height);

    draw.background().color(WHITE);
    draw_sierpinkski(&draw, start, top, end, depth);
    draw.to_frame(app, &frame).unwrap();
}
