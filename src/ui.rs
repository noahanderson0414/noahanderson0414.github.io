use macroquad::prelude::*;

pub trait Component {
    fn draw(&self, position: Vec2, size: Vec2);
}

pub struct Container {
    pub position: Vec2,
    pub size: Vec2,
    pub components: Vec<Box<dyn Component>>,
    pub containers: Vec<Container>,
}

impl Default for Container {
    fn default() -> Self {
        Self {
            position: Default::default(),
            size: Vec2::splat(1.),
            components: Default::default(),
            containers: Default::default(),
        }
    }
}

impl Container {
    pub fn draw(&mut self) {
        let mut position = self.position;
        let mut size = self.size;
        size.x /= self.components.len() as f32;

        println!("{:?}\n{:?}\n", self.position, self.size);

        for component in self.components.iter() {
            component.draw(position, size);
            position.x += size.x;
        }

        position = self.position;
        size = self.size;
        size.x /= self.containers.len() as f32;

        for container in self.containers.iter_mut() {
            container.position = position;
            container.size = size;
            container.draw();
            position.x += size.x;
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
