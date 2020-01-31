pub mod fix32;

/*   TODO:
    - overflow checks
    - Benchmark operations looking for performance problems

*/

#[cfg(test)]
mod tests {
    #[test]
    fn creation() {
        let n1 = super::fix32::Fix32::new();
        assert_eq!(n1.n, 0);
        let n1 = super::fix32::Fix32::from_i32(21_i32);
        assert_eq!(n1.n, 21000_i32);
        let n1 = super::fix32::Fix32::from_i64(-4_i64);
        assert_eq!(n1.n, -4000_i32);
        let n1 = super::fix32::Fix32::from_f32(3.1416);
        assert_eq!(n1.n, 3141_i32);
        let n1 = super::fix32::Fix32::from_f64(-1.23456789_f64);
        assert_eq!(n1.n, -1234_i32);
        let n1 = super::fix32::Fix32::with_n(456789_i32);
        assert_eq!(n1.n, 456789_i32);

    }

    #[test]
    fn get_values_from() {
        let mut n1 = super::fix32::Fix32::from_f32(1.23456_f32);
        assert_eq!((n1.get_f32() - 1.234_f32).abs() < 0.001_f32, true);
        assert_eq!((n1.get_f64() - 1.234_f64).abs() < 0.001_f64, true);
        assert_eq!(n1.get_str(), String::from("1.234"));
        assert_eq!(n1.get_i32(), 1_i32);
        assert_eq!(n1.get_rem(), 234_i32);
        n1.trunc();
        assert_eq!(n1.n, 1000_i32);
    }

    #[test]
    fn trait_add() {
        let n1 = super::fix32::Fix32::from_f32(1.23456_f32);
        let n2 = super::fix32::Fix32::from_f32(6.5_f32);
        let res = n1+n2;
        assert_eq!(res.n, 7734_i32);
    }

    #[test]
    fn trait_sub() {
        let n1 = super::fix32::Fix32::from_f32(1.23456_f32);
        let n2 = super::fix32::Fix32::from_f32(6.5_f32);
        let res = n1-n2;
        assert_eq!(res.n, -5266_i32);
    }

    #[test]
    fn trait_mul() {
        let n1 = super::fix32::Fix32::from_f32(1.5_f32);
        let n2 = super::fix32::Fix32::from_f32(-6.5_f32);
        let res = n1*n2;
        assert_eq!(res.n, -9750_i32);
    }

    #[test]
    fn trait_div() {
        let n1 = super::fix32::Fix32::from_f32(1.5_f32);
        let n2 = super::fix32::Fix32::from_f32(-6.5_f32);
        let res = n1/n2;
        assert_eq!(res.n, -230_i32);
    }

        #[test]
    fn trait_rem() {
        let n1 = super::fix32::Fix32::from_f32(100_f32);
        let n2 = super::fix32::Fix32::from_f32(10_f32);
        let res = n1%n2;
        assert_eq!(res.n, 0_i32);
    }

    #[test]
    fn get_abs() {
        let n = super::fix32::Fix32::from_f32(-5.124_f32);
        assert_eq!(n.abs().n, 5124_i32);
    }

    #[test]
    fn get_pow2() {
        let n = super::fix32::Fix32::from_f32(-5.124_f32);
        assert_eq!(n.pow2().n, 26255_i32);
    }

    #[test]
    fn get_pow3() {
        let n = super::fix32::Fix32::from_f32(-5.124_f32);
        assert_eq!(n.pow3().n, -134530_i32);
    }

        #[test]
    fn get_pow() {
        let n = super::fix32::Fix32::from_f32(-2.2_f32);
        assert_eq!(n.pow(5).n, -51535_i32);
    }

    // add here tests made for c++ class
}
