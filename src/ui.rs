use macroquad::prelude::*;

#[derive(Default)]
pub enum Anchor {
    TopLeft,
    TopRight,
    #[default]
    Center,
}

impl Anchor {
    pub fn offset(&self, position: Vec2, size: Vec2) -> Vec2 {
        match self {
            Self::TopLeft => position,
            Self::TopRight => position - Vec2::new(size.x, 0.),
            Self::Center => position - size / 2.,
        }
    }
}

pub struct Rectangle {
    pub position: Vec2,
    pub size: Vec2,
    pub anchor: Anchor,
    pub color: Color,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            position: Vec2::splat(0.5),
            size: Vec2::splat(0.5),
            anchor: Default::default(),
            color: WHITE,
        }
    }
}

impl Rectangle {
    pub fn draw(&self) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        let position = self.anchor.offset(self.position, self.size) * screen_size;
        let size = self.size * screen_size;

        draw_rectangle(position.x, position.y, size.x, size.y, self.color);
    }
}

pub struct RoundedRectangle {
    pub position: Vec2,
    pub size: Vec2,
    pub radius: f32,
    pub sides: u8,
    pub anchor: Anchor,
    pub color: Color,
}

impl Default for RoundedRectangle {
    fn default() -> Self {
        Self {
            position: Vec2::splat(0.5),
            size: Vec2::splat(0.5),
            radius: 0.1,
            sides: 32,
            anchor: Default::default(),
            color: WHITE,
        }
    }
}

impl RoundedRectangle {
    pub fn draw(&self) {
        let screen_size = Vec2::new(screen_width(), screen_height());
        let position = self.anchor.offset(self.position, self.size) * screen_size;
        let size = self.size * screen_size;
        let radius = self.radius * size.x.min(size.y) / 2.;

        draw_poly(position.x + radius, position.y + radius, self.sides, radius, 0., self.color);
        draw_poly(position.x + size.x - radius, position.y + radius, self.sides, radius, 0., self.color);
        draw_poly(position.x + size.x - radius, position.y + size.y - radius, self.sides, radius, 0., self.color);
        draw_poly(position.x + radius, position.y + size.y - radius, self.sides, radius, 0., self.color);
        draw_rectangle(position.x, position.y + radius, size.x, size.y - radius * 2., self.color);
        draw_rectangle(position.x + radius, position.y, size.x - radius * 2., size.y, self.color);
    }
}
