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
    pub weight: f32,
    pub color: Color,
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
            weight: 1.,
            color: BLACK,
            align: Align::default(),
            components: Vec::default(),
            containers: Vec::default(),
        }
    }
}

impl Container {
    pub fn draw(&mut self) {
        for component in self.components.iter() {
            component.draw(self.position, self.size);
        }

        let align = match self.align {
            Align::Horizontal => Vec2::X,
            Align::Vertical => Vec2::Y,
        };

        let mut position = self.position;
        let size = self.size - self.outer_margin * 2.;

        let mut total_weight = 0.;
        for container in self.containers.iter() {
            total_weight += container.weight;
        }

        for container in self.containers.iter_mut() {
            let weight_percent = container.weight / total_weight;
            container.position = position;
            container.size = size * (align * weight_percent + Vec2::ONE - align);
            container.draw();
            position += align * container.size;
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
