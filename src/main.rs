use winit::{dpi::LogicalSize, event::{Event, KeyEvent, MouseButton, WindowEvent}, keyboard::{KeyCode, PhysicalKey}, window::{Icon, WindowBuilder}};
use glium::{index::PrimitiveType, texture::{MipmapsOption, UncompressedUintFormat, UnsignedTexture2d}, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction, UniformsStorage}, IndexBuffer, Surface, VertexBuffer};

#[allow(dead_code)] mod vec;
#[allow(dead_code)] mod entity;
#[allow(dead_code)] mod graphics;

use vec::*;
use entity::*;
use graphics::*;






fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().set_window_builder(WindowBuilder::new()
        .with_title("balls")
        // .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_maximized(true)
        // .with_inner_size(LogicalSize { width: 300, height: 300 })
        // .with_transparent(true)
        .with_decorations(false)
        .with_resizable(false)
        // .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
        .with_window_icon(Some(Icon::from_rgba(image::load_from_memory_with_format(include_bytes!("../assets/icon.png"), image::ImageFormat::Png).unwrap().resize(64, 64, image::imageops::FilterType::Nearest).into_bytes().to_vec(), 64, 64).unwrap()))
    ).build(&event_loop);
    
    
    let mut tilemap_program = load_shader_program(&display, "tilemap", "tilemap");
    let _screen_texture_program = load_shader_program(&display, "screen_rectangle", "rectangle");
    let world_texture_program = load_shader_program(&display, "world_rectangle", "rectangle");
    
    let rect_vertex_buffer = VertexBuffer::new(&display, &[Vec2(0.0, 0.0), Vec2(1.0, 0.0), Vec2(1.0, 1.0), Vec2(0.0, 1.0)]).unwrap();
    let rect_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0, 1, 2, 0, 2, 3u8]).unwrap();
    
    
    
    let LogicalSize { width: window_width, height: window_height }: LogicalSize<f32> = window.inner_size().to_logical(window.scale_factor());
    let mut aspect_ratio = window_height / window_width;
    
    let mut tile_size = 1.0/20.0;
    let camera_position = Vec3(0.0, 0.0, 0.0);
    
    let tilemap_texture = load_texture(&display, "tilemap");
    
    
    let mut entities = vec![];
    
    entities.push(Entity::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(0.75, 0.75, 0.75),
        Direction::Down,
        5.0,
        120.0,
        vec![
            load_texture(&display, "player/up"),
            load_texture(&display, "player/down"),
            load_texture(&display, "player/left"),
            load_texture(&display, "player/right"),
        ]
    ));
    
    
    let mut key_w = false;
    let mut key_a = false;
    let mut key_s = false;
    let mut key_d = false;
    
    let mut previous_frame_time = std::time::Instant::now();
    
    
    event_loop.run(move |event, window_target| {
        match event {
            Event::Resumed => {
                
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { event, window_id: _ } => match event {
                WindowEvent::CloseRequested => {
                    window_target.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    let LogicalSize { width: window_width, height: window_height }: LogicalSize<f32> = physical_size.to_logical(window.scale_factor());
                    aspect_ratio = window_height / window_width;
                }
                WindowEvent::CursorMoved { position: _, device_id: _ } => {
                    
                }
                WindowEvent::MouseInput { state, button, device_id: _ } => match button {
                    MouseButton::Left => if state.is_pressed() {
                        
                    },
                    _ => ()
                }
                WindowEvent::KeyboardInput { event: KeyEvent { physical_key: PhysicalKey::Code(code), state, .. }, device_id: _, is_synthetic: _ } => {
                    match code {
                        KeyCode::KeyW => key_w = state.is_pressed(),
                        KeyCode::KeyS => key_s = state.is_pressed(),
                        KeyCode::KeyA => key_a = state.is_pressed(),
                        KeyCode::KeyD => key_d = state.is_pressed(),
                        KeyCode::ArrowUp => if state.is_pressed() {
                            entities[0].velocity.1 -= 30.0;
                        }
                        KeyCode::ArrowDown => if state.is_pressed() {
                            entities[0].velocity.1 += 30.0;
                        }
                        KeyCode::ArrowLeft => if state.is_pressed() {
                            entities[0].velocity.0 -= 30.0;
                        }
                        KeyCode::ArrowRight => if state.is_pressed() {
                            entities[0].velocity.0 += 30.0;
                        }
                        KeyCode::Minus => if state.is_pressed() {
                            tile_size /= 1.1;
                        }
                        KeyCode::Equal => if state.is_pressed() {
                            tile_size *= 1.1;
                        }
                        KeyCode::KeyR => if state.is_pressed() {
                            tilemap_program = load_shader_program(&display, "tilemap", "tilemap");
                        }
                        KeyCode::Escape => if state.is_pressed() {
                            window_target.exit();
                        }
                        _ => ()
                    }
                }
                WindowEvent::RedrawRequested => {
                    let now = std::time::Instant::now();
                    let dt = now.duration_since(previous_frame_time).as_secs_f32();
                    previous_frame_time = now;
                    
                    let mut dp = Vec3::ZERO;
                    if key_w { dp.1 -= 1.0; }
                    if key_s { dp.1 += 1.0; }
                    if key_a { dp.0 -= 1.0; }
                    if key_d { dp.0 += 1.0; }
                    if !dp.is_zero() {
                        entities[0].input_move(dp.normalize(), dt);
                    }
                    
                    for entity in &mut entities {
                        entity.physics_step(dt);
                    }
                    
                    
                    
                    
                    let screen_width_in_tiles = 1.0 / tile_size;
                    
                    let render_size = screen_width_in_tiles * Vec2(1.0, aspect_ratio);
                    let render_position = camera_position.xy() - 0.5 * render_size;
                    
                    let mut tile_data_buffer = vec![];
                    for y in 0..=render_size.y().ceil() as u32 {
                        tile_data_buffer.push(vec![]);
                        for x in 0..=render_size.x().ceil() as u32 {
                            tile_data_buffer[y as usize].push(((x as i32 + render_position.x().floor() as i32) as u16, (y as i32 + render_position.y().floor() as i32) as u16));
                        }
                    }
                    
                    let tile_data_texture = UnsignedTexture2d::with_format(&display, tile_data_buffer, UncompressedUintFormat::U16U16, MipmapsOption::NoMipmap).unwrap();
                    
                    
                    
                    
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 0.0);
                    
                    target.draw(&rect_vertex_buffer, &rect_index_buffer, &tilemap_program, &UniformsStorage::
                         new("aspect_ratio", aspect_ratio)
                        .add("screen_width_in_tiles", screen_width_in_tiles)
                        .add("offset", ((render_position % 1.0) + Vec2(1.0, 1.0)) % 1.0)
                        .add("tile_data_texture", &tile_data_texture)
                        .add("tilemap_texture", Sampler(&tilemap_texture, SamplerBehavior {
                            wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
                            minify_filter: MinifySamplerFilter::Linear,
                            magnify_filter: MagnifySamplerFilter::Nearest,
                            depth_texture_comparison: None,
                            max_anisotropy: 1,
                        }))
                    , &glium::DrawParameters::default()).unwrap();
                    
                    
                    let render_size_inverse = 1.0 / render_size;
                    
                    entities.iter().rev().for_each(|entity| {
                        target.draw(&rect_vertex_buffer, &rect_index_buffer, &world_texture_program, &UniformsStorage::
                            new("texture_position", entity.position.xy() + Vec2(0.0, -0.7 * entity.position.z()) - 0.5 * entity.size.xy())
                           .add("texture_size", entity.size.xy())
                           .add("render_position", render_position)
                           .add("render_size_inverse", render_size_inverse)
                           .add("tex", Sampler(&entity.textures[0], SamplerBehavior {
                               wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
                               minify_filter: MinifySamplerFilter::Linear,
                               magnify_filter: MagnifySamplerFilter::Nearest,
                               depth_texture_comparison: None,
                               max_anisotropy: 1,
                           }))
                        , &glium::DrawParameters {
                            blend: glium::Blend::alpha_blending(),
                            ..Default::default()
                        }).unwrap();
                    });
                    
                    target.finish().unwrap();
                    
                }
                _ => ()
            }
            _ => ()
        }
    }).unwrap();
}
