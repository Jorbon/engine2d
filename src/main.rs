use winit::{dpi::PhysicalSize, event::{Event, KeyEvent, MouseButton, WindowEvent}, keyboard::{KeyCode, PhysicalKey}, window::{Icon, WindowBuilder}};
use glium::{framebuffer::SimpleFrameBuffer, index::PrimitiveType, texture::{MipmapsOption, Texture2d, UncompressedUintFormat, UnsignedTexture2d}, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction, UniformsStorage}, Blend, BlendingFunction, DrawParameters, IndexBuffer, LinearBlendingFactor, Surface, VertexBuffer};

#[allow(dead_code)] mod vec;
#[allow(dead_code)] mod entity;
#[allow(dead_code)] mod graphics;
#[allow(dead_code)] mod tiles;

use vec::*;
use entity::*;
use graphics::*;
use tiles::*;






fn main() {
	let event_loop = winit::event_loop::EventLoop::new().unwrap();
	let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().set_window_builder(WindowBuilder::new()
		.with_title("balls")
		// .with_fullscreen(Some(Fullscreen::Borderless(None)))
		.with_maximized(true)
		// .with_inner_size(LogicalSize { width: 300, height: 300 })
		.with_transparent(true)
		.with_decorations(false)
		.with_resizable(false)
		// .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
		.with_window_icon(Some(Icon::from_rgba(image::load_from_memory_with_format(include_bytes!("../assets/icon.png"), image::ImageFormat::Png).unwrap().resize(64, 64, image::imageops::FilterType::Nearest).into_bytes().to_vec(), 64, 64).unwrap()))
	).build(&event_loop);
	
	
	let mut tilemap_program = load_shader_program(&display, "tilemap", "tilemap");
	let _screen_texture_program = load_shader_program(&display, "screen_rectangle", "rectangle");
	let world_texture_program = load_shader_program(&display, "world_rectangle", "rectangle");
	let mut post_program = load_shader_program(&display, "default", "post_process");
	
	let rect_vertex_buffer = VertexBuffer::new(&display, &[Vec2(0.0f32, 0.0), Vec2(1.0, 0.0), Vec2(1.0, 1.0), Vec2(0.0, 1.0)]).unwrap();
	let rect_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0, 1, 2, 0, 2, 3u8]).unwrap();
	
	
	let PhysicalSize { width: window_width, height: window_height } = window.inner_size();
	let mut aspect_ratio = window_height as f32 / window_width as f32;
	
	let mut camera_position = Vec3(0.0, 0.0, 0.0);
	let mut tile_size = 1.0/40.0;
	
	let mut screen_texture = Texture2d::empty(&display, window_width, window_height).unwrap();
	
	let tilemap_texture = load_texture(&display, "tilemap");
	
	let mut cells: Vec<Cell> = vec![];
	let mut entities = vec![];
	
	entities.push(Entity::new(
		Vec3(0.5, 0.5, 0.0),
		Vec3(0.75, 0.75, 0.75),
		SpriteSet::load(&display, "player")
	));
	
	
	let mut key_w = false;
	let mut key_a = false;
	let mut key_s = false;
	let mut key_d = false;
	
	let mut _key_shift = false;
	let mut key_ctrl = false;
	
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
					aspect_ratio = physical_size.height as f32 / physical_size.width as f32;
					screen_texture = Texture2d::empty(&display, physical_size.width, physical_size.height).unwrap();
				}
				WindowEvent::CursorMoved { position: _, device_id: _ } => {
					
				}
				WindowEvent::MouseInput { state, button, device_id: _ } => match button {
					MouseButton::Left => if state.is_pressed() {
						
					},
					_ => ()
				}
				WindowEvent::MouseWheel { device_id: _, delta: _, phase: _ } => {
					entities[0].jump();
				}
				WindowEvent::KeyboardInput { event: KeyEvent { physical_key: PhysicalKey::Code(code), state, repeat, .. }, device_id: _, is_synthetic: _ } => {
					match code {
						KeyCode::KeyW => key_w = state.is_pressed(),
						KeyCode::KeyS => key_s = state.is_pressed(),
						KeyCode::KeyA => key_a = state.is_pressed(),
						KeyCode::KeyD => key_d = state.is_pressed(),
						KeyCode::ShiftLeft | KeyCode::ShiftRight => _key_shift = state.is_pressed(),
						KeyCode::ControlLeft | KeyCode::ControlRight => key_ctrl = state.is_pressed(),
						KeyCode::Space => if state.is_pressed() && !repeat {
							entities[0].jump();
						}
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
						KeyCode::KeyR => if state.is_pressed() && key_ctrl {
							tilemap_program = load_shader_program(&display, "tilemap", "tilemap");
							post_program = load_shader_program(&display, "default", "post_process");
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
					
					let mut dp = Vec3(0.0f32, 0.0, 0.0);
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
					
					
					
					let cell_position = entities[0].position.xy() / CELL_WIDTH as f32;
					let cell_corner = Vec2(cell_position.x().round() as isize, cell_position.y().round() as isize);
					let mut nearby_cells = [
						(cell_corner + Vec2(-1, -1), false),
						(cell_corner + Vec2( 0, -1), false),
						(cell_corner + Vec2(-1,  0), false),
						(cell_corner + Vec2( 0,  0), false),
					];
					
					for i in (0..cells.len()).rev() {
						let mut nearby = false;
						for nc in &mut nearby_cells {
							if nc.1 { continue }
							if nc.0 == cells[i].location {
								nc.1 = true;
								nearby = true;
								break
							}
						}
						if !nearby {
							cells.swap_remove(i);
						}
					}
					
					for nc in nearby_cells {
						if nc.1 { continue }
						cells.push(Cell::new(nc.0));
					}
					
					
					
					camera_position = entities[0].position;
					
					let screen_width_in_tiles = 1.0 / tile_size;
					
					let render_size = Vec2(1.0, aspect_ratio) * screen_width_in_tiles;
					let render_position = camera_position.xy() - render_size * 0.5;
					
					let x_size = render_size.x().ceil() as usize + 1;
					let y_size = render_size.y().ceil() as usize + 1;
					let mut tile_data_buffer = vec![vec![(0, 0); x_size]; y_size];
					
					let mut target = SimpleFrameBuffer::new(&display, &screen_texture).unwrap();
					target.clear_color(0.0, 0.0, 0.0, 0.0);
					
					
					for z in 0..4 {
						let render_position = render_position + Vec2(0.0, 0.7 * z as f32);
						
						let x_start = render_position.x().floor() as isize;
						let y_start = render_position.y().floor() as isize;
						let x_end = x_start + x_size as isize;
						let y_end = y_start + y_size as isize;
						
						for cell in &cells {
							let cell_start = cell.location * CELL_WIDTH as isize;
							let cell_end = cell_start + Vec2::all(CELL_WIDTH as isize);
							let x_start_cell = isize::max(x_start, cell_start.x());
							let y_start_cell = isize::max(y_start, cell_start.y());
							let x_end_cell = isize::min(x_end, cell_end.x());
							let y_end_cell = isize::min(y_end, cell_end.y());
							
							if x_end_cell <= x_start_cell || y_end_cell <= y_start_cell { continue }
							
							for y in y_start_cell..y_end_cell {
								for x in x_start_cell..x_end_cell {
									tile_data_buffer[(y - y_start) as usize][(x - x_start) as usize] = cell.tiles[(y - cell_start.y()) as usize][(x - cell_start.x()) as usize][z].get_uv();
								}
							}
						}
						
						let tile_data_texture = UnsignedTexture2d::with_format(&display, tile_data_buffer.clone(), UncompressedUintFormat::U16U16, MipmapsOption::NoMipmap).unwrap();
						
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
							..Default::default()
						}).unwrap();
					}
					
					
					let render_size_inverse = Vec2(1.0 / render_size.x(), 1.0 / render_size.y());
					
					entities.iter().rev().for_each(|entity| {
						target.draw(&rect_vertex_buffer, &rect_index_buffer, &world_texture_program, &UniformsStorage::
							new("texture_position", entity.position.xy() + Vec2(0.0, entity.position.z() * -0.7) - entity.size.xy() * 0.5)
						   .add("texture_size", entity.size.xy())
						   .add("render_position", render_position)
						   .add("render_size_inverse", render_size_inverse)
						   .add("tex", Sampler(entity.current_sprite(), SamplerBehavior {
							   wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
							   minify_filter: MinifySamplerFilter::Linear,
							   magnify_filter: MagnifySamplerFilter::Nearest,
							   depth_texture_comparison: None,
							   max_anisotropy: 1,
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
					, &DrawParameters::default()).unwrap();
					
					display_target.finish().unwrap();
					
					
				}
				_ => ()
			}
			_ => ()
		}
	}).unwrap();
}
