use cgmath::{Matrix4, SquareMatrix};
pub struct Camera {
    x : f32, 
    y : f32,
    width : f32,
    height : f32,
    world : Matrix4<f32>,
    view : Matrix4<f32>,
    ortho : Matrix4<f32>,
}


impl Camera {
    pub fn new(width : f32, height:f32) -> Camera {
        let world = Matrix4::identity();
        let view = Matrix4::identity();
        let projection = cgmath::ortho(0.0f32, width, height, 0.0f32, 0.0f32, 100.0f32);
        Camera { x : 0.0, y : 0.0, width : width, height : height, world : world, view : view, ortho : projection }
    }

    pub fn move_camera(&mut self, x : f32, y : f32) {
        self.x = x;
        self.y = y;
        self.ortho = cgmath::ortho(x, self.width, self.height, y, 0.0f32, 100.0f32);
    }

}