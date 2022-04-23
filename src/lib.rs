use std::vec::Vec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn normalize_returns_0_0_if_no_length(){
        let start = [0.,0.];
        let result = normalize(start);
        assert_eq!(result[0], 0.);
        assert_eq!(result[1], 0.);
    }

    #[test]
    fn normalize_returns_correctly_for_horizontal_vector(){
        let start = [15.,0.];
        let result = normalize(start);
        assert_eq!(result[0], 1.);
        assert_eq!(result[1], 0.);
    }
    
    #[test]
    fn normalize_returns_correctly_for_vertical_vector(){
        let start = [0.,42.];
        let result = normalize(start);
        assert_eq!(result[0], 0.);
        assert_eq!(result[1], 1.);
    }    
    
    #[test]
    fn normalize_returns_correctly_for_angled_vector(){
        let start = [3.,4.];
        let result = normalize(start);
        assert_eq!(result[0], 3./5.);
        assert_eq!(result[1], 4./5.);
    }

    #[test]
    fn get_force_vector_returns_high_val_nbr_if_at_center(){
        let x = 542.;
        let y = -123.;
        let pole = Pole2D {loc: [x,y], mass: 10.};
        let result = pole.get_force_vector(6.6, [x,y]);
        
        assert_eq!(result[0], f32::MAX);
        assert_eq!(result[1], f32::MAX);
    }

    #[test]
    fn get_vec_returns_0_0_for_default_bounded_flow_field_2d(){
        let bounded = BoundedFlowfield2D::default();

        let zero = bounded.get_vec(0.0, 0.0);
        let first = bounded.get_vec(1.0, 3.2);
        let second = bounded.get_vec(-4.1223, 2.5);
        let third = bounded.get_vec(-5.21, -2.789);
        let fourth = bounded.get_vec(6.2, 0.45);

        assert_eq!(zero, [0.,0.]);
        assert_eq!(first, [0.,0.]);
        assert_eq!(second, [0.,0.]);
        assert_eq!(third, [0.,0.]);
        assert_eq!(fourth, [0.,0.]);
    }
}

// A 2D vector.
pub type Vector2<T> = [T; 2];

// A 3D vector.
pub type Vector3<T> = [T; 3];

#[derive(Clone, Copy)]
pub struct Pole2D {
    loc: Vector2<f32>,
    mass: f32,
}

impl Pole2D{
    pub fn new(x: f32, y: f32, mass: f32) -> Self {
        let pole = Pole2D {
            loc: [x,y],
            mass
        };

        pole
    }

    pub fn get_force_vector(& self, coeff: f32, p: Vector2<f32>) -> Vector2<f32>{
        let dx = p[0] - self.loc[0];
        let dy = p[1] - self.loc[1];
        
        let mut x = coeff * self.mass / (dx * dx);
        let mut y = coeff * self.mass / (dy * dy);

        if x == f32::INFINITY {
            x = f32::MAX;
        }
        if x== f32::NEG_INFINITY {
            x = f32::MIN;
        }
        if y == f32::INFINITY {
            y = f32::MAX;
        }
        if y== f32::NEG_INFINITY {
            y = f32::MIN;
        }
        
        [x,y]
    }
}

pub trait Flowfield2D {
    fn get_vec (&self, x: f32, y: f32) -> Vector2<f32>;
    fn get_normalized_vec (&self, x: f32, y: f32) -> Vector2<f32>;
    fn get_perp_vec (&self, x: f32, y: f32) -> Vector2<f32>;
    fn get_perp_normalized_vec (&self, x: f32, y: f32) -> Vector2<f32>;

    fn add_pole(&mut self, pole: Pole2D);
}

pub struct BoundedFlowfield2D {
    poles: Vec<Pole2D>,
    coeff: f32,
}

impl BoundedFlowfield2D {
    pub fn default() -> Self {
        BoundedFlowfield2D {poles: vec![], coeff: 6.67 }
    }    
}

impl Flowfield2D for BoundedFlowfield2D {    
    fn get_vec(&self, x: f32, y: f32) -> [f32; 2] { 
        let mut cur_x = 0.0;
        let mut cur_y = 0.0;

        for pole in &self.poles {
            let pole_vec = (*pole).get_force_vector(self.coeff, [x,y]);
            cur_x += pole_vec[0];
            cur_y += pole_vec[1];
        }

        [cur_x,cur_y]
    }

    fn get_normalized_vec(&self, x: f32, y: f32) -> [f32; 2] { 
        let vec = self.get_vec(x,y);
        normalize(vec)
     }
    fn get_perp_vec(&self, x: f32, y: f32) -> [f32; 2] {
        let vec = self.get_vec(x,y);

        [vec[1], -vec[0]]
    }
    fn get_perp_normalized_vec(&self, x: f32, y: f32) -> [f32; 2] { 
        let vec = self.get_perp_vec(x,y);        
        normalize(vec)
     }
        
    fn add_pole(&mut self, pole: Pole2D){
        self.poles.push(pole);
    }
    
}


fn normalize(vec: [f32; 2]) -> [f32;2] {
    let length = ((vec[0] * vec[0]) + (vec[1] * vec[1])).sqrt();
    if length == 0.0 {
        return [0.,0.];
    }
    
    [vec[0]/length, vec[1]/length]
}



