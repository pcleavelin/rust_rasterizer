#[macro_use] extern crate unborrow;
#[macro_use] extern crate text_io;

extern crate sdl2;
use sdl2::*;
use sdl2::rect::Point;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::io::prelude;

mod pointf;
use pointf::*;

mod triangle;
use triangle::*;

mod color;
use color::Color;

fn create_sdl<'a>() -> (Sdl, VideoSubsystem, render::Renderer<'a>, EventPump) {
	let sdl_context: Sdl;
	let sdl_video: VideoSubsystem;
	let event_pump: sdl2::EventPump;

	match sdl2::init() {
		Ok(context) => {
			sdl_context = context;
			println!("SDL2 initialized!");
		},
		Err(msg) => panic!("Could not initialize sdl2: {}", msg),
	}

	match sdl_context.video() {
		Ok(subsys) => {
			println!("{:?}", subsys);
			sdl_video = subsys;
		}
		Err(msg) => panic!("{}", msg),
	}
	let mut win_builder = sdl_video.window("Triangle2D Partitioning", 1920, 1080);
	//win_builder.set_window_flags(0x4 as u32);

	let window: sdl2::video::Window;
	match win_builder.build() {
		Ok(win) => {
			println!("Created window!");
			window = win;
		},
		Err(err) => panic!("{:?}", err)
	}

	let mut render_builder = window.renderer();
	render_builder = render_builder.accelerated();
	render_builder = render_builder.present_vsync();
	let mut renderer = match render_builder.build() {
		Ok(rend) => rend,
		Err(_) => panic!("Couldn't get renderer!"),
	};

	match sdl_context.event_pump() {
		Ok(evtp) => event_pump = evtp,
		Err(msg) => panic!("{:?}", msg),
	}

	(sdl_context, sdl_video, renderer, event_pump)
}


fn main() {
	let path = Path::new("C:/Users/patri/OneDrive/Documents/Projects/mooD Engine in C/NMF-ED/bin/Debug/Maps/spaceman.nmf");
	let mut map_file = match File::open(&path) {
		Ok(file) => file,
		Err(why) => panic!("failed to open {}, {}", path.display(), why.description()),
	};
	let mut buf_reader = BufReader::new(map_file);
	let mut contents = String::new();

	let (mut sdl_context, mut sdl_video, mut renderer, mut event_pump) = create_sdl();

	let mut triangles: Vec<Triangle3D> = Vec::new();
	let (mut x1, mut z1, mut x2, mut z2) = (0f32,0f32,0f32,0f32);
	let mut wall = false;
	let (mut y1, mut y2) = (1.0, -1.0);

	for line in buf_reader.lines() {
		let line = line.unwrap();

		if line.contains("vertex") {
			if !wall {
				wall = true;
				scan!(line.bytes() => "vertex {}, {}", x1, z1);
			} else {
				scan!(line.bytes() => "vertex {}, {}", x2, z2);

				//1st wall Triangle2D
				let v0 = Point3f::new(x1, y2, z1);
				let v1 = Point3f::new(x1, y1, z1);
				let v2 = Point3f::new(x2, y2, z2);
				//2nd wall Triangle2D
				let v3 = Point3f::new(x1, y1, z1);
				let v4 = Point3f::new(x2, y2, z2);
				let v5 = Point3f::new(x2, y1, z2);

				let mut index = triangles.len();
				triangles.push(Triangle3D::new(v0, v1, v2));
				triangles[index].set_color(Color::new(70,120,80));

				triangles.push(Triangle3D::new(v3, v4, v5));
				//triangles[index+1].set_color(Color::new(120,80,70));
				triangles[index+1].set_color(Color::new(70,120,80));

				x1 = x2;
				z1 = z2;
			}
		}

		if line.contains("sector :") {
			wall = false;

			scan!(line.bytes() => "sector : {}, {}", y1, y2);
		}
	}	

	let mut counter: f32 = 0.0;
	let mut v0 = Point3f::new( 0.0,  -0.50+(counter.sin()/2.0), 1.0);
	let mut v1 = Point3f::new( 0.5,  -0.50+(counter.sin()/2.0), 1.0);
	let mut v2 = Point3f::new( 0.75, -0.50+(counter.sin()/2.0), 5.0);

	let mut tri1 = Triangle2D::new(v0.xy(), v1.xy(), v2.xy());

	let mut current_vert = 0;
	let mut is_moving_vert = false;

	let mut cam_pos = Point3f::new(20.0,2.5,20.0);
	let mut cam_rot = Point3f::from((0,180,0));
	let (mut forward, mut backward, mut left, mut right) = (false, false, false, false);
	let speed = 0.5;

	let mut running = true;
    
	let mut texture = renderer.create_texture_streaming(pixels::PixelFormatEnum::RGB24, 1920, 1080).unwrap();

	while running {
		for event in event_pump.poll_iter() {
			use sdl2::event::Event;
			match event {
				Event::Quit {..} => {
					running = false;
				}
				
				Event::MouseMotion {x, y, mousestate, xrel, yrel, ..} => {
					if mousestate.left() {
						//tri1.set_vert(current_vert, x as f32, y as f32);
						cam_rot = cam_rot.offset(0.0,-xrel as f32 * 0.2,0.0);
						if cam_rot.y() > 360.0 {
							cam_rot = cam_rot.offset(0.0,-360.0,0.0);
						}
						if cam_rot.y() < 0.0 {
							cam_rot = cam_rot.offset(0.0,360.0,0.0);
						}
					}
				}
				
				Event::MouseButtonDown {mouse_btn, ..} => {
					match mouse_btn {
						sdl2::mouse::MouseButton::Right => {
							current_vert += 1;
							if current_vert > 2 {
								current_vert = 0;
							}
						}
						_ => {}
					}
				}
				
				Event::KeyDown {keycode, ..} => {
					if keycode != None {
						match keycode.unwrap() {
							keyboard::Keycode::W => {
								forward = true;
							}
							keyboard::Keycode::S => {
								backward = true;
							}
							keyboard::Keycode::A => {
								left = true;
							}
							keyboard::Keycode::D => {
								right = true;
							}

							_ => {}
						}
					}
				}
				
				Event::KeyUp {keycode, ..} => {
					if keycode != None {
						match keycode.unwrap() {
							keyboard::Keycode::W => {
								forward = false;
							}
							keyboard::Keycode::S => {
								backward = false;
							}
							keyboard::Keycode::A => {
								left = false;
							}
							keyboard::Keycode::D => {
								right = false;
							}

							_ => {}
						}
					}
				}

				_ => {}
			}
		}

		if forward {
			cam_pos = cam_pos.offset(-speed * cam_rot.y().to_radians().sin(),0.0, -speed * cam_rot.y().to_radians().cos());
		}
		if backward {
			cam_pos = cam_pos.offset(speed * cam_rot.y().to_radians().sin(),0.0, speed * cam_rot.y().to_radians().cos());
		}
		if left {
			cam_pos = cam_pos.offset(speed * (-90.0 + cam_rot.y()).to_radians().sin(),0.0, speed * (-90.0 + cam_rot.y()).to_radians().cos());
		}
		if right {
			cam_pos = cam_pos.offset(-speed * (-90.0 + cam_rot.y()).to_radians().sin(),0.0, -speed * (-90.0 + cam_rot.y()).to_radians().cos());
		}

		renderer.set_draw_color(sdl2::pixels::Color::RGB(0x00,0x00,0x00));
		renderer.clear();
		renderer.copy(&texture, None, None).unwrap();

		/*texture.with_lock(None, |pixels: &mut [u8], pitch: usize| {
			for y in 0..1024 {
				for x in 0..1024 {
					let offset = (x*3) as usize + (y as usize) * pitch;

					pixels[offset + 0] = 0;
					pixels[offset + 1] = 0;
					pixels[offset + 2] = 0;
				}
			}
		}).unwrap();*/

		counter += 0.1;
		{
			v0 = Point3f::new( 0.0, -2.5+3.0, 1.0);
			v1 = Point3f::new( 0.5, 2.5+3.0, 1.0);
			v2 = Point3f::new( 0.75, -2.5+3.0, 5.0);

			let old_v2 = v2.to_camera_view(cam_pos, cam_rot).perspective_project();

			v0 = v0.to_camera_view(cam_pos, cam_rot).perspective_project();
			v1 = v1.to_camera_view(cam_pos, cam_rot).perspective_project();
			v2 = v2.to_camera_view(cam_pos, cam_rot).perspective_project();

			//v2 = v2.perspective_project();

			tri1 = Triangle2D::new(v0.xy(), v1.xy(), v2.xy());
			let tri2 = Triangle2D::new(v0.xy(), v1.xy(), old_v2.xy());

			let (fix1, fix2) = fix_triangle(&mut tri1);
			//draw_triangle_wireframe(tri1, 0, 0, &mut renderer);
			//draw_triangle_wireframe(tri2, 0, 0, &mut renderer);
			//draw_triangle_wireframe(fix1, 0, 0, &mut renderer);
			//draw_triangle_wireframe(fix2, 16, 0, &mut renderer);

			//draw_triangle_solid(fix1, 0, 0, &mut texture);
			//draw_triangle_solid(fix2, 0, 0, &mut texture);

			for i in 0..triangles.len() {
				let (screen_tri_1, screen_tri_2) = triangles[i].to_screen_space(cam_pos, cam_rot);

				if screen_tri_1 != None {
					let screen_tri_1 = screen_tri_1.unwrap();
					let (fix1, fix2) = fix_triangle(&screen_tri_1);
					draw_triangle_shaded(fix1, 0, 0, triangles[i].color(), &mut renderer);
					draw_triangle_shaded(fix2, 0, 0, triangles[i].color(), &mut renderer);

					//draw_triangle_solid(fix1, 0, 0, triangles[i].color(), &mut texture);
					//draw_triangle_solid(fix2, 0, 0, triangles[i].color(), &mut texture);
					draw_triangle_wireframe(screen_tri_1, 0, 0, &mut renderer);
					//draw_triangle_wireframe(fix1, 0, 0, &mut renderer);
					//draw_triangle_wireframe(fix2, 0, 0, &mut renderer);
				}
				if screen_tri_2 != None {
					let screen_tri_2 = screen_tri_1.unwrap();
					let (fix1, fix2) = fix_triangle(&screen_tri_2);
					//draw_triangle_shaded(fix1, 0, 0, triangles[i].color(), &mut renderer);
					//draw_triangle_shaded(fix2, 0, 0, triangles[i].color(), &mut renderer);

					//draw_triangle_solid(fix1, 0, 0, triangles[i].color(), &mut texture);
					//draw_triangle_solid(fix2, 0, 0, triangles[i].color(), &mut texture);
					draw_triangle_wireframe(screen_tri_2, 0, 0, &mut renderer);
					//draw_triangle_wireframe(fix1, 0, 0, &mut renderer);
					//draw_triangle_wireframe(fix2, 0, 0, &mut renderer);
				}
			}
		}
		
		//renderer.set_draw_color(sdl2::pixels::Color::RGB(0xc6,0x99,0x39));
		//let (x,y) = tri1.get_vert(current_vert);
		//renderer.draw_rect(sdl2::rect::Rect::new(x as i32 - 8i32,y as i32 - 8i32, 16, 16));

		renderer.present();
	}
}
