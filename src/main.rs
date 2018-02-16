extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod nbody;

pub struct App {
    gl: GlGraphics,
    particles: Vec<nbody::Particle>,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let particles = &self.particles;
        
        self.gl.draw(args.viewport(), |context, gl| {
            clear(WHITE, gl);
            
            for particle in particles {
                let particle_render = ellipse::circle(0.0, 0.0, particle.mass / 10000000.0);
                let transform = context.transform.trans(particle.position[0], particle.position[1]);
                ellipse(particle.color, particle_render, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        nbody::step(&mut self.particles, args.dt);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("n_body_simulation", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .expect("Could not create window");

    let mut app = App {
        gl: GlGraphics::new(opengl),
        particles: Vec::new(),
    };

    for _ in 0..4 {
        app.particles.push(nbody::random_particle());
    }

    let mut cursor_pos = [0.0, 0.0];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(pos) = e.mouse_cursor_args() {
            cursor_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let mut particle = nbody::random_particle();
            particle.position = cursor_pos;
            app.particles.push(particle);
        }
    }
}
