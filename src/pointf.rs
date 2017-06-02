use std::ops::{Add, Neg, Div, Sub, Mul};
use std::mem::swap;

/*
A 2-dimensional vector
*/
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point2f {
	x: f32,
	y: f32
}

impl From<(f32, f32)> for Point2f {
	fn from((x, y): (f32, f32)) -> Point2f {
		Point2f::new(x, y)
	}
}

impl Into<(f32, f32)> for Point2f {
	fn into(self) -> (f32, f32) {
		(self.x, self.y)
	}
}

impl Point2f {
	/// Creates a new Point2f from the given coordinates.
	pub fn new(x: f32, y: f32) -> Point2f {
		Point2f {
			x: x,
			y: y,
		}
	}

	pub fn slope(&self, rhs: &Point2f) -> f32 {
		(rhs.x - self.x) / (rhs.y - self.y)
		//(rhs.y - self.y) / (rhs.x - self.x)
	}

	pub fn offset(&self, x: f32, y: f32) -> Point2f {
		Point2f::new(self.x + x, self.y + y)
	}

	pub fn set_x(&mut self, x: f32) {
		self.x = x;
	}

	pub fn set_y(&mut self, y: f32) {
		self.y = y;
	}

	pub fn x(&self) -> f32 {
		self.x
	}

	pub fn y(&self) -> f32 {
		self.y
	}
}

impl Add for Point2f {
	type Output = Point2f;

	fn add(self, rhs: Point2f) -> Point2f {
		self.offset(rhs.x, rhs.y)
	}
}

impl Neg for Point2f {
	type Output = Point2f;

	fn neg(self) -> Point2f {
		Point2f::new(-self.x, -self.y)
	}
}

impl Sub for Point2f {
	type Output = Point2f;

	fn sub(self, rhs: Point2f) -> Point2f {
		self.offset(-rhs.x, -rhs.y)
	}
}

impl Mul<f32> for Point2f {
	type Output = Point2f;

	fn mul(self, rhs: f32) -> Point2f {
		Point2f::new(self.x * rhs, self.y * rhs)
	}
}

impl Div<f32> for Point2f {
	type Output = Point2f;

	fn div(self, rhs: f32) -> Point2f {
		Point2f::new(self.x / rhs, self.y / rhs)
	}
}

/*
A 3-dimensional vector
*/
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point3f {
	x: f32,
	y: f32,
	z: f32,
}


impl From<(f32, f32, f32)> for Point3f {
	fn from((x, y, z): (f32, f32, f32)) -> Point3f {
		Point3f::new(x, y, z)
	}
}

impl From<(i32, i32, i32)> for Point3f {
	fn from((x, y, z): (i32, i32, i32)) -> Point3f {
		Point3f::new(x as f32, y as f32, z as f32)
	}
}

impl Into<(f32, f32, f32)> for Point3f {
	fn into(self) -> (f32, f32, f32) {
		(self.x, self.y, self.z)
	}
}
impl Point3f {
	/// Creates a new Point3f from the given coordinates.
	pub fn new(x: f32, y: f32, z: f32) -> Point3f {
		Point3f {
			x: x,
			y: y,
			z: z,
		}
	}

	pub fn offset(&self, x: f32, y: f32, z: f32) -> Point3f {
		Point3f::new(self.x + x, self.y + y, self.z + z)
	}

	pub fn to_camera_view(&self, cam_pos: Point3f, cam_rot: Point3f) -> Point3f{
		let translated = Point3f::new(self.x - cam_pos.x(), self.y - cam_pos.y(), self.z - cam_pos.z());

		let pcos = cam_rot.y().to_radians().cos();
		let psin = cam_rot.y().to_radians().sin();

		let mut cam_view = translated;
		cam_view.set_x(pcos * translated.x() - psin * translated.z());
		cam_view.set_z(pcos * translated.z() + psin * translated.x());

		cam_view
	}

	pub fn perspective_project(&self) -> Point3f {
		let mut e = Point3f::new((1920.0 / 2.0) * self.x / self.z,
								(1080.0 / 2.0) * self.y / self.z,
								1.0 / ((0.73f32 * 1080.0) / 2.0).tan());

		let mut b = Point3f::new(1920.0 / 2.0 + ((e.z() / self.z) * self.x - e.x()),
								1080.0 / 2.0 + ((e.z() / self.z) * self.y - e.y()),
								self.z);

		b
	}

	pub fn dot(&self, v2: Point3f) -> f32 {
		self.x*v2.x() + self.y*v2.y() + self.z*v2.z()
	}

	//Gets the intersection point between the vector and a plane
	pub fn intersect_plane(v1: Point3f, v2: Point3f, norm: Point3f) -> Option<Point3f> {
		let correct_v1: Point3f;
		let correct_v2: Point3f;

		/*if v2.z() < v1.z() {
			correct_v1 = v2;
			correct_v2 = v1;
		} else {
			correct_v1 = v2;
			correct_v2 = v1;
		}*/

		match (v1 - v2).normalize() {
			Some(direction) => {
				let d = direction.dot(norm);

				if d == 0.0 {
					return None;
				}

				let distance: f32 = -((v2.dot(norm)) / d);

				Some(v2 + direction * (distance+0.001))
			}
			_ => {
				None
			}
		}
	}

	pub fn normalize(&self) -> Option<Point3f> {
		let length = (self.x*self.x + self.y*self.y + self.z*self.z).sqrt();

		if length == 0.0 {
			return None;
		}

		Some(Point3f::new(self.x / length, self.y / length, self.z / length))
	}

	pub fn xy(&self) -> Point2f {
		Point2f::new(self.x, self.y())
	}

	pub fn set_x(&mut self, x: f32) {
		self.x = x;
	}

	pub fn set_y(&mut self, y: f32) {
		self.y = y;
	}

	pub fn set_z(&mut self, z: f32) {
		self.z = z;
	}

	pub fn x(&self) -> f32 {
		self.x
	}

	pub fn y(&self) -> f32 {
		self.y
	}

	pub fn z(&self) -> f32 {
		self.z
	}
}

impl Add for Point3f {
	type Output = Point3f;

	fn add(self, rhs: Point3f) -> Point3f {
		self.offset(rhs.x, rhs.y, rhs.z)
	}
}

impl Neg for Point3f {
	type Output = Point3f;

	fn neg(self) -> Point3f {
		Point3f::new(-self.x, -self.y, -self.z)
	}
}

impl Sub for Point3f {
	type Output = Point3f;

	fn sub(self, rhs: Point3f) -> Point3f {
		self.offset(-rhs.x, -rhs.y, -rhs.z)
	}
}

impl Mul<f32> for Point3f {
	type Output = Point3f;

	fn mul(self, rhs: f32) -> Point3f {
		Point3f::new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}

impl Div<f32> for Point3f {
	type Output = Point3f;

	fn div(self, rhs: f32) -> Point3f {
		Point3f::new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}