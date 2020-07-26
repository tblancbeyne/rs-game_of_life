use std::collections::{HashMap, HashSet};

use sfml::graphics::{
    Color, Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Transformable,
};

use sfml::system::Vector2f;

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    scale: f32,
    data: HashSet<(usize, usize)>,
}

impl Matrix {
    pub fn new(size: (usize, usize, f32)) -> Self {
        Matrix {
            rows: size.0,
            cols: size.1,
            scale: size.2,
            data: HashSet::new(),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn change_state(&mut self, row: usize, col: usize) {
        if self.data.contains(&(row, col)) {
            self.data.remove(&(row, col));
        } else {
            self.data.insert((row, col));
        }
    }

    pub fn update(&mut self) {
        let mut count = HashMap::new();

        for element in &self.data {
            let i = element.0;
            let j = element.1;
            for k in -1..2 {
                for l in -1..2 {
                    if k != 0 || l != 0 {
                        if i as isize + k >= 0
                            && i as isize + k < self.rows() as isize
                            && j as isize + l >= 0
                            && j as isize + l < self.cols() as isize
                        {
                            let c = count
                                .entry(((i as isize + k) as usize, (j as isize + l) as usize))
                                .or_insert(0);
                            *c += 1;
                        }
                    }
                }
            }
        }

        self.data.clear();

        for (k, v) in &count {
            if *v == 3 || (*v == 2 && rand::random::<u8>() % 10 < 3) {
                self.data.insert((k.0, k.1));
            }
        }

        for _i in 0..rand::random::<u8>() {
            let x = rand::random::<usize>() % self.rows;
            let y = rand::random::<usize>() % 3;
            self.change_state(x, y);
        }
    }
}

impl Drawable for Matrix {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        render_target: &mut dyn RenderTarget,
        _: RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut square = RectangleShape::with_size(Vector2f::new(self.scale, self.scale));

                square.set_origin(square.size() / 2.0);
                square.set_position((
                    j as f32 * self.scale + self.scale / 2.0,
                    i as f32 * self.scale + self.scale / 2.0,
                ));
                square.set_outline_thickness(2.0);
                square.set_outline_color(Color::rgb(0, 0, 0));
                render_target.draw(&square);
            }
        }

        for element in &self.data {
            let mut square = RectangleShape::with_size(Vector2f::new(self.scale, self.scale));

            square.set_origin(square.size() / 2.0);
            square.set_position((
                element.1 as f32 * self.scale + self.scale / 2.0,
                element.0 as f32 * self.scale + self.scale / 2.0,
            ));

            square.set_fill_color(Color::rgb(0, 0, 0));
            render_target.draw(&square);
        }
    }
}
