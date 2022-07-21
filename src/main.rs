// use miniquad::*;

// use glam::{vec3, Mat4};

// struct Stage {
//     pipeline: Pipeline,
//     bind: Bindings,
// }

// impl Stage {
//     pub fn new(ctx: &mut Context) -> Stage {
//         #[rustfmt::skip]
//         let vertices: &[f32] = &[
//             /* pos               color                   uvs */
//             -1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     0.0, 0.0,
//              1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     1.0, 0.0,
//              1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     1.0, 1.0,
//             -1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     0.0, 1.0,

//             -1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     0.0, 0.0,
//              1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     1.0, 0.0,
//              1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     1.0, 1.0,
//             -1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     0.0, 1.0,

//             -1.0, -1.0, -1.0,    0.5, 0.5, 1.0, 1.0,     0.0, 0.0,
//             -1.0,  1.0, -1.0,    0.5, 0.5, 1.0, 1.0,     1.0, 0.0,
//             -1.0,  1.0,  1.0,    0.5, 0.5, 1.0, 1.0,     1.0, 1.0,
//             -1.0, -1.0,  1.0,    0.5, 0.5, 1.0, 1.0,     0.0, 1.0,

//              1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,     0.0, 0.0,
//              1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,     1.0, 0.0,
//              1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,     1.0, 1.0,
//              1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,     0.0, 1.0,

//             -1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,     0.0, 0.0,
//             -1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,     1.0, 0.0,
//              1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,     1.0, 1.0,
//              1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,     0.0, 1.0,

//             -1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,     0.0, 0.0,
//             -1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,     1.0, 0.0,
//              1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,     1.0, 1.0,
//              1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,     0.0, 1.0
//         ];

//         let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

//         #[rustfmt::skip]
//         let indices: &[u16] = &[
//             0, 1, 2,  0, 2, 3,
//             6, 5, 4,  7, 6, 4,
//             8, 9, 10,  8, 10, 11,
//             14, 13, 12,  15, 14, 12,
//             16, 17, 18,  16, 18, 19,
//             22, 21, 20,  23, 22, 20
//         ];

//         let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

//         let color_img = Texture::new_render_texture(
//             ctx,
//             TextureParams {
//                 width: 256,
//                 height: 256,
//                 format: TextureFormat::RGBA8,
//                 ..Default::default()
//             },
//         );

//         let bind = Bindings {
//             vertex_buffers: vec![vertex_buffer],
//             index_buffer,
//             images: vec![color_img],
//         };

//         let default_shader = Shader::new(
//             ctx,
//             shader::VERTEX,
//             shader::FRAGMENT,
//             shader::meta(),
//         )
//         .unwrap();

//         let pipeline = Pipeline::with_params(
//             ctx,
//             &[BufferLayout::default()],
//             &[
//                 VertexAttribute::new("pos", VertexFormat::Float3),
//                 // VertexAttribute::new("color0", VertexFormat::Float4),
//                 // VertexAttribute::new("uv0", VertexFormat::Float2),
//                 VertexAttribute::new("uv", VertexFormat::Float2),
//             ],
//             default_shader,
//             PipelineParams {
//                 depth_test: Comparison::LessOrEqual,
//                 depth_write: true,
//                 ..Default::default()
//             },
//         );

//         Stage {
//             pipeline,
//             bind,
//         }
//     }
// }

// impl EventHandler for Stage {
//     fn update(&mut self, _ctx: &mut Context) {}

//     fn draw(&mut self, ctx: &mut Context) {
//         let (width, height) = ctx.screen_size();
//         let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 10.0);
//         let view = Mat4::look_at_rh(
//             vec3(0.0, 1.5, 3.0),
//             vec3(0.0, 0.0, 0.0),
//             vec3(0.0, 1.0, 0.0),
//         );
//         let view_proj = proj * view;

//         ctx.begin_default_pass(PassAction::clear_color(0.0, 0., 0.45, 1.));
//         ctx.apply_pipeline(&self.pipeline);
//         ctx.apply_bindings(&self.bind);
//         // ctx.apply_uniforms(&vs_params);
//         ctx.draw(0, 36, 1);
//         ctx.end_render_pass();
//         ctx.commit_frame();
//     }
// }

// fn main() {
//     miniquad::start(
//         conf::Conf {
//             window_title: "Miniquad".to_string(),
//             ..Default::default()
//         },
//         |mut ctx| Box::new(Stage::new(&mut ctx)),
//     );
// }

// mod shader {
//     use miniquad::*;

//     pub const VERTEX: &str = r#"#version 100
// 	attribute vec3 pos;
// 	attribute vec2 texcoord;

// 	varying lowp vec2 uv;
	
// 	uniform mat4 Model;
// 	uniform mat4 Projection;
	
// 	void main() {
// 		gl_Position = Projection * Model * vec4(pos, 1);
// 		uv = texcoord;
//     }"#;

//     pub const FRAGMENT: &str = r#"#version 100
//     varying lowp vec2 texcoord;
//     uniform sampler2D tex;
//     void main() {
//         gl_FragColor = texture2D(tex, texcoord);
//     }"#;

//     pub fn meta() -> ShaderMeta {
//         ShaderMeta {
//             images: vec!["tex".to_string()],
//             uniforms: UniformBlockLayout {
//                 uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
//             },
//         }
//     }

//     #[repr(C)]
//     pub struct Uniforms {
//         // pub offset: (f32, f32),
//     }
// }

use glam::vec3;
use glam::Mat4;
use miniquad::*;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            /* pos               color                   uvs */
            -1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     0.0, 0.0,
             1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     1.0, 1.0,
            -1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,     0.0, 1.0,

            -1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     0.0, 0.0,
             1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     1.0, 1.0,
            -1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 1.0,     0.0, 1.0,

            -1.0, -1.0, -1.0,    0.5, 0.5, 1.0, 1.0,     0.0, 0.0,
            -1.0,  1.0, -1.0,    0.5, 0.5, 1.0, 1.0,     1.0, 0.0,
            -1.0,  1.0,  1.0,    0.5, 0.5, 1.0, 1.0,     1.0, 1.0,
            -1.0, -1.0,  1.0,    0.5, 0.5, 1.0, 1.0,     0.0, 1.0,

             1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,     0.0, 0.0,
             1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,     1.0, 1.0,
             1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,     0.0, 1.0,

            -1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,     0.0, 0.0,
            -1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,     1.0, 0.0,
             1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,     1.0, 1.0,
             1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,     0.0, 1.0,

            -1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,     0.0, 0.0,
            -1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,     1.0, 0.0,
             1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,     1.0, 1.0,
             1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,     0.0, 1.0
        ];
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        #[rustfmt::skip]
        let indices: &[u16] = &[
            0, 1, 2,  0, 2, 3,
            6, 5, 4,  7, 6, 4,
            8, 9, 10,  8, 10, 11,
            14, 13, 12,  15, 14, 12,
            16, 17, 18,  16, 18, 19,
            22, 21, 20,  23, 22, 20
        ];
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);
	
        let texture = Texture::empty();
        // let texture = Texture::from_rgba8(ctx, 4, 4, &pixels);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
			PipelineParams {
				cull_face: CullFace::Nothing,
				// front_face_order: todo!(),
				depth_test: Comparison::Always,
				// depth_write: todo!(),
				// depth_write_offset: todo!(),
				color_blend: Some(BlendState::new(
					Equation::Add,
					BlendFactor::Value(BlendValue::SourceAlpha),
					BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
				)),
				alpha_blend: Some(BlendState::new(
					Equation::Add,
					BlendFactor::Zero,
					BlendFactor::One)
				),
				// stencil_test: todo!(),
				// color_write: todo!(),
				primitive_type: PrimitiveType::Triangles,
				..Default::default()
			},
        );

        Stage { pipeline, bindings }
    }
}

impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        let (width, height) = ctx.screen_size();
        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 10.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 1.5, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );
        let view_proj = proj * view;

        ctx.begin_default_pass(PassAction::clear_color(0.6, 0.6, 0.6, 1.0));

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
		ctx.apply_uniforms(&shader::Uniforms {
			mvp: view_proj,
		});
		ctx.draw(0, 36, 1);
        ctx.end_render_pass();

        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        Box::new(Stage::new(&mut ctx))
    });
}

mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec4 pos;
    attribute vec4 color0;
    attribute vec2 uv0;
    varying lowp vec4 color;
    varying lowp vec2 uv;
    uniform mat4 mvp;
    void main() {
        gl_Position = mvp * pos;
        color = color0;
        uv = uv0;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec4 color;
    varying lowp vec2 uv;
    uniform sampler2D tex;
    void main() {
        gl_FragColor = color * texture2D(tex, uv);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}