use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model;

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();
    Model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Update logic goes here
}

enum KochType {
    Linear,
    Radial(u32), // Number of sides
}

fn koch_line(draw: &nannou::Draw, start: Point2, end: Point2, depth: u32, koch_type: KochType) {
    match koch_type {
        KochType::Linear => {
            if depth == 0 {
                draw.line()
                    .start(start)
                    .end(end)
                    .stroke_weight(4.0)
                    .color(BLACK);
            } else {
                // Calculate points for the Koch curve
                let one_third = start + (end - start) / 3.0;
                let two_thirds = start + (end - start) * 2.0 / 3.0;
                let middle_vec = (end - start) / 3.0;
                let angle = PI / 3.0; // 60 degrees in radians
                let rotation_matrix = |v: Vec2| -> Vec2 { v.rotate(angle) };
                let apex = one_third + rotation_matrix(middle_vec);

                // Recursively draw the four new line segments
                koch_line(draw, start, one_third, depth - 1, KochType::Linear);
                koch_line(draw, one_third, apex, depth - 1, KochType::Linear);
                koch_line(draw, apex, two_thirds, depth - 1, KochType::Linear);
                koch_line(draw, two_thirds, end, depth - 1, KochType::Linear);
            }
        }
        KochType::Radial(sides) => {
            let center = pt2(0.0, 0.0); // Center of the circle
            let radius = 300.0; // Radius of the radial Koch curve

            for i in 0..sides {
                let angle_start = i as f32 * TAU / sides as f32;
                let angle_end = (i as f32 + 1.0) * TAU / sides as f32;

                let start = pt2(
                    center.x + angle_start.cos() * radius,
                    center.y + angle_start.sin() * radius,
                );

                let end = pt2(
                    center.x + angle_end.cos() * radius,
                    center.y + angle_end.sin() * radius,
                );

                koch_line(draw, start, end, depth, KochType::Linear); // Use Linear here to avoid infinite recursion
            }
        }
    }
}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Drawing logic goes here
    let draw = app.draw();
    draw.background().color(WHITE);

    let boundary = app.window_rect();
    let middle = boundary.xy();

    // Set the starting and ending points for the Koch curve
    let start = pt2(middle.x - boundary.w() / 2.0, middle.y);
    let end = pt2(middle.x + boundary.w() / 2.0, middle.y);

    // Draw the Koch curve with a certain depth of recursion
    // Change KochType::Linear to KochType::Radial(n) to draw a radial Koch curve with n sides
    koch_line(&draw, start, end, 4, KochType::Radial(12));

    // Finish and present the frame
    draw.to_frame(app, &frame).unwrap();
}
