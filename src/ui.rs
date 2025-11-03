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

        let screen_size = Vec2::new(screen_width(), screen_height());
        let aspect = if screen_size.x > screen_size.y {
            Vec2::new(screen_size.y / screen_size.x, 1.)
        } else {
            Vec2::new(1., screen_size.x / screen_size.y)
        };

        let outer_margin = self.outer_margin * aspect;
        let inner_margin = self.inner_margin * aspect;
        let mut position = self.position + outer_margin;
        let size = self.size - outer_margin * 2. - align * inner_margin * (self.containers.len() as f32 - 1.);

        let mut total_weight = 0.;
        for container in self.containers.iter() {
            total_weight += container.weight;
        }

        for container in self.containers.iter_mut() {
            let weight_percent = container.weight / total_weight;
            container.position = position;
            container.size = size * (align * weight_percent + Vec2::ONE - align);
            container.draw();
            position += align * (container.size + inner_margin);
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
    fn draw(&self, mut position: Vec2, mut size: Vec2) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        position *= screen_size;
        size *= screen_size;

        draw_rectangle(position.x, position.y, size.x, size.y, self.color);
    }
}

pub struct RoundedPanel {
    pub color: Color,
    pub radius: f32,
}

impl Default for RoundedPanel {
    fn default() -> Self {
        Self {
            color: WHITE,
            radius: 0.,
        }
    }
}

impl Component for RoundedPanel {
    fn draw(&self, mut position: Vec2, mut size: Vec2) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        let radius = self.radius * screen_size.x.min(screen_size.y);
        position *= screen_size;
        size *= screen_size;

        draw_circle(position.x + radius, position.y + radius, radius, self.color);
        draw_circle(position.x + size.x - radius, position.y + radius, radius, self.color);
        draw_circle(position.x + radius, position.y + size.y - radius, radius, self.color);
        draw_circle(position.x + size.x - radius, position.y + size.y - radius, radius, self.color);
        draw_rectangle(position.x, position.y + radius, size.x, size.y - radius * 2., self.color);
        draw_rectangle(position.x + radius, position.y, size.x - radius * 2., size.y, self.color);
    }
}
