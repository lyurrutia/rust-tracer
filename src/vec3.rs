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
pub struct Vec3 {
    pub e:[f64;3],
}

impl Vec3 {
    // Function to create an empty vector
    pub fn new_empty() -> Vec3 {
        Vec3 { e : [0.0,0.0,0.0] }
    }

    // Function to create a vector from any numbers castable into f64
    pub fn new_vec<T:Into<f64> + Copy>(x:T, y:T, z:T) -> Vec3 {
        Vec3 { e : [x.into(), y.into(), z.into()] }
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

    pub fn unit_vector(self) -> Vec3 {
        return self / self.length();
    }

    pub fn dot(u:Vec3, v:Vec3) -> f64 {
        return u.e[0] * v.e[0] +
               u.e[1] * v.e[1] +
               u.e[2] * v.e[2]
    }

    pub fn cross(u:Vec3, v:Vec3) -> Vec3 {
        return Vec3 { e:
            [u.e[1] * v.e[2] - u.e[2] * v.e[1],
             u.e[2] * v.e[0] - u.e[0] * v.e[2],
             u.e[0] * v.e[1] - u.e[1] * v.e[0] ] 
        }
    }

    pub fn print(self) { println!("{} {} {}", self.e[0], self.e[1],self.e[2]);}
    pub fn printerr(self) { eprintln!("{} {} {}", self.e[0], self.e[1],self.e[2]);}
}

// Allows for negation (-) operator with vector
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3 {e:[-self.e[0],-self.e[1],-self.e[2]]}
    }
}

// Allows for indexing the vector, returning the value of 
// the vector at that index
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index:usize) -> &f64 { 
        return &self.e[index] 
    }
}

// Allows for indexing the vector mutably, returning the location
// of the value in the vector at that index
impl IndexMut<usize> for Vec3 {
    //type Output = f64;

    fn index_mut(&mut self, index:usize) -> &mut Self::Output { 
        return &mut self.e[index] 
    }
}

//  vec3& operator+=(const vec3& v) {
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [ self.e[0] + other.e[0],
                 self.e[1] + other.e[1],
                 self.e[2] + other.e[2] ]
        };
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [ self.e[0] + other.e[0],
                 self.e[1] + other.e[1],
                 self.e[2] + other.e[2] ] }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [ self.e[0] - other.e[0],
                 self.e[1] - other.e[1],
                 self.e[2] - other.e[2] ] }
    }
}

//  vec3& operator*=(double t) {
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [ self.e[0] * other,
                 self.e[1] * other,
                 self.e[2] * other ]
        };
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [ other.e[0] * self,
                 other.e[1] * self,
                 other.e[2] * self ] }
    }
}

// Scalar multiplication
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        return Vec3 {
            e: [ self.e[0] * other.e[0],
                 self.e[1] * other.e[1],
                 self.e[2] * other.e[2] ] }
    }
}

//  vec3& operator/=(double t) {
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0/other;
    }
}

// Scalar division
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        return (1.0/other) * self;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec_empty() {
        let blank_vec = Vec3::new_empty();
        assert_eq!(blank_vec.e[0],  0.0);
        assert_eq!(blank_vec.e[1],  0.0);
        assert_eq!(blank_vec.e[2],  0.0);
        vec_equality(&blank_vec);
        vec_negation(&blank_vec);

        let pos_vec = Vec3::new_vec(1, 2, 3);
        assert_eq!(pos_vec.e[0],  1.0);
        assert_eq!(pos_vec.e[1],  2.0);
        assert_eq!(pos_vec.e[2],  3.0);
        vec_equality(&pos_vec);
        vec_negation(&pos_vec);
    }

    fn vec_equality(test_vec: &Vec3) {
        assert_eq!(test_vec.e[0],  test_vec.e[0]);
        assert_eq!(test_vec.e[1],  test_vec.e[1]);
        assert_eq!(test_vec.e[2],  test_vec.e[2]);
    }

    fn vec_negation(test_vec: &Vec3) {
        let negtest_vec = -*test_vec;
        assert_eq!(negtest_vec.e[0],  -test_vec.e[0]);
        assert_eq!(negtest_vec.e[1],  -test_vec.e[1]);
        assert_eq!(negtest_vec.e[2],  -test_vec.e[2]);
    }
}
