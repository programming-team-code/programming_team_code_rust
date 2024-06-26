//! Fast Fourier Transform (FFT) implementation ripped from: <https://github.com/bminaiev/rust-contests/blob/main/algo_lib/src/math/fft.rs>
use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    const ZERO: Self = Complex {
        real: 0.0,
        imag: 0.0,
    };
    const ONE: Self = Complex {
        real: 1.0,
        imag: 0.0,
    };
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

fn fft(a: &mut [Complex], invert: bool) {
    let n = a.len();
    assert!(n.is_power_of_two());
    let shift = usize::BITS - n.trailing_zeros();
    for i in 1..n {
        let j = (i << shift).reverse_bits();
        assert!(j < n);
        if i < j {
            a.swap(i, j);
        }
    }
    for len in (1..).map(|x| 1 << x).take_while(|s| *s <= n) {
        let half = len / 2;
        let alpha = std::f64::consts::PI * 2.0 / (len as f64);
        let cos = f64::cos(alpha);
        let sin = f64::sin(alpha) * (if invert { -1.0 } else { 1.0 });
        let complex_angle = Complex {
            real: cos,
            imag: sin,
        };
        for start in (0..n).step_by(len) {
            let mut mult = Complex::ONE;
            for j in 0..half {
                let u = a[start + j];
                let v = a[start + half + j] * mult;
                a[start + j] = u + v;
                a[start + j + half] = u - v;
                mult *= complex_angle;
            }
        }
    }
    if invert {
        for elem in a.iter_mut().take(n) {
            let n = n as f64;
            elem.imag /= n;
            elem.real /= n;
        }
    }
}

fn fft_multiply_raw(mut a: Vec<Complex>, mut b: Vec<Complex>) -> Vec<Complex> {
    assert!(a.len().is_power_of_two());
    assert!(b.len().is_power_of_two());
    assert_eq!(a.len(), b.len());
    fft(&mut a, false);
    fft(&mut b, false);
    for (x, y) in a.iter_mut().zip(b.iter()) {
        *x *= *y;
    }
    fft(&mut a, true);
    a
}

fn fft_multiply_complex(mut a: Vec<Complex>, mut b: Vec<Complex>) -> Vec<Complex> {
    let expected_size = (a.len() + b.len() - 1).next_power_of_two();
    a.resize(expected_size, Complex::ZERO);
    b.resize(expected_size, Complex::ZERO);
    fft_multiply_raw(a, b)
}

/// # Example
/// ```
/// use programming_team_code_rust::numbers::fft::fft_multiply;
///
/// let a = vec![1.0, 2.0, 3.0];
/// let b = vec![4.0, 5.0, 6.0];
/// let c = fft_multiply(a, b);
/// let expected = vec![4.0, 13.0, 28.0, 27.0, 18.0];
/// for (x, y) in c.iter().zip(expected.iter()) {
///    assert!((x - y).abs() < 1e-15);
/// }
/// ```
///
/// # Complexity (n = max(a.len(), b.len()))
/// Given for practical use cases, although consider it with a relatively large constant factor
/// - Time: O(n log n)
/// - Space: O(n)
pub fn fft_multiply(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let a: Vec<_> = a.iter().map(|&x| Complex { real: x, imag: 0.0 }).collect();
    let b: Vec<_> = b.iter().map(|&x| Complex { real: x, imag: 0.0 }).collect();
    fft_multiply_complex(a, b).iter().map(|c| c.real).collect()
}
