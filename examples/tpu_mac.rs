//! Simple TPU MAC (Multiply-Accumulate) Operation Demo
//!
//! This demonstrates basic matrix multiplication using Metal/TPU on Apple Silicon
//! Matrix multiplication is composed of MAC operations: result[i][j] = sum(A[i][k] * B[k][j])

use candle_core::{Device, Result as CandleResult, Tensor};

fn main() -> CandleResult<()> {
    println!("=== TPU MAC Operation Demo ===\n");

    // Initialize Metal device
    let device = Device::new_metal(0)?;
    println!("Metal device: {:?}\n", device);

    // Create two 2x2 matrices
    // Matrix A:
    // [1.0, 2.0]
    // [3.0, 4.0]
    let a = Tensor::from_vec(vec![1.0f32, 2.0, 3.0, 4.0], (2, 2), &device)?;

    // Matrix B:
    // [5.0, 6.0]
    // [7.0, 8.0]
    let b = Tensor::from_vec(vec![5.0f32, 6.0, 7.0, 8.0], (2, 2), &device)?;

    println!("Matrix A (2x2):");
    print_matrix(&a.to_vec2::<f32>()?);

    println!("Matrix B (2x2):");
    print_matrix(&b.to_vec2::<f32>()?);

    // Perform matrix multiplication (uses MAC operations)
    // Expected result:
    // [1*5 + 2*7, 1*6 + 2*8] = [19, 22]
    // [3*5 + 4*7, 3*6 + 4*8] = [43, 50]
    let result = a.matmul(&b)?;

    println!("Result (A × B) - computed via TPU MAC operations:");
    print_matrix(&result.to_vec2::<f32>()?);

    // Verify result
    let result_vec = result.to_vec2::<f32>()?;
    let result_flat: Vec<f32> = result_vec.iter().flatten().copied().collect();
    let expected = vec![19.0f32, 22.0, 43.0, 50.0];
    println!("Expected: {:?}", expected);
    println!("Got:      {:?}", result_flat);
    println!("Match: {}", result_flat == expected);

    Ok(())
}

fn print_matrix(data: &[Vec<f32>]) {
    for row in data {
        let row_str: Vec<String> =
            row.iter().map(|&x| format!("{:>6.1}", x)).collect();
        println!("  [{}]", row_str.join(", "));
    }
    println!();
}
