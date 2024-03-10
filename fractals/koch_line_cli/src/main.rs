use clap::{Arg, Command};
use nannou::prelude::*;
use once_cell::sync::OnceCell;

// Define a global static variable using OnceCell
static GLOBAL_DATA: OnceCell<Model> = OnceCell::new();

fn main() {
    let matches = Command::new("Koch Curve Generator")
        .version("1.0")
        .about("Draws a Koch Curve")
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                // .required(true)
                .help("Type of Koch curve: 'linear' or 'radial'"),
        )
        .arg(
            Arg::new("sides")
                .short('s')
                .long("sides")
                .help("Number of sides for radial (ignored if type is linear)"),
        )
        .arg(
            Arg::new("depth")
                .short('d')
                .long("depth")
                .help("Depth of recursion for the Koch curve"),
        )
        .get_matches();

    let koch_type_str = matches.get_one::<String>("type").expect(
        "type is required\nplease specify with `-t` or `--type`\noptions include: linear or radial",
    );
    let koch_type = match koch_type_str.as_str() {
        "radial" => {
            let sides = matches
                .get_one::<String>("sides")
                .and_then(|s| s.parse().ok())
                .unwrap_or(5);
            KochType::Radial(sides)
        }
        _ => KochType::Linear,
    };

    let depth_str = matches
        .get_one::<String>("depth")
        .cloned()
        .unwrap_or("4".to_string());
    let depth = depth_str.parse().unwrap_or(4);

    // Initialize the global data once at runtime
    let _ = GLOBAL_DATA.set(Model {
        koch_type: koch_type,
        depth: depth,
    });

    nannou::app(model).update(update).view(view).run();
}

struct Model {
    koch_type: KochType,
    depth: u32,
}

fn model(app: &App) -> Model {
    // Initialize variables with default values or use Option type
    let mut koch_type = KochType::Linear; // Default value
    let mut depth = 4; // Default value

    if let Some(params) = GLOBAL_DATA.get() {
        // Now params is a reference to the Model instance

        // Access individual fields
        koch_type = params.koch_type.clone();
        depth = params.depth;

        // Use the values as needed
        println!("Koch type: {:?}", koch_type);
        println!("Depth: {:?}", depth);

        // If you need to match on the KochType
        match koch_type {
            KochType::Linear => println!("Koch type is Linear"),
            KochType::Radial(sides) => println!("Koch type is Radial with {} sides", sides),
        }
    } else {
        // Handle the case where the params have not been initialized
        println!("Parameters have not been initialized.");
    }

    app.new_window().size(800, 600).view(view).build().unwrap();
    Model { koch_type, depth }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    // Update logic goes here
}

#[derive(Clone, Debug)]
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

fn view(app: &App, model: &Model, frame: Frame) {
    // Drawing logic goes here
    let draw = app.draw();
    draw.background().color(WHITE);

    let boundary = app.window_rect();
    let middle = boundary.xy();

    // Set the starting and ending points for the Koch curve
    let start = pt2(middle.x - boundary.w() / 2.0, middle.y);
    let end = pt2(middle.x + boundary.w() / 2.0, middle.y);

    // Draw the Koch curve based on the specified type and depth
    koch_line(&draw, start, end, model.depth, model.koch_type.clone());

    // Finish and present the frame
    draw.to_frame(app, &frame).unwrap();
}
