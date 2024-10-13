use std::collections::HashMap;

use glium::{draw_parameters::DepthClamp, framebuffer::MultiOutputFrameBuffer, glutin::{dpi::PhysicalSize, event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Icon, WindowBuilder}, ContextBuilder}, index::PrimitiveType, texture::{DepthTexture2d, Texture2d}, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction, UniformsStorage}, BackfaceCullingMode, Blend, BlendingFunction, Depth, DepthTest, Display, DrawParameters, IndexBuffer, LinearBlendingFactor, Surface, VertexBuffer};

#[allow(dead_code)] mod math;
#[allow(dead_code)] mod entity;
#[allow(dead_code)] mod graphics;
#[allow(dead_code)] mod tiles;
#[allow(dead_code)] mod world;

use math::*;
use entity::*;
use graphics::*;
use tiles::*;
use world::*;


const _PROJECTION_OFFSET: f32 = 0.5;


trait IsPressed {
	fn is_pressed(&self) -> bool;
}
impl IsPressed for ElementState {
	fn is_pressed(&self) -> bool {
		match self {
			ElementState::Pressed => true,
			ElementState::Released => false,
		}
	}
}






fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new()
		.with_title("balls")
		// .with_fullscreen(Some(glium::winit::window::Fullscreen::Borderless(None)))
		.with_maximized(true)
		// .with_inner_size(Some(glium::winit::dpi::Size::Logical(LogicalSize { width: 300.0, height: 300.0 })))
		.with_transparent(true)
		.with_decorations(false)
		.with_resizable(false)
		// .with_window_level(glium::winit::window::WindowLevel::AlwaysOnTop)
		.with_window_icon(Some(Icon::from_rgba(image::load_from_memory_with_format(include_bytes!("../assets/icon.png"), image::ImageFormat::Png).unwrap().resize(64, 64, image::imageops::FilterType::Nearest).into_bytes().to_vec(), 64, 64).unwrap()));
	let cb = ContextBuilder::new()
		.with_vsync(true);
	let display = Display::new(wb, cb, &event_loop).unwrap();
	
	
	let mut tilemap_program = load_shader_program(&display, "tilemap", "tilemap");
	let _screen_texture_program = load_shader_program(&display, "screen_rectangle", "screen_rectangle");
	let world_texture_program = load_shader_program(&display, "world_rectangle", "world_rectangle");
	let mut post_program = load_shader_program(&display, "default", "post_process");
	
	let rect_vertex_buffer = VertexBuffer::new(&display, &[Vec2(0.0f32, 0.0), Vec2(1.0, 0.0), Vec2(1.0, 1.0), Vec2(0.0, 1.0)]).unwrap();
	let rect_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0, 1, 2, 0, 2, 3u8]).unwrap();
	
	
	let PhysicalSize { width: window_width, height: window_height } = display.gl_window().window().inner_size();
	let mut aspect_ratio = window_width as f32 / window_height as f32;
	
	let mut tile_size = 1.0/40.0;
	let tile_depth = 1.0/32.0;
	
	let mut screen_texture = Texture2d::empty(&display, window_width, window_height).unwrap();
	let mut data_texture = Texture2d::empty(&display, window_width, window_height).unwrap();
	let mut depth_texture = DepthTexture2d::empty(&display, window_width, window_height).unwrap();
	
	let tilemap_texture = load_texture(&display, "tilemap");
	
	let mut world = World::new();
	
	let player_pos = world.place_player(Vec3(0.5, 0.5, CELL_HEIGHT as f64));
	world.entities.push(Entity::new(
		player_pos,
		Vec3(0.75, 0.75, 0.75),
		SpriteSet::load(&display, "player")
	));
	
	let mut key_w = false;
	let mut key_a = false;
	let mut key_s = false;
	let mut key_d = false;
	
	let mut key_shift = false;
	let mut key_ctrl = false;
	let mut key_space = false;
	
	let mut previous_frame_time = std::time::Instant::now();
	
	
	event_loop.run(move |event, _window_target, control_flow| {
		match event {
			Event::RedrawEventsCleared => {
				display.gl_window().window().request_redraw();
			}
			Event::WindowEvent { event, window_id: _ } => match event {
				WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
				}
				WindowEvent::Resized(physical_size) => {
					aspect_ratio = physical_size.width as f32 / physical_size.height as f32;
					screen_texture = Texture2d::empty(&display, physical_size.width, physical_size.height).unwrap();
					data_texture = Texture2d::empty(&display, physical_size.width, physical_size.height).unwrap();
					depth_texture = DepthTexture2d::empty(&display, physical_size.width, physical_size.height).unwrap();
				}
				WindowEvent::CursorMoved { position: _, device_id: _, .. } => {
					
				}
				WindowEvent::MouseInput { state, button, device_id: _, .. } => match button {
					MouseButton::Left => if state == ElementState::Pressed {
						// println!("click");
					},
					_ => ()
				}
				WindowEvent::MouseWheel { device_id: _, delta: _, phase: _, .. } => {
					
				}
				WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(keycode), state, scancode: _, .. }, device_id: _, is_synthetic: _} => {
					match keycode {
						VirtualKeyCode::W => key_w = state.is_pressed(),
						VirtualKeyCode::S => key_s = state.is_pressed(),
						VirtualKeyCode::A => key_a = state.is_pressed(),
						VirtualKeyCode::D => key_d = state.is_pressed(),
						VirtualKeyCode::LShift | VirtualKeyCode::RShift => key_shift = state.is_pressed(),
						VirtualKeyCode::LControl | VirtualKeyCode::RControl => key_ctrl = state.is_pressed(),
						VirtualKeyCode::Space => key_space = state.is_pressed(),
						VirtualKeyCode::Up => if state.is_pressed() {
							world.entities[0].velocity.1 -= 30.0;
						}
						VirtualKeyCode::Down => if state.is_pressed() {
							world.entities[0].velocity.1 += 30.0;
						}
						VirtualKeyCode::Left => if state.is_pressed() {
							world.entities[0].velocity.0 -= 30.0;
						}
						VirtualKeyCode::Right => if state.is_pressed() {
							world.entities[0].velocity.0 += 30.0;
						}
						VirtualKeyCode::Minus => if state.is_pressed() {
							tile_size /= 1.1;
						}
						VirtualKeyCode::Equals => if state.is_pressed() {
							tile_size *= 1.1;
						}
						VirtualKeyCode::R => if state.is_pressed() {
							if key_ctrl {
								tilemap_program = load_shader_program(&display, "tilemap", "tilemap");
								post_program = load_shader_program(&display, "default", "post_process");
							} else {
								if key_shift {
									let mut w = World::new();
									w.entities.append(&mut world.entities.drain(0..=0).collect());
									world = w;
								}
								
								world.entities[0].position = world.place_player(Vec3(0.5, 0.5, CELL_HEIGHT as f64));
								world.entities[0].velocity = Vec3(0.0, 0.0, 0.0);
							}
						}
						VirtualKeyCode::Escape => if state.is_pressed() {
							*control_flow = ControlFlow::Exit;
						}
						_ => ()
					}
				}
				_ => ()
			}
			Event::RedrawRequested(_) => {
				let now = std::time::Instant::now();
				let dt = now.duration_since(previous_frame_time).as_secs_f64();
				previous_frame_time = now;
				
				let cell_position = world.entities[0].position.scale_divide(CELL_SIZE.as_type::<f64>()) - Vec3(0.5, 0.5, 0.0);
				
				for (pos, cell) in &mut world.cells {
					if (pos.x() as f64 - cell_position.x()).abs() > 1.25
					|| (pos.y() as f64 - cell_position.y()).abs() > 1.25 {
						cell.unload = true;
					}
				}
				
				world.unload_flagged();
				
				for pos in Vec3Range::<isize, ZYX>::inclusive((cell_position - Vec3(0.0, 0.0, 0.0)).floor_to().with_z(0), (cell_position + Vec3(1.0, 1.0, 0.0)).floor_to().with_z(0)) {
					world.get_or_load(pos);
				}
				
				
				let mut dp = Vec3(0.0, 0.0, 0.0f64);
				if key_w { dp.1 -= 1.0; }
				if key_s { dp.1 += 1.0; }
				if key_a { dp.0 -= 1.0; }
				if key_d { dp.0 += 1.0; }
				world.entities[0].movement_input = dp.normalize_or_zero();
				
				world.entities[0].jump_input = key_space;
				
				for entity in &mut world.entities {
					entity.physics_step(&world.cells, dt);
				}
				
				
				
				world.update_buffers(&display);
				
				let mut target = MultiOutputFrameBuffer::with_depth_buffer(&display, [
					("color", &screen_texture),
					("data", &data_texture),
				], &depth_texture).unwrap();
				target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 0.0);
				
				// MARK: Draw Tilemap
				for (location, cell) in &world.cells {
					if let Some((vertex_buffer, index_buffer)) = &cell.buffers {
						target.draw(vertex_buffer, index_buffer, &tilemap_program, &UniformsStorage::
							 new("tile_size", Vec3(tile_size, tile_size * aspect_ratio, tile_depth))
							.add("cell_position", (*location << CELL_SIZE_BITS).as_type::<f32>() - world.entities[0].position.as_type::<f32>())
							.add("tilemap_texture", Sampler(&tilemap_texture, SamplerBehavior {
								wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
								minify_filter: MinifySamplerFilter::Linear,
								magnify_filter: MagnifySamplerFilter::Nearest,
								depth_texture_comparison: None,
								max_anisotropy: 1,
							}))
						, &DrawParameters {
							backface_culling: BackfaceCullingMode::CullClockwise,
							blend: Blend {
								color: BlendingFunction::Addition {
									source: LinearBlendingFactor::SourceAlpha,
									destination: LinearBlendingFactor::OneMinusSourceAlpha,
								},
								alpha: BlendingFunction::Addition {
									source: LinearBlendingFactor::One,
									destination: LinearBlendingFactor::OneMinusSourceAlpha,
								},
								constant_value: (0.0, 0.0, 0.0, 0.0)
							},
							depth: Depth {
								test: DepthTest::IfMoreOrEqual,
								write: true,
								range: (0.0, 1.0),
								clamp: DepthClamp::NoClamp,
							},
							.. Default::default()
						}).unwrap();
					}
				}
				
				
				// MARK: Draw Entities
				world.entities.iter().rev().for_each(|entity| {
					target.draw(&rect_vertex_buffer, &rect_index_buffer, &world_texture_program, &UniformsStorage::
						 new("texture_position", entity.position.as_type::<f32>() + entity.size.scale(LOW_CORNER).as_type::<f32>().add_z(0.01))
						.add("texture_size", entity.size.xy().as_type::<f32>())
						.add("render_position", world.entities[0].position.as_type::<f32>())
						.add("tile_size", Vec3(tile_size, tile_size * aspect_ratio, tile_depth))
						.add("tex", Sampler(entity.current_sprite(), SamplerBehavior {
							wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
							minify_filter: MinifySamplerFilter::Linear,
							magnify_filter: MagnifySamplerFilter::Nearest,
							depth_texture_comparison: None,
							max_anisotropy: 1,
						}))
						// .add("data_texture", Sampler(&data_texture, SamplerBehavior {
						// 	wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
						// 	minify_filter: MinifySamplerFilter::Nearest,
						// 	magnify_filter: MagnifySamplerFilter::Nearest,
						// 	depth_texture_comparison: None,
						// 	max_anisotropy: 1
						// }))
					, &DrawParameters {
						blend: Blend::alpha_blending(),
						depth: Depth {
							test: DepthTest::IfMoreOrEqual,
							write: true,
							range: (0.0, 1.0),
							clamp: DepthClamp::NoClamp,
						},
						..Default::default()
					}).unwrap();
				});
				
				
				// MARK: Debug noise renderer
				// let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
				
				// let mut noise_data = vec![vec![0.0; window_width as usize]; window_height as usize];
				// for y in 0..window_height as usize {
				// 	for x in 0..window_width as usize {
				// 		noise_data[window_height as usize - (y + 1)][x] = perlin_noise(Vec2(x as f64 / 100.0, y as f64 / 100.0), seed) as f32 + 0.5;
				// 	}
				// }
				
				// screen_texture = Texture2d::with_format(&display, noise_data, glium::texture::UncompressedFloatFormat::F32, MipmapsOption::NoMipmap).unwrap();
				
				
				// MARK: Post-processing
				let mut display_target = display.draw();
				display_target.draw(&rect_vertex_buffer, &rect_index_buffer, &post_program, &UniformsStorage::
					 new("aspect_ratio", aspect_ratio)
					.add("screen_texture", Sampler(&screen_texture, SamplerBehavior {
						wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
						minify_filter: MinifySamplerFilter::Nearest,
						magnify_filter: MagnifySamplerFilter::Nearest,
						depth_texture_comparison: None,
						max_anisotropy: 1
					}))
					.add("data_texture", Sampler(&data_texture, SamplerBehavior {
						wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
						minify_filter: MinifySamplerFilter::Nearest,
						magnify_filter: MagnifySamplerFilter::Nearest,
						depth_texture_comparison: None,
						max_anisotropy: 1
					}))
					.add("depth_texture", Sampler(&depth_texture, SamplerBehavior {
						wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
						minify_filter: MinifySamplerFilter::Nearest,
						magnify_filter: MagnifySamplerFilter::Nearest,
						depth_texture_comparison: None,
						max_anisotropy: 1
					}))
				, &DrawParameters::default()).unwrap();
				
				display_target.finish().unwrap();
				
				
				
			}
			_ => ()
		}
	});
}
