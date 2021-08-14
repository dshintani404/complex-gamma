extern crate complex_gamma;

use complex_gamma::ComplexGamma;
use num::complex::Complex;
use rstest::rstest;

#[rstest]
#[case(Complex {re: 2.0, im: 0.0}, Complex {re: 1.0, im: 0.0}, -9)]
#[case(Complex {re: 3.0, im: 0.0}, Complex {re: 2.0, im: 0.0}, -8)]
#[case(Complex {re: 4.0, im: 0.0}, Complex {re: 6.0, im: 0.0}, -9)]
#[case(Complex {re: 5.0, im: 0.0}, Complex {re: 24.0, im: 0.0}, -7)]
#[case(Complex {re: 6.0, im: 0.0}, Complex {re: 120.0, im: 0.0}, -5)]
#[case(Complex {re: 1.0, im: 1.0}, Complex {re: 0.498, im: -0.152}, -2)]
#[case(Complex {re: 10.0, im: 5.0}, Complex {re: 47216.0, im: -91468.0}, 1)]
fn test_complex_gamma(
    #[case] z: Complex<f64>,
    #[case] expected: Complex<f64>,
    #[case] error_digit: i32,
) {
    let cg = ComplexGamma { z: z };
    let actual = cg.value();
    assert_eq!(
        ((actual.re - expected.re) * 10.0f64.powf(-error_digit as f64)).round(),
        0.0
    );
    assert_eq!(
        ((actual.im - expected.im) * 10.0f64.powf(-error_digit as f64)).round(),
        0.0
    )
}
