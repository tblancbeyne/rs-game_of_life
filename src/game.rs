use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::window::{mouse::Button as MouseButton, ContextSettings, Event, Key, Style};

use sfml::system::Vector2i;

use crate::matrix::Matrix;

pub struct Game {
    map: Matrix,
    window: RenderWindow,
    running: bool,
    stopped: bool,
    scale: f32,
}

impl Game {
    pub fn new(rows: usize, cols: usize, scale: usize) -> Self {
        let window = RenderWindow::new(
            ((scale * cols) as u32, (scale * cols) as u32),
            "Game of Life",
            Style::DEFAULT,
            &ContextSettings::default(),
        );

        Game {
            map: Matrix::new((rows, cols, scale as f32)),
            window: window,
            running: true,
            stopped: true,
            scale: scale as f32,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            self.update();
            self.draw();
        }
    }

    pub fn update(&mut self) {
        while let Some(event) = self.window.poll_event() {
            self.manage_event(event);
        }

        if !self.stopped {
            self.map.update();
        }
    }

    fn manage_event(&mut self, event: Event) {
        match (event, self.stopped) {
            // Quit the game is the window is closed
            (Event::Closed, _) => self.running = false,

            // Quit the game if escape is pressed
            (
                Event::KeyPressed {
                    code: Key::Escape, ..
                },
                _,
            ) => self.running = false,

            (
                Event::KeyPressed {
                    code: Key::Return, ..
                },
                _,
            ) => self.stopped = !self.stopped,

            (Event::MouseButtonPressed { button, x, y }, true) => {
                if button == MouseButton::Left {
                    let coords = self
                        .window
                        .map_pixel_to_coords_current_view(Vector2i::new(x, y));
                    let x = (coords.x as f32 / self.scale) as usize;
                    let y = (coords.y as f32 / self.scale) as usize;
                    if x < self.map.rows() && y < self.map.cols() {
                        self.map.change_state(y, x);
                    }
                }
            }

            _ => {}
        }
    }

    fn draw(&mut self) {
        self.window.clear(Color::rgb(0, 0, 0));

        self.window.draw(&self.map);

        self.window.display();
    }
}
