extern crate rand;
use nannou::prelude::*;
use rand::Rng;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    window_id: window::Id,
    effect: Effect,
}

struct Particle {
    x: i32,
    y: i32,
}

impl Particle {
    fn new(effect: &Effect) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-effect.width..effect.width) as i32;
        let y = rng.gen_range(-effect.height..effect.height) as i32;

        Particle { x, y }
    }

    pub fn draw(&self, draw: &Draw, side_length: f32) {
        let center_x = self.x as f32 + side_length / 2.0;
        let center_y = self.y as f32 + side_length / 2.0;
        draw.rect()
            .x_y(center_x, center_y)
            .w_h(side_length, side_length)
            .color(WHITE);
    }
}

struct Effect {
    width: f32,
    height: f32,
    particles: Vec<Particle>,
}

impl Effect {
    fn new(width: f32, height: f32) -> Self {
        Effect {
            width,
            height,
            particles: Vec::new(),
        }
    }

    fn generate_particles(&mut self, amount: usize) {
        self.particles = (0..amount).map(|_| Particle::new(self)).collect();
    }

    fn render_particles(&self, draw: &Draw) {
        self.particles
            .iter()
            .for_each(|particle| particle.draw(&draw, 10.0))
    }
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().size(800, 800).view(view).build().unwrap();
    let window = app.window(window_id).unwrap();
    let window_rect = window.rect();

    let mut effect = Effect::new(window_rect.w(), window_rect.h());
    effect.generate_particles(1000);

    Model { window_id, effect }
}

fn update(app: &App, model: &mut Model, update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    // model.effect.generate_particles(100);
    model.effect.render_particles(&draw);
    // draw.line()
    //     .start(pt2(-350.0, 300.0))
    //     .end(pt2(-200.0, 150.0))
    //     .weight(1.0)
    //     .color(WHITE);
    draw.to_frame(app, &frame).unwrap();
}
