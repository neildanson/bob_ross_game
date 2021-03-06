extern crate glium;

use glium::{Blend, Depth, Display, DrawParameters, Frame, IndexBuffer, Program, Surface,
            VertexBuffer};
use glium::index::PrimitiveType;
use glium::BackfaceCullingMode;
use glium::draw_parameters::DepthTest;
use glium::uniforms::MagnifySamplerFilter;
use engine::SpriteBatch;
use engine::Camera;
use engine::Vertex;

pub struct SpriteRenderer<'a> {
    program: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    draw_parameters: DrawParameters<'a>,
}

impl<'a> SpriteRenderer<'a> {
    pub fn new(display: &Display) -> SpriteRenderer {
        let program = program!(display,
        140 => {
            vertex: "
                #version 140
                uniform mat4 world;
                uniform mat4 projection;
                uniform mat4 view;
                uniform float depth;
                in vec2 position;
                in vec2 tex_coord;
                out vec2 v_texcoord;
                void main() {
                    mat4 wvp = world * view * projection;
                    gl_Position = wvp * vec4(position.xy, -depth, 1.0);
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
                    f_color = tex_color;
                }
            "
        },
        ).unwrap();

        let vertex_buffer = VertexBuffer::empty_dynamic(display, 6_000).unwrap();
        let index_buffer =
            IndexBuffer::empty_dynamic(display, PrimitiveType::TrianglesList, 18_000).unwrap();

        let draw_parameters = DrawParameters {
            blend: Blend::alpha_blending(),
            depth: Depth {
                test: DepthTest::IfMore,
                write: true,
                ..Default::default()
            },
            backface_culling: BackfaceCullingMode::CullingDisabled,
            ..Default::default()
        };

        SpriteRenderer {
            draw_parameters: draw_parameters,
            program: program,
            vertex_buffer: vertex_buffer,
            index_buffer: index_buffer,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, spritebatch: &SpriteBatch, camera: &Camera) {
        let mut ordered = spritebatch.draw_calls.iter().collect::<Vec<_>>();

        ordered.sort_by(|kvp1, kvp2| {
            let (k, _) = *kvp1;
            let (l, _) = *kvp2;
            let &(k, ref s) = k;
            let &(l, ref t) = l;
            k.cmp(&l).then(s.cmp(&t))
        });

        for (key, value) in ordered.into_iter() {
            if !value.indices.is_empty() {
                self.index_buffer.as_mut_slice().write(&value.indices);
                self.vertex_buffer.as_mut_slice().write(&value.quads);
                let &(depth, ref spritesheet) = key;
                let depth = depth as f32;

                // building the uniforms
                let uniforms = uniform! {
                    world: Into::<[[f32;4];4]>::into(camera.world),
                    view: Into::<[[f32;4];4]>::into(camera.view),
                    projection: Into::<[[f32;4];4]>::into(camera.ortho),
                    tex: spritesheet.texture.sampled().magnify_filter(MagnifySamplerFilter::Nearest),
                    depth: depth
                };

                //let vb_slice = self.vertex_buffer.slice(0 .. spritebatch.quads.len()).unwrap();
                let ib_slice = self.index_buffer.slice(0..value.indices.len()).unwrap();

                frame
                    .draw(
                        &self.vertex_buffer,
                        &ib_slice,
                        &self.program,
                        &uniforms,
                        &self.draw_parameters,
                    )
                    .unwrap();
            }
        }
    }
}
