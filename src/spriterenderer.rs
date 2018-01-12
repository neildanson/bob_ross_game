extern crate cgmath;
extern crate glium;

use glium::{Display, Program,VertexBuffer, IndexBuffer, Frame, Surface, DrawParameters};
use glium::index::PrimitiveType;
use cgmath::SquareMatrix; //TODO replace with camera
use SpriteBatch;
use SpriteSheet;

pub struct SpriteRenderer<'a>  { 
    program : Program,
    display : &'a Display
}

impl <'a> SpriteRenderer<'a> { 
    pub fn new (display : &Display) -> SpriteRenderer {
        let program = program!(display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 world;
                uniform mat4 projection;
                uniform mat4 view;
                in vec2 position;
                in vec2 tex_coord;
                out vec2 v_texcoord;
                void main() {
                    mat4 wvp = world * view * projection;
                    gl_Position = wvp * vec4(position, 0.0, 1.0);
                    v_texcoord = tex_coord;
                }
            ",

            fragment: "
                #version 140
                uniform sampler2D tex;
                in vec2 v_texcoord;
                out vec4 f_color;
                void main() {
                    vec4 tex_color = texture(tex, v_texcoord);
                    //if (tex_color.z < 0.1) discard;
                    f_color = tex_color;
                }
            "
        },
        ).unwrap();

        SpriteRenderer { display : display, program : program }
    }

    //TODO - dont pass in here & use camera
    pub fn draw(&self, frame: &mut Frame, spritebatch : &SpriteBatch, spritesheet : &SpriteSheet) {
        let draw_parameters =  DrawParameters {
            .. Default::default()
        }; //TODO create once
        let vertex_buffer = VertexBuffer::new(self.display, &spritebatch.quads).unwrap();
        let index_buffer = IndexBuffer::new(self.display,
                                                   PrimitiveType::TrianglesList,
                                                   &spritebatch.indices).unwrap();


        let world = cgmath::Matrix4::identity();
        let view = cgmath::Matrix4::identity();
        let projection = cgmath::ortho(0.0f32, 320.0f32, 240.0f32, 0.0f32, 0.0f32, 100.0f32);
        // building the uniforms
        let uniforms = uniform! {
            world: Into::<[[f32;4];4]>::into(world),
            view: Into::<[[f32;4];4]>::into(view),
            projection: Into::<[[f32;4];4]>::into(projection),
            tex: spritesheet.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };

        frame.draw(
                  &vertex_buffer,
                  &index_buffer,
                  &self.program,
                  &uniforms,
                  &draw_parameters)
            .unwrap();
    }
}