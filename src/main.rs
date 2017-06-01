#[macro_use] extern crate unborrow;
#[macro_use] extern crate text_io;

extern crate sdl2;
use sdl2::*;
use sdl2::rect::Point;
use std::mem::swap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::io::prelude;

mod pointf;
use pointf::*;

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
	let mut win_builder = sdl_video.window("Triangle Partitioning", 1024, 1024);
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

#[derive(Copy, Clone)]
pub struct Triangle {
	v0: Point2f,
	v1: Point2f,
	v2: Point2f,
}

impl Triangle {
	pub fn new(v0: Point2f, v1: Point2f, v2: Point2f) -> Triangle {
		Triangle {
			v0: v0,
			v1: v1,
			v2: v2,
		}
	}

	pub fn standard() -> Triangle {
		Triangle {
			v0: Point2f::new(0.0,0.0),
			v2: Point2f::new(0.0,1.0),
			v1: Point2f::new(1.0,1.0),
		}
	}

	pub fn copy(tri: &Triangle) -> Triangle {
		Triangle {
			v0: tri.v0,
			v1: tri.v1,
			v2: tri.v2,
		}
	}

	pub fn v0(&self) -> Point2f {
        self.v0
    }

	pub fn v1(&self) -> Point2f {
        self.v1
    }

	pub fn v2(&self) -> Point2f {
        self.v2
    }

	pub fn sort(&self) -> Triangle{
		let mut tri = Triangle::copy(self);

		if tri.v1.y() < tri.v0.y() {
			std::mem::swap(&mut tri.v0, &mut tri.v1);
		}
		if tri.v2.y() < tri.v0.y() {
			std::mem::swap(&mut tri.v0, &mut tri.v2);
		}
		if tri.v2.x() < tri.v1.x() {
			//std::mem::swap(&mut tri.v1, &mut tri.v2);
		}

		tri
	}

	pub fn set_vert(&mut self, vert: usize, x: f32, y: f32) {
		match vert {
			0 => {
				self.v0 = Point2f::from((x,y));
			}

			1 => {
				self.v1 = Point2f::from((x,y));
			}

			2 => {
				self.v2 = Point2f::from((x,y));
			}
			
			_ => {
				println!("invalid vertex {}", vert);
			}
		}
	}

	pub fn get_vert(&self, vert: usize) -> (f32, f32) {
		match vert {
			0 => {
				(self.v0.x(), self.v0.y())
			}

			1 => {
				(self.v1.x(), self.v1.y())
			}

			2 => {
				(self.v2.x(), self.v2.y())
			}
			
			_ => {
				println!("invalid vertex {}", vert);
				(0.0,0.0)
			}
		}
	}
}

fn fix_triangle(tri: &Triangle) -> (Triangle, Triangle) {
	let tri = tri.sort();

	let mut intersect = Point2f::new(tri.v1().x(), tri.v1().y());
	let mut newxy = Point2f::new(tri.v1().x(), tri.v1().y());
	let mut slope = tri.v0().slope(&tri.v2);
	if tri.v1().y() > tri.v2().y() {
		newxy = Point2f::new(tri.v2().x(), tri.v2().y());
		intersect = newxy;
		slope = tri.v0().slope(&tri.v1);
	}

	//x = (y-b)/m
	{
		let y = newxy.y();
		newxy.set_x((y - tri.v0().y())*slope + tri.v0().x());
	}

	let mut tri1 = Triangle::standard();
	tri1.set_vert(0, tri.v0().x(), tri.v0().y());
	tri1.set_vert(1, intersect.x(), intersect.y());
	tri1.set_vert(2, newxy.x(), newxy.y());

	let mut tri2 = Triangle::standard();
	tri2.set_vert(0, tri.v1().x(), tri.v1().y());
	tri2.set_vert(1, newxy.x(), newxy.y());
	tri2.set_vert(2, tri.v2().x(), tri.v2().y());

	(tri1,tri2)
}

fn draw_triangle_wireframe(tri: Triangle, xoffset: i32, yoffset: i32, renderer: &mut render::Renderer) {
	let sort_tri = tri.sort();
	
	for i in 0..3 {
		renderer.set_draw_color(sdl2::pixels::Color::RGB(0xff,0xff,0xff));

		if i < 2 {
			let (x,y) = sort_tri.get_vert(i);
			let (x1,y1) = sort_tri.get_vert(i+1);
			renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
		} else {
			let (x,y) = sort_tri.get_vert(i);
			let (x1,y1) = sort_tri.get_vert(0);
			renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
		}

		renderer.set_draw_color(match i {
			0 => {
				sdl2::pixels::Color::RGB(0xff,0x0,0x0)
			}
			1 => {
				sdl2::pixels::Color::RGB(0x0,0xff,0x0)
			}
			2 => {
				sdl2::pixels::Color::RGB(0x0,0x0,0xff)
			}
			_ => {
				sdl2::pixels::Color::RGB(0xff,0xff,0xff)
			}
		});
		let (x,y) = sort_tri.get_vert(i);
		renderer.draw_rect(sdl2::rect::Rect::new(x as i32 - 4 + xoffset,y as i32 - 4 + yoffset, 8, 8));
	}
}

fn draw_triangle_solid(tri: Triangle, xoffset: i32, yoffset: i32, texture: &mut render::Texture) {
	let sort_tri = tri.sort();

	let mut slope_left = 0.0; //sort_tri.v0().slope(&sort_tri.v1());
	let mut slope_right = 0.0; //sort_tri.v0().slope(&sort_tri.v2());
	let mut top_left = sort_tri.v0().x();
	let mut top_right = sort_tri.v0().x();

	if sort_tri.v1().y() != sort_tri.v2().y() {
		if sort_tri.v0().y() == sort_tri.v1().y() {
			slope_left = sort_tri.v0().slope(&sort_tri.v2());
			slope_right = sort_tri.v1().slope(&sort_tri.v2());
			top_right = sort_tri.v1().x();
		} else {
			slope_left = sort_tri.v0().slope(&sort_tri.v1());
			slope_right = sort_tri.v2().slope(&sort_tri.v1());
			top_right = sort_tri.v2().x();
		}
	} else {
		slope_left = sort_tri.v0().slope(&sort_tri.v1());
		slope_right = sort_tri.v0().slope(&sort_tri.v2());
	}

	if top_left > top_right {
		std::mem::swap(&mut slope_right, &mut slope_left);
		std::mem::swap(&mut top_left, &mut top_right);
		//top_left = sort_tri.v1().x();
		//top_right = sort_tri.v0().x();
	}

	let mut begin: i32 = sort_tri.v0().y() as i32;
	let mut end: i32 = if sort_tri.v1().y() > sort_tri.v2().y() { sort_tri.v1().y() as i32 } else { sort_tri.v2().y() as i32 };

	if begin < 0 { begin = 0; }
	if end >= 1024 { end = 1024-1; }

	texture.with_lock(None, |pixels: &mut [u8], pitch: usize| {
		for y in begin..end {

			//if y%16 != 0 { continue; }

			let mut begin_x: i32 = (((y-begin) as f32)*slope_left + top_left) as i32;
			let mut end_x: i32 = (((y-begin) as f32)*slope_right + top_right) as i32;

			if end_x < begin_x {
				std::mem::swap(&mut begin_x, &mut end_x);
			}

			if begin_x >= 1024 || end_x < 0 {
				continue;
			}

			if begin_x < 0 { begin_x = 0; }
			if end_x >= 1024 { end_x = 1024-1; }

			for x in begin_x..end_x {
				let offset = (x*3) as usize + (y as usize) * pitch;
				pixels[offset + 0] = 0xc9;
				pixels[offset + 1] = 0x99;
				pixels[offset + 2] = 0x30;
			}

			//renderer.set_draw_color(sdl2::pixels::Color::RGB(0xc9,0x99,0x30));
			//renderer.draw_line(sdl2::rect::Point::new(begin_x + xoffset, y*8 + begin + yoffset), sdl2::rect::Point::new(end_x + xoffset, y*8 + begin + yoffset));
		}
	}).unwrap();

	//renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
	//renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
}

fn main() {
	let path = Path::new("Maps/funtime.nmf");
	let mut map_file = match File::open(&path) {
		Ok(file) => file,
		Err(why) => panic!("failed to open {}, {}", path.display(), why.description()),
	};
	let mut buf_reader = BufReader::new(map_file);
	let mut contents = String::new();

	let (mut sdl_context, mut sdl_video, mut renderer, mut event_pump) = create_sdl();

	let mut triangles: Vec<Triangle> = Vec::new();
	loop {
		contents.clear();
		buf_reader.read_line(&mut contents);

		if contents.contains("vertex") {
			println!("{}", contents);
			let x: f32;
			let z: f32;
			scan!(contents.bytes() => "vertex {}, {}", x, z);
		}

		if contents.contains("neighbor") {
			break;
		}
	}	

	let mut counter: f32 = 0.0;
	let mut v0 = Point3f::new( 0.0,  -0.50+(counter.sin()/2.0), 1.0);
	let mut v1 = Point3f::new( 0.5,  -0.50+(counter.sin()/2.0), 1.0);
	let mut v2 = Point3f::new( 0.75, -0.50+(counter.sin()/2.0), 5.0);

	let mut tri1 = Triangle::new(v0.xy(), v1.xy(), v2.xy());

	let mut current_vert = 0;
	let mut is_moving_vert = false;

	let mut running = true;
    
	let mut texture = renderer.create_texture_streaming(pixels::PixelFormatEnum::RGB24, 1024, 1024).unwrap();

	while running {
		for event in event_pump.poll_iter() {
			use sdl2::event::Event;
			match event {
				Event::Quit {..} => {
					running = false;
				}
				Event::MouseMotion {x, y, mousestate, ..} => {
					if mousestate.left() {
						tri1.set_vert(current_vert, x as f32, y as f32);
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
				_ => {}
			}
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
			v0 = Point3f::new( 0.0+(counter.sin()*2.0), -0.5, 1.0);
			v1 = Point3f::new( 0.5+(counter.sin()*2.0), 0.5, 1.0);
			v2 = Point3f::new( 0.75+(counter.sin()*2.0), -0.5, 5.0);

			v0 = v0.perspective_project();
			v1 = v1.perspective_project();
			v2 = v2.perspective_project();

			tri1 = Triangle::new(v0.xy(), v1.xy(), v2.xy());

			let (fix1, fix2) = fix_triangle(&mut tri1);
			draw_triangle_wireframe(tri1, 0, 0, &mut renderer);
			//draw_triangle_wireframe(fix1, 0, 0, &mut renderer);
			//draw_triangle_wireframe(fix2, 16, 0, &mut renderer);

			//draw_triangle_solid(fix1, 0, 0, &mut texture);
			//draw_triangle_solid(fix2, 0, 0, &mut texture);
		}
		
		renderer.set_draw_color(sdl2::pixels::Color::RGB(0xc6,0x99,0x39));
		let (x,y) = tri1.get_vert(current_vert);
		renderer.draw_rect(sdl2::rect::Rect::new(x as i32 - 8i32,y as i32 - 8i32, 16, 16));

		renderer.present();
	}
}
