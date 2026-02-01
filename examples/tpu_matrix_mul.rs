//! Example demonstrating polynomial multiplication via TPU acceleration
//! using candle-coreml on Apple Silicon

use candle_core::{Device, Result as CandleResult, Tensor};
use std::time::Instant;

/// Simple polynomial with u64 coefficients
#[derive(Debug, Clone)]
struct Polynomial {
    coefficients: Vec<u64>,
}

impl Polynomial {
    /// Create a new polynomial from coefficients (lowest degree first)
    fn new(coefficients: Vec<u64>) -> Self {
        Polynomial { coefficients }
    }

    /// Naive O(n²) polynomial multiplication
    fn multiply_naive(&self, other: &Polynomial) -> Polynomial {
        let n = self.coefficients.len();
        let m = other.coefficients.len();
        let mut result = vec![0u64; n + m - 1];

        for (i, &a) in self.coefficients.iter().enumerate() {
            for (j, &b) in other.coefficients.iter().enumerate() {
                result[i + j] += a * b;
            }
        }

        Polynomial::new(result)
    }

    /// Convert polynomial to Toeplitz matrix representation for multiplication
    /// Matrix M is Toeplitz where M_{i,j} = a_{i-j} for i >= j, 0 otherwise
    /// When multiplied by vector B, gives A × B
    fn to_multiplication_matrix(&self, result_degree: usize) -> Vec<Vec<u64>> {
        let n = self.coefficients.len();
        let mut matrix = vec![vec![0u64; n]; result_degree];

        for i in 0..result_degree {
            for j in 0..n {
                if i >= j && (i - j) < n {
                    matrix[i][j] = self.coefficients[i - j];
                }
            }
        }

        matrix
    }

    /// Multiply using matrix multiplication via candle-coreml with provided device
    fn multiply_matrix_with_device(
        &self,
        other: &Polynomial,
        device: &Device,
    ) -> CandleResult<Polynomial> {
        let n = self.coefficients.len();
        let m = other.coefficients.len();
        let result_degree = n + m - 1;

        // Build multiplication matrix (Toeplitz: result_degree x n)
        let matrix = self.to_multiplication_matrix(result_degree);
        let matrix_flat: Vec<f32> = matrix
            .iter()
            .flat_map(|row| row.iter().map(|&x| x as f32))
            .collect();

        let matrix_tensor =
            Tensor::from_vec(matrix_flat, (result_degree, n), device)?;

        // Convert other polynomial to vector (n x 1) - need to pad
        let mut vec_b = vec![0.0f32; n];
        for (i, &coeff) in other.coefficients.iter().enumerate() {
            if i < n {
                vec_b[i] = coeff as f32;
            }
        }
        let b_tensor = Tensor::from_vec(vec_b, (n, 1), device)?;

        // Perform matrix multiplication
        let result_tensor = matrix_tensor.matmul(&b_tensor)?;

        // Extract result back to polynomial (reshape from 2D to 1D)
        let result_reshaped = result_tensor.reshape((result_degree,))?;
        let result_vec = result_reshaped.to_vec1::<f32>()?;
        let coefficients: Vec<u64> =
            result_vec.iter().map(|&x| x as u64).collect();

        Ok(Polynomial::new(coefficients))
    }

    /// Compare if two polynomials are approximately equal
    fn approx_eq(&self, other: &Polynomial, epsilon: f64) -> bool {
        let max_len = self.coefficients.len().max(other.coefficients.len());
        for i in 0..max_len {
            let a = self.coefficients.get(i).copied().unwrap_or(0);
            let b = other.coefficients.get(i).copied().unwrap_or(0);
            if (a as f64 - b as f64).abs() > epsilon {
                return false;
            }
        }
        true
    }
}

fn main() -> CandleResult<()> {
    println!("=== TPU Polynomial Multiplication Example ===\n");

    // Initialize Metal device once (to amortize initialization cost)
    let device = Device::new_metal(0)?;

    // Create test polynomials
    let poly_a = Polynomial::new(vec![1u64, 2, 3, 4]); // 1 + 2x + 3x^2 + 4x^3
    let poly_b = Polynomial::new(vec![5u64, 6, 7]); // 5 + 6x + 7x^2

    println!("Polynomial A: {:?}", poly_a);
    println!("Polynomial B: {:?}\n", poly_b);

    // Naive multiplication
    let start = Instant::now();
    let result_naive = poly_a.multiply_naive(&poly_b);
    let time_naive = start.elapsed();
    println!("Naive result: {:?}", result_naive);
    println!("Naive time:    {:?}\n", time_naive);

    // Matrix multiplication via TPU
    let start = Instant::now();
    let result_matrix = poly_a.multiply_matrix_with_device(&poly_b, &device)?;
    let time_matrix = start.elapsed();
    println!("Matrix result: {:?}", result_matrix);
    println!("Matrix time:   {:?}", time_matrix);

    // Verify correctness
    println!(
        "\nResults match: {}",
        result_naive.approx_eq(&result_matrix, 0.1)
    );

    // Larger polynomials for performance comparison
    println!("\n=== Performance Comparison ===\n");

    let degrees = [512, 1024, 2048, 4096, 8192];
    let iterations = 5;

    for deg in degrees {
        let large_poly_a =
            Polynomial::new((0..deg).map(|x| (x as u64 % 1000) + 1).collect());
        let large_poly_b =
            Polynomial::new((0..deg).map(|x| (x as u64 % 1000) + 1).collect());

        println!(
            "Polynomials: degree {} each (averaged over {} iterations)",
            deg, iterations
        );

        // Warm-up
        let _ = large_poly_a.multiply_naive(&large_poly_b);
        let _ =
            large_poly_a.multiply_matrix_with_device(&large_poly_b, &device)?;

        // Naive benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = large_poly_a.multiply_naive(&large_poly_b);
        }
        let large_naive_time = start.elapsed() / iterations as u32;
        println!("  Naive time:   {:?}", large_naive_time);

        // Matrix benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = large_poly_a
                .multiply_matrix_with_device(&large_poly_b, &device)?;
        }
        let large_matrix_time = start.elapsed() / iterations as u32;
        println!("  Matrix time:  {:?}", large_matrix_time);

        let speedup =
            large_naive_time.as_secs_f64() / large_matrix_time.as_secs_f64();
        println!("  Speedup: {:.2}x", speedup);
        println!();
    }

    Ok(())
}
