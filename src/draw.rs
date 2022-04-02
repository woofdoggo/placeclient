// primarily based off of various miniquad examples
// https://github.com/not-fl3/miniquad/tree/master/examples

use glam::{vec3, Mat4};
use miniquad::{
    conf::Conf, Bindings, Buffer, BufferLayout, BufferType, Context, EventHandler, Pipeline,
    Shader, ShaderMeta, Texture, UniformBlockLayout, UniformType, UserData, VertexAttribute,
    VertexFormat,
};

#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
pub struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

#[repr(C)]
pub struct Uniforms {
    pub mvp: glam::Mat4,
}

pub struct Stage {
    pub pipeline: Pipeline,
    pub bindings: Bindings,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Self {
        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos: Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0.0, y: 0.0 } },
            Vertex { pos: Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1.0, y: 0.0 } },
            Vertex { pos: Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1.0, y: 1.0 } },
            Vertex { pos: Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0.0, y: 1.0 } },
        ];

        let vtxbuf = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let idxbuf = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let texture_data: [u8; 1000 * 1000 * 4] = [0xAF; 1000 * 1000 * 4];
        let texture = Texture::from_rgba8(ctx, 1000, 1000, &texture_data);

        let bindings = Bindings {
            vertex_buffers: vec![vtxbuf],
            index_buffer: idxbuf,
            images: vec![texture],
        };

        let shader = Shader::new(
            ctx,
            include_str!("vertex.glsl"),
            include_str!("fragment.glsl"),
            ShaderMeta {
                images: &["tex"],
                uniforms: UniformBlockLayout {
                    uniforms: &[("mvp", UniformType::Mat4)],
                },
            },
        );

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
        );

        Stage { pipeline, bindings }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        // to_radians is not const unfortunately
        let fov = 60.0_f32.to_radians();

        let (width, height) = ctx.screen_size();
        let proj = Mat4::perspective_rh_gl(fov, width / height, 0.0, 50.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 1.5, 12.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let mat = proj * view;

        ctx.apply_uniforms(&Uniforms { mvp: mat });
        ctx.draw(0, 6, 1);

        ctx.end_render_pass();
        ctx.commit_frame();
    }
}
