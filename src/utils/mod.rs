mod sorted_by_angle;

pub use sorted_by_angle::SortedByAngle;

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    let mut t;
    while b != 0 {
        t = b;
        b = a % b;
        a = t;
    }
    a
}
