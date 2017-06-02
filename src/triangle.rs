use std::mem::swap;
use super::sdl2;
use sdl2::render;

use pointf::Point2f;
use pointf::Point3f;
use color::Color;

#[derive(Copy, Clone)]
pub struct Triangle3D {
	v0: Point3f,
	v1: Point3f,
	v2: Point3f,

	color: Color,
}

impl Triangle3D {
	pub fn new(v0: Point3f, v1: Point3f, v2: Point3f) -> Triangle3D {
		Triangle3D {
			v0: v0,
			v1: v1,
			v2: v2,
			color: Color::new(255,255,255),
		}
	}
	
	pub fn set_color(&mut self, color: Color) {
		self.color = color;
	}

	pub fn to_screen_space(&self, cam_pos: Point3f, cam_rot: Point3f) -> (Option<Triangle2D>, Option<Triangle2D>) {
		let mut v0 = self.v0.to_camera_view(cam_pos, cam_rot);//.perspective_project();
		let mut v1 = self.v1.to_camera_view(cam_pos, cam_rot);//.perspective_project();
		let mut v2 = self.v2.to_camera_view(cam_pos, cam_rot);//.perspective_project();

		//Sort vertices from z- to z+ (v0 < v1 < v2)
		if v1.z() < v0.z() {
			swap(&mut v1, &mut v0);
		}
		if v2.z() < v1.z() {
			swap(&mut v2, &mut v1);
		}

		if v2.z() > 0.0 {

			//If all points are behind camera, don't even try
			if v0.z() > 0.0 {
				return (None, None);
			}

			//First case, only v2 is behind camera (e.g. v1.x < 0)
			if v1.z() <= 0.0 {
				let mut plane_1 = Point3f::intersect_plane(v0,v2, Point3f::new(0.0,0.0,-1.0));
				let mut plane_2 = Point3f::intersect_plane(v1,v2, Point3f::new(0.0,0.0,-1.0));

				if plane_1 == None || plane_2 == None {
					return (None, None);
				}

				let mut plane_1 = plane_1.unwrap();
				let mut plane_2 = plane_2.unwrap();

				//Sort so plane_1 is the highest point
				if plane_1.y() < plane_2.y() { swap(&mut plane_1, &mut plane_2); }

				//Sort v0 & v1, so v1 is higher
				if v1.y() < v0.y() {
					swap(&mut v0, &mut v1);
				}

				v0 = v0.perspective_project();
				v1 = v1.perspective_project();
				plane_1 = plane_1.perspective_project();
				plane_2 = plane_2.perspective_project();

				//Create first triangle with (v0,v1,plane_1)
				let tri1 = Triangle2D::new(v0.xy(), v1.xy(), plane_1.xy());

				//Create second triangle with (v0, plane_1, plane_2)
				let tri2 = Triangle2D::new(v0.xy(), plane_1.xy(), plane_2.xy());

				return (Some(tri1), Some(tri2));
			} 
			//Second case, both v2 & v1 are behind the camera
			else if v1.z() > 0.0 {
				let mut plane_1 = Point3f::intersect_plane(v0,v1, Point3f::new(0.0,0.0,-1.0));
				let mut plane_2 = Point3f::intersect_plane(v0,v2, Point3f::new(0.0,0.0,-1.0));

				if plane_1 == None || plane_2 == None {
					return (None, None);
				}

				v1 = plane_1.unwrap();
				v2 = plane_2.unwrap();
				
				v0 = v0.perspective_project();
				v1 = v1.perspective_project();
				v2 = v2.perspective_project();

				return (Some(Triangle2D::new(v0.xy(), v1.xy(), v2.xy())), None);
			}
		}
				
		v0 = v0.perspective_project();
		v1 = v1.perspective_project();
		v2 = v2.perspective_project();

		(Some(Triangle2D::new(v0.xy(), v1.xy(), v2.xy())), None)
	}

	pub fn v0(&self) -> Point3f {
		self.v0
	}

	pub fn v1(&self) -> Point3f {
		self.v1
	}

	pub fn v2(&self) -> Point3f {
		self.v2
	}
	
	pub fn color(&self) -> Color {
		self.color
	}

}

#[derive(Copy, Clone, PartialEq)]
pub struct Triangle2D {
	v0: Point2f,
	v1: Point2f,
	v2: Point2f,
}

impl Triangle2D {
	pub fn new(v0: Point2f, v1: Point2f, v2: Point2f) -> Triangle2D {
		Triangle2D {
			v0: v0,
			v1: v1,
			v2: v2,
		}
	}

	pub fn standard() -> Triangle2D {
		Triangle2D {
			v0: Point2f::new(0.0,0.0),
			v2: Point2f::new(0.0,1.0),
			v1: Point2f::new(1.0,1.0),
		}
	}

	pub fn copy(tri: &Triangle2D) -> Triangle2D {
		Triangle2D {
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

	pub fn sort(&self) -> Triangle2D{
		let mut tri = Triangle2D::copy(self);

		if tri.v1.y() < tri.v0.y() {
			swap(&mut tri.v0, &mut tri.v1);
		}
		if tri.v2.y() < tri.v0.y() {
			swap(&mut tri.v0, &mut tri.v2);
		}
		if tri.v2.x() < tri.v1.x() {
			//swap(&mut tri.v1, &mut tri.v2);
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

//Vector Cross Product
fn vxs(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
	(x1*y2) - (x2*y1)
}

fn intersect(v1: Point3f, v2: Point3f, v3: Point3f, v4: Point3f) -> Point3f {
	let mut v = Point3f::new(0.0,0.0,0.0);

	v.set_x(vxs(vxs(v1.x(), v1.z(), v2.x(), v2.z()), (v1.x()) - (v2.x()), vxs(v3.x(), v3.z(), v4.x(), v4.z()), (v3.x()) - (v4.x())) / vxs((v1.x()) - (v2.x()), (v1.z()) - (v2.z()), (v3.x()) - (v4.x()), (v3.z()) - (v4.z())));
	v.set_z(vxs(vxs(v1.x(), v1.z(), v2.x(), v2.z()), (v1.z()) - (v2.z()), vxs(v3.x(), v3.z(), v4.x(), v4.z()), (v3.z()) - (v4.z())) / vxs((v1.x()) - (v2.x()), (v1.z()) - (v2.z()), (v3.x()) - (v4.x()), (v3.z()) - (v4.z())));
	
	v
}

pub fn fix_triangle(tri: &Triangle2D) -> (Triangle2D, Triangle2D) {
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

	let mut tri1 = Triangle2D::standard();
	tri1.set_vert(0, tri.v0().x(), tri.v0().y());
	tri1.set_vert(1, intersect.x(), intersect.y());
	tri1.set_vert(2, newxy.x(), newxy.y());

	let mut tri2 = Triangle2D::standard();
	tri2.set_vert(0, tri.v1().x(), tri.v1().y());
	tri2.set_vert(1, newxy.x(), newxy.y());
	tri2.set_vert(2, tri.v2().x(), tri.v2().y());

	(tri1,tri2)
}

pub fn draw_triangle_wireframe(tri: Triangle2D, xoffset: i32, yoffset: i32, renderer: &mut render::Renderer) {
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
		if x < 0.0 || x > 1280.0 || y < 0.0 || y > 720.0 {
			continue;
		}
		renderer.draw_rect(sdl2::rect::Rect::new(x as i32 - 4 + xoffset,y as i32 - 4 + yoffset, 8, 8));
	}
}

pub fn draw_triangle_solid(tri: Triangle2D, xoffset: i32, yoffset: i32, color: Color, texture: &mut render::Texture) {
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
		swap(&mut slope_right, &mut slope_left);
		swap(&mut top_left, &mut top_right);
		//top_left = sort_tri.v1().x();
		//top_right = sort_tri.v0().x();
	}

	let mut begin: i32 = sort_tri.v0().y() as i32;
	let mut end: i32 = if sort_tri.v1().y() > sort_tri.v2().y() { sort_tri.v1().y() as i32 } else { sort_tri.v2().y() as i32 };

	if begin < 0 { begin = 0; }
	if end >= 1080 { end = 1080-1; }

	texture.with_lock(None, |pixels: &mut [u8], pitch: usize| {
		for y in begin..end {

			if y%16 != 0 { continue; }

			let mut begin_x: i32 = (((y-begin) as f32)*slope_left + top_left) as i32;
			let mut end_x: i32 = (((y-begin) as f32)*slope_right + top_right) as i32;

			if end_x < begin_x {
				swap(&mut begin_x, &mut end_x);
			}

			if begin_x >= 1920 || end_x < 0 {
				continue;
			}

			if begin_x < 0 { begin_x = 0; }
			if end_x >= 1920 { end_x = 1920-1; }

			for x in begin_x..end_x {
				let offset = (x*3) as usize + (y as usize) * pitch;
				pixels[offset + 0] = color.r();
				pixels[offset + 1] = color.g();
				pixels[offset + 2] = color.b();
			}

			//renderer.set_draw_color(sdl2::pixels::Color::RGB(0xc9,0x99,0x30));
			//renderer.draw_line(sdl2::rect::Point::new(begin_x + xoffset, y*8 + begin + yoffset), sdl2::rect::Point::new(end_x + xoffset, y*8 + begin + yoffset));
		}
	}).unwrap();

	//renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
	//renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
}

pub fn draw_triangle_shaded(tri: Triangle2D, xoffset: i32, yoffset: i32, color: Color, renderer: &mut render::Renderer) {
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
		swap(&mut slope_right, &mut slope_left);
		swap(&mut top_left, &mut top_right);
	}

	let mut begin: i32 = sort_tri.v0().y() as i32;
	let mut end: i32 = if sort_tri.v1().y() > sort_tri.v2().y() { sort_tri.v1().y() as i32 } else { sort_tri.v2().y() as i32 };

	if begin >= end {
		return;
	}

	if begin < -1080 { begin = -1080; }
	if end >= 1080 { end = 1080-1; }

	for y in 0..(end-begin) {
		let mut begin_x: i32 = (((y) as f32)*slope_left + top_left) as i32;
		let mut end_x: i32 = (((y) as f32)*slope_right + top_right) as i32;

		if end_x < begin_x {
			swap(&mut begin_x, &mut end_x);
		}

		if begin_x >= 1920 || end_x < 0 {
			continue;
		}

		if begin_x < 0 { begin_x = 0; }
		if end_x >= 1920 { end_x = 1920-1; }

		renderer.set_draw_color(sdl2::pixels::Color::RGB(color.r(),color.g(),color.b()));
		renderer.draw_line(sdl2::rect::Point::new(begin_x + xoffset, y + begin + yoffset), sdl2::rect::Point::new(end_x + xoffset, y + begin + yoffset));

	}

	//renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
	//renderer.draw_line(sdl2::rect::Point::new(x as i32 + xoffset, y as i32 + yoffset), sdl2::rect::Point::new(x1 as i32 + xoffset, y1 as i32 + yoffset));
}