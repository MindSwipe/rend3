use std::time::Instant;

fn main() {
    wgpu_subscriber::initialize_default_subscriber(None);

    let el = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("rend3 imgui example")
        .build(&el)
        .unwrap();

    let window_size = window.inner_size();

    let iad = pollster::block_on(rend3::create_iad(None, None, None)).expect("Could not block on creation of iad");
    let surface = unsafe { iad.instance.create_surface(&window) };
    let format = surface.get_preferred_format(&iad.adapter).expect("Could not get preferred format");
    rend3::configure_surface(
        &surface,
        &iad.device,
        format,
        glam::UVec2::new(window_size.width, window_size.height),
        rend3::types::PresentMode::Mailbox,
    );

    let renderer = rend3::Renderer::new(iad, Some(window_size.width as f32 / window_size.height as f32)).expect("Could not create Renderer");

    let mut routine = rend3_pbr::PbrRenderRoutine::new(
        &renderer,
        rend3_pbr::RenderTextureOptions {
            resolution: glam::UVec2::new(window_size.width, window_size.height),
            samples: rend3_pbr::SampleCount::Four,
        },
        format,
    );

    let mut imgui = imgui::Context::create();
    let mut imgui_platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    imgui_platform.attach_window(imgui.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);
    imgui.set_ini_filename(None);
    imgui.fonts().add_font(&[imgui::FontSource::DefaultFontData {
        config: Some(imgui::FontConfig {
            oversample_h: 3,
            oversample_v: 1,
            pixel_snap_h: true,
            size_pixels: 13.0,
            ..imgui::FontConfig::default()
        })
    }]);

    let imgui_renderer_config = imgui_wgpu::RendererConfig {
        texture_format: format,
        ..Default::default()
    };
    let mut imgui_renderer = imgui_wgpu::Renderer::new(&mut imgui, &renderer.device, &renderer.queue, imgui_renderer_config);

    let camera = rend3::types::Camera {
        projection: rend3::types::CameraProjection::Projection {
            vfov: 60.0,
            near: 0.1,
            pitch: 0.5,
            yaw: -0.55,
        },
        location: glam::Vec3A::new(3.0, 3.0, -5.0),
    };

    renderer.set_camera_data(camera);

    let _directional_handle = renderer.add_directional_light(rend3::types::DirectionalLight {
        color: glam::Vec3::ONE,
        intensity: 10.0,
        // Direction will be normalized
        direction: glam::Vec3::new(-1.0, -4.0, 2.0),
        distance: 400.0,
    });

    let _cube_handle = add_cube(&renderer);

    let mut last_frame = Instant::now();
    let mut demo_open = true;
    let mut last_cursor = None;
    // let size = glam::UVec2::new(window_size.width, window_size.height);

    el.run(move |event, _, control| {
        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                *control = winit::event_loop::ControlFlow::Exit;
            },
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::Resized(size),
                ..
            } => {
                let size = glam::UVec2::new(size.width, size.height);
                rend3::configure_surface(
                    &surface,
                    &renderer.device,
                    format,
                    size,
                    rend3::types::PresentMode::Mailbox,
                );

                renderer.set_aspect_ratio(size.x as f32 / size.y as f32);
            },
            winit::event::Event::MainEventsCleared => {
                let delta = {
                    let now = Instant::now();
                    let delta = now - last_frame;
                    last_frame = now;

                    delta
                };

                imgui.io_mut().update_delta_time(delta);
                imgui_platform
                    .prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let imgui_ui = imgui.frame();
                imgui_ui.show_demo_window(&mut demo_open);

                if last_cursor != Some(imgui_ui.mouse_cursor()) {
                    last_cursor = Some(imgui_ui.mouse_cursor());
                    imgui_platform.prepare_render(&imgui_ui, &window);
                }

                let frame = rend3::util::output::OutputFrame::from_surface(&surface).expect("Failed to get output frame from surface");
                let _stats = renderer.render(&mut routine, &frame);

                let mut encoder = renderer
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("imgui renderer"),
                    });

                // let frame = rend3::util::output::OutputFrame::from_surface(&surface).expect("Failed to get output frame from surface");
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: frame.as_view(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                    label: Some("imgui render pass descriptor"),
                });

                imgui_renderer
                    .render(imgui_ui.render(), &renderer.queue, &renderer.device, &mut rpass)
                    .expect("imgui rendering failed");

                drop(rpass);
                renderer.queue.submit(Some(encoder.finish()));
            }
            _ => {}
        }

        imgui_platform.handle_event(imgui.io_mut(), &window, &event);
    });
}

fn add_cube(renderer: &rend3::Renderer) -> rend3::types::ResourceHandle<rend3::types::Object> {
    let mut mesh = create_mesh();
    mesh.calculate_normals();

    let mesh_handle = renderer.add_mesh(mesh);
    
    let material = rend3::types::Material {
        albedo: rend3::types::AlbedoComponent::Value(glam::Vec4::new(0.0, 0.5, 0.5, 1.0)),
        ..Default::default()
    };
    let material_handle = renderer.add_material(material);

    let object = rend3::types::Object {
        mesh: mesh_handle,
        material: material_handle,
        transform: glam::Mat4::IDENTITY,
    };

    renderer.add_object(object)
}

fn create_mesh() -> rend3::types::Mesh {
    let vertex_positions = [
        // far side (0.0, 0.0, 1.0)
        vertex([-1.0, -1.0, 1.0]),
        vertex([1.0, -1.0, 1.0]),
        vertex([1.0, 1.0, 1.0]),
        vertex([-1.0, 1.0, 1.0]),
        // near side (0.0, 0.0, -1.0)
        vertex([-1.0, 1.0, -1.0]),
        vertex([1.0, 1.0, -1.0]),
        vertex([1.0, -1.0, -1.0]),
        vertex([-1.0, -1.0, -1.0]),
        // right side (1.0, 0.0, 0.0)
        vertex([1.0, -1.0, -1.0]),
        vertex([1.0, 1.0, -1.0]),
        vertex([1.0, 1.0, 1.0]),
        vertex([1.0, -1.0, 1.0]),
        // left side (-1.0, 0.0, 0.0)
        vertex([-1.0, -1.0, 1.0]),
        vertex([-1.0, 1.0, 1.0]),
        vertex([-1.0, 1.0, -1.0]),
        vertex([-1.0, -1.0, -1.0]),
        // top (0.0, 1.0, 0.0)
        vertex([1.0, 1.0, -1.0]),
        vertex([-1.0, 1.0, -1.0]),
        vertex([-1.0, 1.0, 1.0]),
        vertex([1.0, 1.0, 1.0]),
        // bottom (0.0, -1.0, 0.0)
        vertex([1.0, -1.0, 1.0]),
        vertex([-1.0, -1.0, 1.0]),
        vertex([-1.0, -1.0, -1.0]),
        vertex([1.0, -1.0, -1.0]),
    ];

    let index_data: &[u32] = &[
        0, 1, 2, 2, 3, 0, // far
        4, 5, 6, 6, 7, 4, // near
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // top
        20, 21, 22, 22, 23, 20, // bottom
    ];

    rend3::types::MeshBuilder::new(vertex_positions.to_vec())
        .with_indices(index_data.to_vec())
        .build()
}

fn vertex(pos: [f32; 3]) -> glam::Vec3 {
    glam::Vec3::from(pos)
}