fn create_point(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32){
    (x, y, z, 1.0f32)
}

fn create_vector(x: f32, y: f32, z: f32) -> (f32, f32, f32, f32){
    (x, y, z, 0.0f32)
}

fn add_tups(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> (f32, f32, f32, f32){
    (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
}

fn sub_tups(a: (f32, f32, f32, f32), b: (f32, f32, f32, f32)) -> (f32, f32, f32, f32){
    (a.0 - b.0, a.1 - b.1, a.2 - b.2, a.3 - b.3)
}

fn negate_tup(a: (f32, f32, f32,f32)) -> (f32, f32, f32, f32){
    (-1f32*a.0, -1f32*a.1, -1f32*a.2, -1f32*a.3)
}

fn mult_tup(a: (f32, f32, f32, f32), b: f32) -> (f32, f32, f32, f32){
    (b*a.0, b*a.1, b*a.2, b*a.3)
}

fn div_tup(a: (f32, f32, f32, f32), b: f32) -> (f32, f32, f32, f32){
    (a.0/b, a.1/b, a.2/b, a.3/b)
}

fn mag_vector(a: (f32, f32, f32, f32)) -> f32{
    assert_ne!(a.3, 0.0);
    (a.0.powf(2.0) + a.1.powf(2.0) + a.2.powf(2.0)).powf(0.5)
}


#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f32 = 0.0001;

    #[test]
    fn test_points_vecs(){
        let a = create_point(4.3, -4.2, 3.1);
        assert_eq!(a.0, 4.3);
        assert_eq!(a.1, -4.2);
        assert_eq!(a.2, 3.1);
        assert_eq!(a.3, 1.0);

        let b = create_vector(4.3, -4.2, 3.1);
        assert_eq!(b.0, 4.3);
        assert_eq!(b.1, -4.2);
        assert_eq!(b.2, 3.1);
        assert_eq!(b.3, 0.0);

    }

    #[test]
    fn test_add_tuples(){
        let a = (3f32, -2f32, 5f32, 1f32);
        let b = (-2f32, 3f32, 1f32, 0f32);
        assert_eq!(add_tups(a, b), (1f32, 1f32, 6f32, 1f32));
    }

    #[test]
    fn test_sub_points(){
        let a = create_point(3f32,2f32,1f32);
        let b = create_point(5f32,6f32,7f32);
        assert_eq!(sub_tups(a, b), create_vector(-2f32, -4f32, -6f32));
    }

    #[test]
    fn test_sub_vector_point(){
        let a = create_point(3f32,2f32,1f32);
        let b = create_vector(5f32,6f32,7f32);
        assert_eq!(sub_tups(a, b), create_point(-2f32, -4f32, -6f32));
    }

    #[test]
    fn test_sub_vectors(){
        let a = create_vector(3f32,2f32,1f32);
        let b = create_vector(5f32,6f32,7f32);
        assert_eq!(sub_tups(a, b), create_vector(-2f32, -4f32, -6f32));
    }

    #[test]
    fn test_negate_tup(){
        let a = (1f32, -2f32, 3f32, -4f32);
        assert_eq!(negate_tup(a), (-1f32, 2f32, -3f32, 4f32));
    }

    #[test]
    fn test_mult_tup(){
        let a = (1f32, -2f32, 3f32, -4f32);
        assert_eq!(mult_tup(a, 3.5), (3.5f32, -7f32, 10.5f32, -14f32));
    }

    #[test]
    fn test_div_tup(){
        let a = (1f32, -2f32, 3f32, -4f32);
        assert_eq!(div_tup(a, 2.0), (0.5f32, -1f32, 1.5f32, -2f32));
    }

    #[test]
    fn test_mag_vector(){
        let v = create_vector(1f32, 2f32, 3f32);
        assert_eq!(mag_vector(v)-14f32.powf(0.5));
    }
}