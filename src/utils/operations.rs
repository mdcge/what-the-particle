use crate::Vec3;

pub fn orthonormal_basis(vec: Vec3) -> (Vec3, Vec3) {
    // Create vector non-colinear with `vec`
    let temp = if vec.norm().0 < 0.999 { // check if vec is colinear with (1.0, 0.0, 0.0)
        Vec3(1.0, 0.0, 0.0)
    } else {
        Vec3(0.0, 1.0, 0.0)
    };
    // Create orthonormal basis (vec, u, v)
    let u = vec.cross(temp).norm();
    let v = vec.cross(u).norm();

    (u, v)
}


// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_vec3_eq;

    #[test]
    fn test_operations_orthonormal_basis() {
        let v1 = Vec3(1.0, 0.0, 0.0);
        let v2 = Vec3(0.0, 1.0, 0.0);
        let v3 = Vec3(0.0, 0.0, 1.0);
        let v4 = Vec3(2.0, 2.0, 2.0);
        let v5 = Vec3(-3.0, -2.0, 6.0);
        assert_vec3_eq!(orthonormal_basis(v1), (Vec3(0.0, 0.0, 1.0), Vec3(0.0, -1.0, 0.0)));
        assert_vec3_eq!(orthonormal_basis(v2), (Vec3(0.0, 0.0, -1.0), Vec3(-1.0, 0.0, 0.0)));
        assert_vec3_eq!(orthonormal_basis(v3), (Vec3(0.0, 1.0, 0.0), Vec3(-1.0, 0.0, 0.0)));
        assert_vec3_eq!(orthonormal_basis(v4), (Vec3(0.0, 1.0/f64::sqrt(2.0), -1.0/f64::sqrt(2.0)), Vec3(-f64::sqrt(2.0/3.0), 1.0/f64::sqrt(6.0), 1.0/f64::sqrt(6.0))));
        assert_vec3_eq!(orthonormal_basis(v5), (Vec3(0.0, 0.9486832980505138, 0.31622776601683794), Vec3(-0.9035079029052513, 0.1355261854357877, -0.40657855630736306)));
    }
}
