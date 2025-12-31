#[macro_export]
macro_rules! assert_vec3_eq {
    ($a:expr, $b:expr $(, $epsilon:expr)? ) => {
        {
            // default epsilon
            let eps =  $( $epsilon )? 1e-12;
            assert!(
                $a == $b,
                "Vec3 not equal: left = {:?}, right = {:?}, epsilon = {}",
                $a,
                $b,
                eps
            );
        }
    };
}
