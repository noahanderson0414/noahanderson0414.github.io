use macroquad::prelude::*;

pub trait Component {
    fn draw(&self, position: Vec2, size: Vec2);
}

#[derive(Default)]
pub enum Align {
    #[default]
    Horizontal,
    Vertical,
}

pub struct Container {
    pub position: Vec2,
    pub size: Vec2,
    pub inner_margin: f32,
    pub outer_margin: f32,
    pub align: Align,
    pub components: Vec<Box<dyn Component>>,
    pub containers: Vec<Container>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            size: Vec2::ONE,
            inner_margin: 0.,
            outer_margin: 0.,
            align: Align::default(),
            components: Vec::default(),
            containers: Vec::default(),
        }
    }
}

impl Container {
    pub fn draw(&mut self) {
        let inner_margin = Vec2::splat(self.inner_margin);
        let outer_margin = Vec2::splat(self.outer_margin);
        let align = match self.align {
            Align::Horizontal => Vec2::X,
            Align::Vertical => Vec2::Y,
        };

        let mut position = self.position + outer_margin;
        let mut size = self.size - outer_margin * 2.;
        size -= align * inner_margin.x * (self.components.len().max(1) - 1) as f32;
        size /= (align * self.components.len() as f32).max(Vec2::ONE);

        for component in self.components.iter() {
            component.draw(position, size);
            position += align * (size + inner_margin);
        }

        position = self.position + outer_margin;
        size = self.size - outer_margin * 2.;
        size -= align * inner_margin.x * (self.containers.len().max(1) - 1) as f32;
        size /= (align * self.containers.len().max(1) as f32).max(Vec2::ONE);

        for container in self.containers.iter_mut() {
            container.position = position;
            container.size = size;
            container.draw();
            position += align * (size + inner_margin);
        }
    }
}

pub struct Panel {
    pub color: Color,
}

impl Default for Panel {
    fn default() -> Self {
        Self {
            color: WHITE,
        }
    }
}

impl Component for Panel {
    fn draw(&self, position: Vec2, size: Vec2) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        let position = position * screen_size;
        let size = size * screen_size;

        draw_rectangle(position.x, position.y, size.x, size.y, self.color);
    }
}
