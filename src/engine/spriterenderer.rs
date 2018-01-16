extern crate glium;

use glium::{Display, Program,VertexBuffer, IndexBuffer, Frame, Surface, DrawParameters};
use glium::index::PrimitiveType;
use engine::SpriteBatch;
use engine::SpriteSheet;
use engine::Camera;
use engine::Vertex;

pub struct SpriteRenderer<'a>  { 
    program : Program,
    vertex_buffer : VertexBuffer<Vertex>,
    index_buffer : IndexBuffer<u32>,
    draw_parameters : DrawParameters<'a>,
}

impl <'a> SpriteRenderer <'a> { 
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
                    //if (tex_color.z < 0.01) discard;
                    //else 
                    f_color = tex_color;
                }
            "
        },
        ).unwrap();

        let vertex_buffer = VertexBuffer::empty_dynamic(display, 200000).unwrap();
        let index_buffer = IndexBuffer::empty_dynamic(display,
                                                   PrimitiveType::TrianglesList,
                                                   200000).unwrap();

        let draw_parameters = DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };


        SpriteRenderer { draw_parameters : draw_parameters, program : program, 
                         vertex_buffer : vertex_buffer, index_buffer : index_buffer }
    }

    pub fn draw(&mut self, frame: &mut Frame, spritebatch : &SpriteBatch, spritesheet : &SpriteSheet, camera : &Camera) {
        {
            let mut vb_map = self.vertex_buffer.map_write();
            for v in 0 .. spritebatch.quads.len() {
                vb_map.set(v, spritebatch.quads[v])
                //vb_map.write(&spritebatch.quads);
            }
            //let vertex_buffer = VertexBuffer::new(self.display, &spritebatch.quads).unwrap();
            let mut ib_map = self.index_buffer.map_write();
            for i in 0 .. spritebatch.indices.len() {
                ib_map.set(i,spritebatch.indices[i]);
            }
        }
        // building the uniforms
        let uniforms = uniform! {
            world: Into::<[[f32;4];4]>::into(camera.world),
            view: Into::<[[f32;4];4]>::into(camera.view),
            projection: Into::<[[f32;4];4]>::into(camera.ortho),
            tex: spritesheet.texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
        };

        //let vb_slice = self.vertex_buffer.slice(0 .. spritebatch.quads.len()).unwrap();
        let ib_slice = self.index_buffer.slice(0 .. spritebatch.indices.len()).unwrap();

        frame.draw(
                  &self.vertex_buffer,
                  &ib_slice,
                  &self.program,
                  &uniforms,
                  &self.draw_parameters)
            .unwrap();
    }
}