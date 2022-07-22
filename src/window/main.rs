use glam::Mat4;
use glam::vec3;
use miniquad::*;

use crate::shader::display_shader;

pub struct Main {
	pipeline: Pipeline,
	bind: Bindings,
	color_img: Texture,
}

impl Main {
	fn new(ctx: &mut Context) -> Self {
		let color_img = Texture::new_render_texture(
			ctx,
			TextureParams {
				width: 256,
				height: 256,
				format: TextureFormat::RGBA8,
				..Default::default()
			},
		);

		#[rustfmt::skip]
		let vertices: &[f32] = &[
			/*           pos     color            */
			-1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,
			 1.0, -1.0, -1.0,    1.0, 0.5, 0.5, 1.0,
			 1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,
			-1.0,  1.0, -1.0,    1.0, 0.5, 0.5, 1.0,

			-1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 0.5,
			 1.0, -1.0,  1.0,    0.5, 1.0, 0.5, 0.0,
			 1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 0.0,
			-1.0,  1.0,  1.0,    0.5, 1.0, 0.5, 0.0,

			-1.0, -1.0, -1.0,    0.5, 0.5, 1.0, 1.0,
			-1.0,  1.0, -1.0,    0.5, 0.5, 1.0, 1.0,
			-1.0,  1.0,  1.0,    0.5, 0.5, 1.0, 1.0,
			-1.0, -1.0,  1.0,    0.5, 0.5, 1.0, 1.0,

			 1.0, -1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
			 1.0,  1.0, -1.0,    1.0, 0.5, 0.0, 1.0,
			 1.0,  1.0,  1.0,    1.0, 0.5, 0.0, 1.0,
			 1.0, -1.0,  1.0,    1.0, 0.5, 0.0, 1.0,

			-1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,
			-1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,
			 1.0, -1.0,  1.0,    0.0, 0.5, 1.0, 1.0,
			 1.0, -1.0, -1.0,    0.0, 0.5, 1.0, 1.0,

			-1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,
			-1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,
			 1.0,  1.0,  1.0,    1.0, 0.0, 0.5, 1.0,
			 1.0,  1.0, -1.0,    1.0, 0.0, 0.5, 1.0,
		];

		let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

		#[rustfmt::skip]
		let indices: &[u16] = &[
				0,  1,  2,   0,  2,  3,
				6,  5,  4,   7,  6,  4,
				8,  9, 10,   8, 10, 11,
			14, 13, 12,  15, 14, 12,
			16, 17, 18,  16, 18, 19,
			22, 21, 20,  23, 22, 20
		];

		let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

		let display_bind = Bindings {
			vertex_buffers: vec![vertex_buffer],
			index_buffer: index_buffer,
			images: vec![color_img],
		};

		let default_shader = Shader::new(
			ctx,
			display_shader::VERTEX,
			display_shader::FRAGMENT,
			display_shader::meta(),
		)
		.unwrap();

		let display_pipeline = Pipeline::with_params(
			ctx,
			&[BufferLayout::default()],
			&[
				VertexAttribute::new("pos", VertexFormat::Float3),
				VertexAttribute::new("color0", VertexFormat::Float4),
			],
			default_shader,
			PipelineParams {
				cull_face: CullFace::Nothing,
				depth_test: Comparison::LessOrEqual,
				depth_write: true,
				primitive_type: PrimitiveType::Triangles,
				..Default::default()
			}
			// PipelineParams {
			// 	depth_test: Comparison::GreaterOrEqual,
			// 	depth_write: true,
			// 	..Default::default()
			// }
		);

		Self {
			pipeline: display_pipeline,
			bind: display_bind,
			color_img,
		}
	}

	pub fn init() {
		start(
			conf::Conf {
				window_title: "Miniquad".to_string(),
				..Default::default()
			},
			|mut ctx| Box::new(Self::new(&mut ctx)),
		);
	}
}

impl EventHandler for Main {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        let (width, height) = ctx.screen_size();
        let proj = Mat4::perspective_rh_gl(60.0f32.to_radians(), width / height, 0.01, 10.0);
        let view = Mat4::look_at_rh(
            vec3(0.0, 1.5, 3.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        let vs_params = display_shader::Uniforms { mvp: proj * view };

        ctx.begin_default_pass(PassAction::clear_color(0.0, 0., 0.45, 1.));
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bind);
        ctx.apply_uniforms(&vs_params);
        ctx.draw(0, 36, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}