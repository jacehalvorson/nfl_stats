use crate::practice::Complex;

mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let c = Complex { real: 1.0, imaginary: 2.0 };
        assert_eq!(format!("{}", c), "1.0 + 2.0i");
    }

    #[test]
    fn test_display_negative_imaginary() {
        let c = Complex { real: 1.0, imaginary: -2.0 };
        assert_eq!(format!("{}", c), "1.0 - 2.0i");
    }

    #[test]
    fn test_display_zero_imaginary() {
        let c = Complex { real: 1.0, imaginary: 0.0 };
        assert_eq!(format!("{}", c), "1.0 + 0.0i");
    }
}