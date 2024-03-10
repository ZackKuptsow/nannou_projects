use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};
use rustfft::{FftPlanner, num_complex::Complex};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    fft_data: Vec<Complex<f32>>,
    frame_count: u64, // For time base animation
    particles: Vec<Particle>,
}

struct Particle{
    position: Point2,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    // generate noise data for fft visualization
    let noise_data = generate_noise_data(1024);
    let fft_data = compute_fft(&noise_data);
    // track frame count for animation
    let frame_count = 0;
    // generate random particles for background
    let particles = generate_particles(1024, 200.0, 400.0);
    
    Model { fft_data, frame_count, particles, _window}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.frame_count += 1;
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let perlin = Perlin::new(); // Noise function for phase offsets

    let center = pt2(0.0, 0.0); // Center of the circle
    let base_radius = 200.0; // Base radius of the visualization

    for (i, complex) in _model.fft_data.iter().enumerate() {
        let angle = map_range(i, 0, _model.fft_data.len(), 0.0, 2.0 * PI);
        let phase_offset = perlin.get([i as f64 * 0.05, _model.frame_count as f64 * 0.02]);
        let radius_offset = (phase_offset as f32) * 50.0;
        let radius = base_radius + complex.norm() * 10.0 + radius_offset;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();

        draw.ellipse()
            .x_y(x, y)
            .w_h(4.0, 4.0)
            .color(WHITE);
    }

    for particle in &_model.particles {
        draw.ellipse()
            .x_y(particle.position.x, particle.position.y)
            .w_h(1.0, 1.0)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn generate_noise_data(length: usize) -> Vec<f32> {
    let perlin = Perlin::new();
    (0..length).map(|i| {
        let noise_value = perlin.get([i as f64 * 0.1, 0.0]);
        noise_value as f32
    }).collect()
}

fn generate_particles(amount: usize, min_radius: f32, max_radius: f32) -> Vec<Particle> {
    (0..amount).map(|_| {
        let angle = random_range(0.0, 2.0 * PI);
        let radius = random_range(min_radius, max_radius);

        let x = radius * angle.cos();
        let y = radius * angle.sin();

        Particle { position: pt2(x, y) }
    }).collect()
}

fn compute_fft(data: &[f32]) -> Vec<Complex<f32>> {
    let mut wave: Vec<Complex<f32>> = data.iter().map(|&f| Complex::new(f, 0.0)).collect();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(wave.len());
    fft.process(&mut wave);

    wave
}