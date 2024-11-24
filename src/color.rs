use core::ops::Neg;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::AddAssign;
use core::ops::Add;
use core::ops::Sub;
use core::ops::MulAssign;
use core::ops::Mul;
use core::ops::DivAssign;
use core::ops::Div;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub e:[f64;3],
}

impl Color {
    // Function to create an empty vector
    pub fn new_empty() -> Color {
        Color { e : [0.0,0.0,0.0] }
    }

    // Function to create a vector from any numbers castable into f64
    pub fn new_vec<T:Into<f64> + Copy>(x:T, y:T, z:T) -> Color {
        Color { e : [x.into(), y.into(), z.into()] }
    }

    pub fn x(self) -> f64 { return self.e[0] }
    pub fn y(self) -> f64 { return self.e[1] }
    pub fn z(self) -> f64 { return self.e[2] }
    
    pub fn length(self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(self) -> f64 {
        return self.e[0] * self.e[0] +
               self.e[1] * self.e[1] +
               self.e[2] * self.e[2]
    }

    pub fn unit_vector(self) -> Color {
        return self / self.length();
    }

    pub fn dot(u:Color, v:Color) -> f64 {
        return u.e[0] * v.e[0] +
               u.e[1] * v.e[1] +
               u.e[2] * v.e[2]
    }

    pub fn cross(u:Color, v:Color) -> Color {
        return Color { e:
            [u.e[1] * v.e[2] - u.e[2] * v.e[1],
             u.e[2] * v.e[0] - u.e[0] * v.e[2],
             u.e[0] * v.e[1] - u.e[1] * v.e[0] ] 
        }
    }

    pub fn print(self) { println!("{} {} {}", self.e[0], self.e[1],self.e[2]);}
    pub fn printerr(self) { eprintln!("{} {} {}", self.e[0], self.e[1],self.e[2]);}

    pub fn write_color(pixel_color:Color) {
        let r = pixel_color.x();
        let g = pixel_color.y();
        let b = pixel_color.z();

        let rbyte = (255.99 * r) as i32;
        let gbyte = (255.99 * g) as i32;
        let bbyte = (255.99 * b) as i32;

        println!("{rbyte} {gbyte} {bbyte}");
    }
}

// Allows for negation (-) operator with vector
impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        return Color {e:[-self.e[0],-self.e[1],-self.e[2]]}
    }
}

// Allows for indexing the vector, returning the value of 
// the vector at that index
impl Index<usize> for Color {
    type Output = f64;

    fn index(&self, index:usize) -> &f64 { 
        return &self.e[index] 
    }
}

// Allows for indexing the vector mutably, returning the location
// of the value in the vector at that index
impl IndexMut<usize> for Color {
    //type Output = f64;

    fn index_mut(&mut self, index:usize) -> &mut Self::Output { 
        return &mut self.e[index] 
    }
}

//  vec3& operator+=(const vec3& v) {
impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [ self.e[0] + other.e[0],
                 self.e[1] + other.e[1],
                 self.e[2] + other.e[2] ]
        };
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        return Color {
            e: [ self.e[0] + other.e[0],
                 self.e[1] + other.e[1],
                 self.e[2] + other.e[2] ] }
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        return Color {
            e: [ self.e[0] - other.e[0],
                 self.e[1] - other.e[1],
                 self.e[2] - other.e[2] ] }
    }
}

//  vec3& operator*=(double t) {
impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [ self.e[0] * other,
                 self.e[1] * other,
                 self.e[2] * other ]
        };
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        return Color {
            e: [ other.e[0] * self,
                 other.e[1] * self,
                 other.e[2] * self ] }
    }
}

// Scalar multiplication
impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        return Color {
            e: [ self.e[0] * other.e[0],
                 self.e[1] * other.e[1],
                 self.e[2] * other.e[2] ] }
    }
}

//  vec3& operator/=(double t) {
impl DivAssign<f64> for Color {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0/other;
    }
}

// Scalar division
impl Div<f64> for Color {
    type Output = Color;
    fn div(self, other: f64) -> Color {
        return (1.0/other) * self;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec_empty() {
        let blank_vec = Color::new_empty();
        assert_eq!(blank_vec.e[0],  0.0);
        assert_eq!(blank_vec.e[1],  0.0);
        assert_eq!(blank_vec.e[2],  0.0);
        vec_equality(&blank_vec);
        vec_negation(&blank_vec);

        let pos_vec = Color::new_vec(1, 2, 3);
        assert_eq!(pos_vec.e[0],  1.0);
        assert_eq!(pos_vec.e[1],  2.0);
        assert_eq!(pos_vec.e[2],  3.0);
        vec_equality(&pos_vec);
        vec_negation(&pos_vec);
    }

    fn vec_equality(test_vec: &Color) {
        assert_eq!(test_vec.e[0],  test_vec.e[0]);
        assert_eq!(test_vec.e[1],  test_vec.e[1]);
        assert_eq!(test_vec.e[2],  test_vec.e[2]);
    }

    fn vec_negation(test_vec: &Color) {
        let negtest_vec = -*test_vec;
        assert_eq!(negtest_vec.e[0],  -test_vec.e[0]);
        assert_eq!(negtest_vec.e[1],  -test_vec.e[1]);
        assert_eq!(negtest_vec.e[2],  -test_vec.e[2]);
    }
}
