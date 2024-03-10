use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    current_depth: u32,
    depth: u32,
    draw: Draw,
    koch_type: KochType,
    last_update: std::time::Instant,
    points: Vec<Point2>,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(10.0)); // Control frame rate of animation

    let draw = app.draw();
    draw.background().color(WHITE);

    let _window = app.new_window().size(800, 600).view(view).build().unwrap();

    let boundary = app.window_rect();
    let middle = boundary.xy();

    // Set the starting and ending points for the Koch curve
    let start = pt2(middle.x - boundary.w() / 2.0, middle.y);
    let end = pt2(middle.x + boundary.w() / 2.0, middle.y);

    Model {
        current_depth: 0, // Start with 0 depth
        depth: 6,         // Total depth you want to reach
        draw,
        koch_type: KochType::Radial(8),         // Change as needed
        last_update: std::time::Instant::now(), // Control animation speed
        points: vec![start, end],               // Initial points
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let now = std::time::Instant::now();

    if now.duration_since(model.last_update).as_secs_f32() > 0.5 {
        // Update every 0.5 seconds
        model.last_update = now;

        // Incrementally increase the depth
        if model.current_depth < model.depth {
            // if model.current_depth < model.depth && app.elapsed_frames() % 2 == 0 { // stagger draw every 2 seconds
            model.current_depth += 1;
            // Calculate the new points based on the current depth
            // Update model.points with new points to draw
            let boundary = app.window_rect();
            let middle = boundary.xy();
            let start = pt2(middle.x - boundary.w() / 2.0, middle.y);
            let end = pt2(middle.x + boundary.w() / 2.0, middle.y);
            model.points = koch_line(start, end, model.current_depth, model.koch_type.clone());
        }
    }
}

#[derive(Clone)]
enum KochType {
    Linear,
    Radial(u32), // Number of sides
}

fn koch_line(start: Point2, end: Point2, depth: u32, koch_type: KochType) -> Vec<Point2> {
    let mut points = Vec::new();

    match koch_type {
        KochType::Linear => {
            // Base case of recursion
            if depth == 0 {
                points.push(start);
                points.push(end);
                return points;
            } else {
                // Calculate points for the Koch curve
                let one_third = start + (end - start) / 3.0;
                let two_thirds = start + (end - start) * 2.0 / 3.0;
                let middle_vec = (end - start) / 3.0;
                let angle = PI / 3.0; // 60 degrees in radians
                let rotation_matrix = |v: Vec2| -> Vec2 { v.rotate(angle) };
                let apex = one_third + rotation_matrix(middle_vec);

                // Recursively call koch_line for each segment and append the results
                points.extend(koch_line(start, one_third, depth - 1, KochType::Linear));
                points.extend(koch_line(one_third, apex, depth - 1, KochType::Linear));
                points.extend(koch_line(apex, two_thirds, depth - 1, KochType::Linear));
                points.extend(koch_line(two_thirds, end, depth - 1, KochType::Linear));
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

                points.extend(koch_line(start, end, depth, KochType::Linear)); // Use Linear here to avoid infinite recursion
            }
        }
    }
    points
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Clear the frame
    draw.background().color(WHITE);

    // Use the current points in model.points to draw the curve as it currently is
    // You might draw lines between each consecutive pair of points
    for i in 0..model.points.len() - 1 {
        draw.line()
            .start(model.points[i])
            .end(model.points[i + 1])
            .stroke_weight(2.0)
            .color(BLACK);
    }

    // Finish and present the frame
    draw.to_frame(app, &frame).unwrap();
}
