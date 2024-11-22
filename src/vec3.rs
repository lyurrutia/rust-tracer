use core::ops::Neg;
use core::ops::Index;
use core::ops::IndexMut;

#[derive(Debug, Copy, Clone)]
struct Vec3 {
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
    
}

// Allows for negation (-) operator with vector
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        return Vec3 {e:[-self.x(),-self.y(),-self.z()]}
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
        assert_eq!(test_vec.x(),  test_vec.e[0]);
        assert_eq!(test_vec.y(),  test_vec.e[1]);
        assert_eq!(test_vec.z(),  test_vec.e[2]);
    }

    fn vec_negation(test_vec: &Vec3) {
        let negtest_vec = -*test_vec;
        assert_eq!(negtest_vec.x(),  -test_vec.e[0]);
        assert_eq!(negtest_vec.y(),  -test_vec.e[1]);
        assert_eq!(negtest_vec.z(),  -test_vec.e[2]);
    }
}
