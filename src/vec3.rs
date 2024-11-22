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
