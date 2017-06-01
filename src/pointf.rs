use std::ops::{Add, Neg, Div, Sub, Mul};


/*
A 2-dimensional vector
*/
#[derive(Copy, Clone, Debug)]
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
#[derive(Copy, Clone, Debug)]
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
		let mut cam_view = Point3f::new(self.x - cam_pos.x(), self.y - cam_pos.y(), self.z - cam_pos.z());

		let pcos = cam_rot.y().to_radians().cos();
		let psin = cam_rot.y().to_radians().sin();

		unborrow!(cam_view.set_x(pcos * cam_view.x() - psin * cam_view.z()));
		unborrow!(cam_view.set_z(pcos * cam_view.z() + psin * cam_view.x()));

		cam_view
	}

	pub fn perspective_project(&self) -> Point3f {
		let mut e = Point3f::new((1024.0 / 2.0) * self.x / self.z,
								(1024.0 / 2.0) * self.y / self.z,
								1.0 / (0.73f32 * 1024.0 / 2.0).tan());

		let mut b = Point3f::new(1024.0 / 2.0 + ((e.z() / self.z) * self.x - e.x()),
								1024.0 / 2.0 + ((e.z() / self.z) * self.y - e.y()),
								self.z);

		b
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