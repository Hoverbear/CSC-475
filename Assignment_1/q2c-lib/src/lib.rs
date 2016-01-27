extern crate rustfft;
extern crate num;

#[cfg(test)]
mod test {
    use num::complex::Complex;
    use rustfft;

    #[test]
    fn transform() {
        // This library can handle arbitrary FFT lengths, but
        // lengths that are highly composite run much faster.
        let fft_len = 8;

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

        let mut fft = rustfft::FFT::new(fft_len, false);
        let mut test_fft = test.clone();
        fft.process(&test, &mut test_fft);
        
        println!("Want: {:?}", target.to_vec());
        println!("Got:  {:?}", test_fft);
        
        for (test_item, target_item) in test_fft.iter().zip(target.iter()) {
            assert!((*test_item - *target_item).norm_sqr() < 1e-6);
        }
    }
}
