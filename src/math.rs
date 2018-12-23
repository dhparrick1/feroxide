/// Calculate Greatest Common Divisor (GCD), using Euclides' algorithm
pub fn gcd(x: i32, y: i32) -> i32 {
    // Store the highest in a, the lowest in b
    let (mut a, mut b) = {
        if x > y {
            (x, y)
        } else {
            (y, x)
        }
    };

    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }

    a
}
