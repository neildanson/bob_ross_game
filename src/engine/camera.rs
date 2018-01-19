extern crate cgmath;
use cgmath::{Matrix4, SquareMatrix};
use engine::BoundingBox;

pub struct Camera {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    pub boundingbox: BoundingBox,
    pub world: Matrix4<f32>,
    pub view: Matrix4<f32>,
    pub ortho: Matrix4<f32>,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Camera {
        let world = Matrix4::identity();
        let view = Matrix4::identity();
        let projection = cgmath::ortho(0.0f32, width, height, 0.0f32, 0.0f32, 100.0f32);
        let x = 0.0;
        let y = 0.0;
        Camera {
            x: x,
            y: y,
            width: width,
            height: height,
            boundingbox: BoundingBox::new(x, y, width, height),
            world: world,
            view: view,
            ortho: projection,
        }
    }

    pub fn look_at(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
        let x = x - self.width / 2.0;
        {
            let y = y - self.height / 2.0;
            self.boundingbox = BoundingBox::new(x, y, self.width, self.height);
        }
        let y = y + self.height / 2.0;

        self.ortho = cgmath::ortho(x, x + self.width, y, y - self.height, 0.0f32, 100.0f32);
    }
}
