use std::f32::consts::PI;
use std::ops::{Add, Mul, Sub};

const NUM_SAMPLES: usize = 2048;

// FFT Details

fn fft(signal: &[Complex]) -> Vec<Complex> {
    // Stopping condition.
    if signal.len() <= 1 {
        return signal.to_vec()
    }

    // Build the stride buffers.
    let (mut evens, mut odds) = (
        Vec::with_capacity(signal.len() / 2),
        Vec::with_capacity(signal.len() / 2)
    );

    // Make even and odd strides.
    for (tick, value) in signal.iter().enumerate() {
        if tick % 2 == 0 {
            evens.push(value.clone())
        } else {
            odds.push(value.clone())
        }
    }

    // Recurse
    let evens_fft = fft(&evens[..]);
    let odds_fft = fft(&odds[..]);

    // Build the output spectrum.
    let mut spectrum = vec![
        Complex::new(0.0, 0.0);
        signal.len()
    ];
    
    // Process.
    for index in 0..signal.len() / 2 {
        // Determine the twiddle value.
        let twiddle = Complex::from_polar(
            1.0,
            -2.0 * PI * (index as f32) / (signal.len() as f32)
        );
        spectrum[index] = evens_fft[index] + (twiddle * odds_fft[index]);
        spectrum[index + (signal.len() / 2)] = evens_fft[index] - (twiddle * odds_fft[index]);
    }

    // Pass back.
    spectrum
}

// Complex Number Details
#[derive(Debug, Clone, Copy)]
struct Complex {
    real:      f32,
    imaginary: f32,
}

// A simple partial implementation of what we can find in `num`
impl Complex {
    #[inline]
    fn new(real: f32, imaginary: f32) -> Self {
        Complex {
            real: real,
            imaginary: imaginary,
        }
    }
    
    #[inline]
    fn from_polar(r: f32, theta: f32) -> Complex {
        Complex {
            real: r * theta.cos(),
            imaginary: r * theta.sin(),
        }
    }

    /// Returns the square of the norm (since `T` doesn't necessarily
    /// have a sqrt function), i.e. `re^2 + im^2`.
    #[inline]
    fn norm_sqr(&self) -> f32 {
        self.real.clone() * self.real.clone() + self.imaginary.clone() * self.imaginary.clone()
    }
}

// (a + i b) + (c + i d) == (a + c) + i (b + d)
impl Add<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imaginary + other.imaginary)
    }
}

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl Mul<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        let real = self.real.clone() * other.real.clone() - self.imaginary.clone() * other.imaginary.clone();
        let imaginary = self.real * other.imaginary + self.imaginary * other.real;
        Complex::new(real, imaginary)
    }
}

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl Sub<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imaginary - other.imaginary)
    }
}

// Test

#[cfg(test)]
mod test {
    use super::fft;
    use Complex;

    #[test]
    fn transform() {
        let test = [
            Complex::new(1f32, 0f32),
            Complex::new(1f32, 0f32),
            Complex::new(1f32, 0f32),
            Complex::new(1f32, 0f32),
            Complex::new(0f32, 0f32),
            Complex::new(0f32, 0f32),
            Complex::new(0f32, 0f32),
            Complex::new(0f32, 0f32)
        ];
        let target = [
            Complex::new(4f32, 0f32),
            Complex::new(1f32, -2.414f32),
            Complex::new(0f32, 0f32),
            Complex::new(1f32, -0.414f32),
            Complex::new(0f32, 0f32),
            Complex::new(1f32, 0.414f32),
            Complex::new(0f32, 0f32),
            Complex::new(1f32, 2.414f32)
        ];

        let test_fft = fft(&test);
        println!("Want: {:?}", target.to_vec());
        println!("Got:  {:?}", test_fft);
        for (test_item, target_item) in test_fft.iter().zip(target.iter()) {
            assert!((*test_item - *target_item).norm_sqr() < 1e-6);
        }
    }
}
