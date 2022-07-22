use miniquad::*;

pub mod display_shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 400
    attribute vec4 pos;
    attribute vec4 color0;
    attribute vec2 uv0;
    varying lowp vec4 color;
    uniform mat4 mvp;
    void main() {
        gl_Position = mvp * pos;
        color = color0;
    }
    "#;

    pub const FRAGMENT: &str = r#"#version 400
    varying lowp vec4 color;
    void main() {
        gl_FragColor = color;
    }
    "#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}