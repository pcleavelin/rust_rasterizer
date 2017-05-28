extern crate sdl2;
use sdl2::*;
use sdl2::rect::Point;
use std::mem::swap;

mod pointf;
use pointf::Pointf;

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
	v0: Pointf,
	v1: Pointf,
	v2: Pointf,
}

impl Triangle {
	pub fn new() -> Triangle {
		Triangle {
			v0: Pointf::new(1024.0/2.0, 1024.0/5.0),
			v1: Pointf::new(1024.0/5.0, 1024.0-1024.0/5.0),
			v2: Pointf::new(1024.0-1024.0/5.0, 1024.0-1024.0/5.0)
		}
	}

	pub fn copy(tri: &Triangle) -> Triangle {
		Triangle {
			v0: tri.v0,
			v1: tri.v1,
			v2: tri.v2,
		}
	}

	pub fn v0(&self) -> Pointf {
        self.v0
    }

	pub fn v1(&self) -> Pointf {
        self.v1
    }

	pub fn v2(&self) -> Pointf {
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
				self.v0 = Pointf::from((x,y));
			}

			1 => {
				self.v1 = Pointf::from((x,y));
			}

			2 => {
				self.v2 = Pointf::from((x,y));
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

	let mut intersect = Pointf::new(tri.v1().x(), tri.v1().y());
	let mut newxy = Pointf::new(tri.v1().x(), tri.v1().y());
	let mut slope = tri.v0().slope(&tri.v2);
	if tri.v1().y() > tri.v2().y() {
		newxy = Pointf::new(tri.v2().x(), tri.v2().y());
		intersect = newxy;
		slope = tri.v0().slope(&tri.v1);
	}

	//x = (y-b)/m
	{
		let y = newxy.y();
		newxy.set_x((y - tri.v0().y())*slope + tri.v0().x());
	}

	let mut tri1 = Triangle::new();
	tri1.set_vert(0, tri.v0().x(), tri.v0().y());
	tri1.set_vert(1, intersect.x(), intersect.y());
	tri1.set_vert(2, newxy.x(), newxy.y());

	let mut tri2 = Triangle::new();
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

fn draw_triangle_solid(tri: Triangle, xoffset: i32, yoffset: i32, renderer: &mut render::Renderer) {
	let sort_tri = tri.sort();

	let mut slope_left = sort_tri.v0().slope(&sort_tri.v1());
	let mut slope_right = sort_tri.v0().slope(&sort_tri.v2());
	let mut top_left = sort_tri.v0().x();
	let mut top_right = sort_tri.v0().x();

	if sort_tri.v1().y() != sort_tri.v2().y() {

		if sort_tri.v0().y() == sort_tri.v1().y() {
			slope_left = sort_tri.v0().slope(&sort_tri.v2());
			slope_right = sort_tri.v1().slope(&sort_tri.v2());
			top_right = sort_tri.v1().x();
		} else {
			slope_right = sort_tri.v2().slope(&sort_tri.v1());
			top_right = sort_tri.v2().x();
		}
	}

	if top_left > top_right {
		std::mem::swap(&mut slope_right, &mut slope_left);
		std::mem::swap(&mut top_left, &mut top_right);
		//top_left = sort_tri.v1().x();
		//top_right = sort_tri.v0().x();
	}

	let begin: i32 = sort_tri.v0().y() as i32;
	let end: i32 = if sort_tri.v1().y() > sort_tri.v2().y() { sort_tri.v1().y() as i32 } else { sort_tri.v2().y() as i32 };

	for y in 0..(end-begin)/8 {
		
		let mut begin_x: i32 = (((y*8) as f32)*slope_left + top_left) as i32;
		let mut end_x: i32 = (((y*8) as f32)*slope_right + top_right) as i32;

		if end_x < begin_x {
			std::mem::swap(&mut begin_x, &mut end_x);
		}

		renderer.set_draw_color(sdl2::pixels::Color::RGB(0xc9,0x99,0x30));
		renderer.draw_line(sdl2::rect::Point::new(begin_x + xoffset, y*8 + begin + yoffset), sdl2::rect::Point::new(end_x + xoffset, y*8 + begin + yoffset));

	}

	//renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
	//renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
}

fn main() {
	let (mut sdl_context, mut sdl_video, mut renderer, mut event_pump) = create_sdl();

	let mut tri1 = Triangle::new();
	let mut current_vert = 0;
	let mut is_moving_vert = false;

	let mut running = true;

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

		{
			let (fix1, fix2) = fix_triangle(&mut tri1);
			//draw_triangle_wireframe(tri1, 0, 0, &mut renderer);
			//draw_triangle_wireframe(fix1, 0, 0, &mut renderer);
			draw_triangle_wireframe(fix2, 16, 0, &mut renderer);

			draw_triangle_solid(fix1, 16, 0, &mut renderer);
			draw_triangle_solid(fix2, 16, 0, &mut renderer);
		}
		
		renderer.set_draw_color(sdl2::pixels::Color::RGB(0xc6,0x99,0x39));
		let (x,y) = tri1.get_vert(current_vert);
		renderer.draw_rect(sdl2::rect::Rect::new(x as i32 - 8,y as i32 - 8, 16, 16));

		renderer.present();
	}
}
