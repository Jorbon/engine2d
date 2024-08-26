use std::collections::HashMap;

use glium::{framebuffer::MultiOutputFrameBuffer, glutin::{dpi::PhysicalSize, event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Icon, WindowBuilder}, ContextBuilder}, index::PrimitiveType, texture::{MipmapsOption, Texture2d, UncompressedUintFormat, UnsignedTexture2d}, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction, UniformsStorage}, Blend, BlendingFunction, Display, DrawParameters, IndexBuffer, LinearBlendingFactor, Surface, VertexBuffer};

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


const PROJECTION_OFFSET: f32 = 0.5;


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
	let mut aspect_ratio = window_height as f32 / window_width as f32;
	
	let mut tile_size = 1.0/40.0;
	
	let mut screen_texture = Texture2d::empty(&display, window_width, window_height).unwrap();
	let mut data_texture = Texture2d::empty(&display, window_width, window_height).unwrap();
	
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
	
	let mut _key_shift = false;
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
					aspect_ratio = physical_size.height as f32 / physical_size.width as f32;
					screen_texture = Texture2d::empty(&display, physical_size.width, physical_size.height).unwrap();
					data_texture = Texture2d::empty(&display, physical_size.width, physical_size.height).unwrap();
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
						VirtualKeyCode::LShift | VirtualKeyCode::RShift => _key_shift = state.is_pressed(),
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
								world.entities[0].position = Vec3(0.5, 0.5, 10.0);
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
					world.load(pos);
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
				
				
				
				let screen_width_in_tiles = 1.0 / tile_size;
				
				let render_size = Vec2(1.0, aspect_ratio) * screen_width_in_tiles + Vec2(0.0, 2.0 * PROJECTION_OFFSET);
				let render_position = world.entities[0].position.xy().as_type::<f32>() - Vec2(0.0, world.entities[0].position.z() as f32 * PROJECTION_OFFSET) - render_size * 0.5 - Vec2(0.0, 1.0 * PROJECTION_OFFSET);
				
				let x_size = render_size.x().ceil() as usize + 1;
				let y_size = render_size.y().ceil() as usize + 1;
				let mut tile_data_buffer = vec![vec![(0, 0); x_size]; y_size];
				
				
				let mut target = MultiOutputFrameBuffer::new(&display, [
					("color", &screen_texture),
					("data", &data_texture),
				]).unwrap();
				target.clear_color(0.0, 0.0, 0.0, 0.0);
				
				
				for z in 0..CELL_HEIGHT {
					let render_position = render_position + Vec2(0.0, PROJECTION_OFFSET * (z + 1) as f32);
					
					let x_start = render_position.x().floor() as isize;
					let y_start = render_position.y().floor() as isize;
					let x_end = x_start + x_size as isize;
					let y_end = y_start + y_size as isize;
					
					for cell_y in (y_start >> CELL_WIDTH_BITS)..=((y_end + 1) >> CELL_WIDTH_BITS) {
						for cell_x in (x_start >> CELL_WIDTH_BITS)..=((x_end + 1) >> CELL_WIDTH_BITS) {
							let cell_pos = Vec3(cell_x, cell_y, 0);
							
							if let Some(cell) = world.cells.get(&cell_pos) {
								let cell_start = cell_pos << CELL_SIZE_BITS;
								let cell_end = cell_pos + Vec3(1, 1, 0) << CELL_SIZE_BITS;
								let x_start_cell = isize::max(x_start, cell_start.x());
								let y_start_cell = isize::max(y_start, cell_start.y());
								let x_end_cell = isize::min(x_end, cell_end.x());
								let y_end_cell = isize::min(y_end, cell_end.y());
								
								for y in y_start_cell..y_end_cell {
									for x in x_start_cell..x_end_cell {
										tile_data_buffer[(y - y_start) as usize][(x - x_start) as usize] = cell.tiles[z][(y & CELL_XY_MASK) as usize][(x & CELL_XY_MASK) as usize].get_uv();
									}
								}
							}
						}
					}
					
					
					
					let tile_data_texture = UnsignedTexture2d::with_format(&display, tile_data_buffer.clone(), UncompressedUintFormat::U16U16, MipmapsOption::NoMipmap).unwrap();
					
					target.draw(&rect_vertex_buffer, &rect_index_buffer, &tilemap_program, &UniformsStorage::
							new("aspect_ratio", aspect_ratio)
						.add("screen_width_in_tiles", screen_width_in_tiles)
						.add("offset", render_position.modulo(1.0) + Vec2(0.0, 1.0 * PROJECTION_OFFSET))
						.add("z", (z + 1) as f32 - world.entities[0].position.z() as f32)
						.add("tile_data_texture", &tile_data_texture)
						.add("tilemap_texture", Sampler(&tilemap_texture, SamplerBehavior {
							wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
							minify_filter: MinifySamplerFilter::Linear,
							magnify_filter: MagnifySamplerFilter::Nearest,
							depth_texture_comparison: None,
							max_anisotropy: 1,
						}))
					, &DrawParameters {
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
						.. Default::default()
					}).unwrap();
				}
				
				
				let render_size_inverse = Vec2(1.0 / render_size.x(), 1.0 / render_size.y());
				
				world.entities.iter().rev().for_each(|entity| {
					target.draw(&rect_vertex_buffer, &rect_index_buffer, &world_texture_program, &UniformsStorage::
							new("texture_position", entity.position.xy().as_type::<f32>() + Vec2(0.0, entity.position.z() as f32 * -PROJECTION_OFFSET) - entity.size.xy().as_type::<f32>() * 0.5)
						.add("texture_size", entity.size.xy().as_type::<f32>())
						.add("render_position", render_position)
						.add("render_size_inverse", render_size_inverse)
						.add("z", (entity.position.z() - world.entities[0].position.z()) as f32 + 0.1)
						.add("tex", Sampler(entity.current_sprite(), SamplerBehavior {
							wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
							minify_filter: MinifySamplerFilter::Linear,
							magnify_filter: MagnifySamplerFilter::Nearest,
							depth_texture_comparison: None,
							max_anisotropy: 1,
						}))
						.add("data_texture", Sampler(&data_texture, SamplerBehavior {
							wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
							minify_filter: MinifySamplerFilter::Nearest,
							magnify_filter: MagnifySamplerFilter::Nearest,
							depth_texture_comparison: None,
							max_anisotropy: 1
						}))
					, &DrawParameters {
						blend: Blend::alpha_blending(),
						..Default::default()
					}).unwrap();
				});
				
				
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
				, &DrawParameters::default()).unwrap();
				
				display_target.finish().unwrap();
				
				
				
			}
			_ => ()
		}
	});
}
