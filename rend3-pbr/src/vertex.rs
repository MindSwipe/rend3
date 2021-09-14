use wgpu::{VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

pub const fn cpu_vertex_buffers() -> [VertexBufferLayout<'static>; 7] {
    [
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_POSITION_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_NORMAL_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 1,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_TANGENT_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 2,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_UV_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 3,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_UV_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 4,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_COLOR_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Unorm8x4,
                offset: 0,
                shader_location: 5,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_MATERIAL_INDEX_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Uint32,
                offset: 0,
                shader_location: 6,
            }],
        },
    ]
}

pub const fn gpu_vertex_buffers() -> [VertexBufferLayout<'static>; 8] {
    [
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_POSITION_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_NORMAL_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 1,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_TANGENT_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 2,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_UV_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 3,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_UV_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: 0,
                shader_location: 4,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_COLOR_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Unorm8x4,
                offset: 0,
                shader_location: 5,
            }],
        },
        VertexBufferLayout {
            array_stride: rend3::resources::VERTEX_MATERIAL_INDEX_SIZE as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                format: VertexFormat::Uint32,
                offset: 0,
                shader_location: 6,
            }],
        },
        VertexBufferLayout {
            array_stride: 20,
            step_mode: VertexStepMode::Instance,
            attributes: &[VertexAttribute {
                format: VertexFormat::Uint32,
                offset: 16,
                shader_location: 7,
            }],
        },
    ]
}
