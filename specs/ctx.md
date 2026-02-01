# TPU as Cryptographic Accelerator - Current Task Context

From "^^^" my remarks starting


# Current task

We need to create example file where we'll check the process first i'll need large polynomial with u64 coefficients which i'll assign mul, in classic way and through the TPU. This way we'll figure out datatypes and project layout

## Project Overview

Implementing techniques from "TPU as Cryptographic Accelerator" (arXiv:2307.06554) in Rust. The paper explores using TPU/NPU hardware to accelerate polynomial multiplication for cryptographic schemes like FHE, ZKPs, and Post-Quantum Cryptography.

Paper link: "https://arxiv.org/html/2307.06554v3" (HTML format)

### Core Goal
Create a Rust library that implements the paper's techniques for efficient polynomial multiplication, with focus on:
1. Matrix conversion approach for TPU-like acceleration
2. Residue Number System (RNS) for large coefficient handling
3. Divide-and-conquer strategies for high-degree polynomials

## Key Technical Challenges from Paper

### 1. Polynomial Multiplication Bottleneck
- Direct multiplication: O(n²) complexity
- NTT-based: O(n log n) but with parameter limitations
- Need to handle polynomial rings (typically xⁿ + 1)

### 2. TPU Hardware Constraints
- 8-bit word length limits coefficient size
- Matrix Multiply Unit (256×256 MAC operations)
- Need to decompose operations to fit hardware

### 3. Adaptation Strategies
- **Matrix Conversion**: Transform polynomial multiplication to matrix multiplication
- **RNS Strategy**: Break large coefficients into residues for parallel processing
- **Divide-and-Conquer**: Recursively split high-degree polynomials

## Library Selection Analysis

### Candidate Libraries Evaluated

1. **manzana (0.2.0)** - "Safe Rust interfaces to Apple hardware for Sovereign AI"
   - **Pros**: Direct API for Neural Engine, Metal GPU, Secure Enclave, Afterburner FPGA
   - **Cons**: Neural Engine module is currently a stub implementation (returns zeros, no actual hardware access)
   - **Verdict**: Too immature for production use in current state

2. **candle-coreml (0.3.1)** - Candle tensor integration with CoreML framework
   - **Pros**: Mature, handles CoreML model loading/compilation/inference, automatic HuggingFace model downloading
   - **Cons**: Requires models to be compiled to CoreML format (.mlmodelc), adds compilation step
   - **Verdict**: Production-ready but requires CoreML model compilation

3. **mlx-rs (0.25.3)** - Unofficial Rust bindings to Apple's MLX framework
   - **Pros**: Optimized for Apple Silicon with Neural Engine acceleration, provides array/tensor operations similar to NumPy/PyTorch
   - **Cons**: Unofficial bindings, may have API stability issues
   - **Verdict**: Promising alternative for direct tensor operations without CoreML compilation

### Decision & Rationale

**Initial choice**: `candle-coreml` for Phase 1 development because:
1. It actually works today (manzana's ANE support is stubbed)
2. Integrates with Candle tensor library for smooth transition
3. Can compile custom CoreML models implementing the paper's matrix multiplication
4. Handles device selection (CPU/Metal) automatically

**Alternative path**: `mlx-rs` may be better long-term as it offers direct tensor operations without CoreML compilation overhead. Will evaluate during Phase 3 (Matrix Conversion).

### Implementation Strategy

Let's think about the problem for a while. What we basically want?

To test different kind of polynomial computations on TPU.

I think let's start from abstrctions. Basically we need some entittry which
represent polynomial: "Poly". Poly should have mul and add perations,
preferable assigned versions (copy op is heavy on large polynomials).

So probably we'll use some kind of trait, with different implemenation of math. We should not care about Poly internals and the way we'll represent the data, it will be internal state of Poly object. And it's up to him how to do math.


Current project structure is shit. We'll probably redefine it.

We'll start from implementing trait for u64 coefficients polynomial. Each polynomial should have some kinf of "context" (abstract, could be any, maybe tables for calculations which shared between polynomials)).

## Success Metrics

1. **Correctness**: All arithmetic operations produce mathematically correct results
2. **Performance**: Significant speedup over naive polynomial multiplication
3. **Scalability**: Handle polynomial degrees up to 2¹⁴ (as per paper experiments)
4. **Flexibility**: Support multiple RNS bases and algorithm choices
5. **Usability**: Clean API for cryptographic scheme integration

## Risks & Mitigations

### Technical Risks
- **Matrix decomposition overhead**: Profile and optimize recursion strategy
- **RNS base selection**: Implement automatic base optimization
- **Numerical stability**: Use appropriate integer types and overflow checks

### Implementation Risks
- **Performance tuning complexity**: Start with correct implementation, then optimize
- **Hardware dependency**: Maintain pure Rust fallback implementation
- **Algorithm complexity**: Phase implementation with thorough testing

