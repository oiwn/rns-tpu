# Agent Guidelines for rns-tpu Project

## Project Overview

This is a Rust implementation of techniques from "TPU as Cryptographic Accelerator" (arXiv:2307.06554). The project aims to accelerate polynomial multiplication for cryptographic schemes (FHE, ZKPs, PQC) using TPU/NPU-inspired techniques.

**Key Context**: See [specs/ctx.md](specs/ctx.md) for detailed implementation plan and paper analysis.

## Essential Commands

### Build & Run
```bash
cargo build          # Debug build
cargo build --release  # Release build
cargo run            # Run main executable
cargo check          # Type checking without building
```

### Testing
```bash
cargo test           # Run all tests
cargo test -- --nocapture  # Run tests with output
cargo test <test_name>     # Run specific test
```

### Code Quality
```bash
cargo fmt           # Format code (respects rustfmt.toml)
cargo clippy        # Lint with Clippy
cargo clippy -- -D warnings  # Treat warnings as errors
```

### Development Tools
```bash
just release        # Build release version (from Justfile)
```

## Project Structure

```
src/
├── main.rs         # Entry point (minimal - library focus)
├── lib.rs          # Library exports (to be created)
├── polynomial/     # Polynomial arithmetic module
├── rns/            # Residue Number System module
├── matrix/         # Matrix conversion module
├── algorithms/     # Multiplication algorithms
```

**Note**: The project is primarily a library. `main.rs` should remain minimal, with core logic in library modules.

## Code Style & Conventions

### Formatting Rules (from rustfmt.toml)
- **Max line width**: 82 characters
- **Edition**: 2024
- **Imports granularity**: "Crate" (group imports by crate)
- **Doc comments**: Formatted
- **Indentation**: 4 spaces for Rust files (per .editorconfig)

### EditorConfig Standards
- **General**: 2 space indentation, UTF-8, LF line endings
- **Rust files**: 4 space indentation
- **Markdown**: No trailing whitespace trimming
- **YAML/JSON**: 2 space indentation
- **Justfile**: Tab indentation, 4 spaces per tab

### Naming Conventions
- Follow standard Rust naming conventions (snake_case for variables/functions, CamelCase for types)
- Use descriptive names for mathematical operations
- Prefer `_t` suffix for type parameters in mathematical contexts (e.g., `T: Integer`)

### Mathematical Notation
**CRITICAL**: Use **standard ASCII characters only** for mathematical notation. No Unicode symbols.

```rust
// GOOD: ASCII only
fn polynomial_mul(a: &Polynomial, b: &Polynomial) -> Polynomial
fn ntt_transform(x: &[Complex]) -> Vec<Complex>

// BAD: Unicode symbols
fn polynomial_×(a: &Polynomial, b: &Polynomial) -> Polynomial  // × is Unicode
fn α_transform(x: &[Complex]) -> Vec<Complex>  // α is Unicode
```

Allowed mathematical notation in identifiers:
- Single letters: `x`, `y`, `z`, `n`, `m`, `k` (for indices, sizes)
- Common abbreviations: `coeff`, `poly`, `mod`, `div`, `gcd`, `lcm`
- Mathematical terms: `polynomial`, `coefficient`, `modulus`, `residue`, `matrix`

## Development Workflow

### 1. Before Making Changes
- Run `cargo check` to ensure current code compiles
- Review `specs/ctx.md` for project context and implementation plan
- Check existing tests to understand expected behavior

### 2. Implementing Features
- Follow the phases outlined in `specs/ctx.md`
- Create modules in appropriate directories
- Write unit tests alongside implementation
- Use `#[cfg(test)]` modules within source files

### 3. Testing Strategy
- **Unit tests**: Test individual functions and data structures
- **Property-based tests**: Verify mathematical properties (commutativity, associativity, etc.)
- **Integration tests**: Test cross-module functionality
- **Benchmarks**: Use `criterion` for performance-critical code

### 4. Code Review Checklist
- [ ] Compiles without warnings (`cargo clippy -- -D warnings`)
- [ ] Passes all tests (`cargo test`)
- [ ] Formatted correctly (`cargo fmt`)
- [ ] Mathematical correctness verified
- [ ] No unintended performance regressions
- [ ] ASCII-only mathematical notation

## Project-Specific Patterns

### Polynomial Representation
- Coefficients stored in `Vec<T>` from lowest to highest degree
- Support for polynomial rings (x^n + 1)
- Generic over integer types (eventually need 8-bit support for TPU simulation)

### Residue Number System (RNS)
- Multiple residue bases (8, 16, 128 as per paper)
- Parallel processing of residues
- Efficient conversion between standard and RNS representation

### Matrix Conversion
- Transform polynomial multiplication to matrix multiplication
- Special matrix structure incorporating ring modulo operation
- Optimization for 256×256 operations (TPU-like)

## Common Gotchas

1. **Integer Overflow**: Cryptographic operations use large numbers. Use appropriate integer types (`u64`, `u128`, or big integer libraries).

2. **Modular Arithmetic**: Polynomial rings use (x^n + 1) modulus. Ensure correct reduction.

3. **Performance**: Polynomial multiplication is O(n²) naive, O(n log n) with NTT. Choose algorithm based on degree.

4. **TPU Constraints**: Simulate 8-bit word length limitations when designing data structures.

5. **Parallelism**: RNS naturally parallelizes across residues. Use `rayon` for CPU parallelism.

## Hardware Acceleration Library Selection

### Library Analysis (see specs/ctx.md for detailed analysis)

**Selected library**: `candle-coreml` for initial development
- **Why**: Manzana's Neural Engine support is currently stubbed; candle-coreml is production-ready
- **Alternative**: `mlx-rs` (Rust bindings to Apple's MLX framework) may be evaluated later
- **Strategy**: Pure Rust for Phase 1-2, integrate candle-coreml for Phase 3 matrix multiplication

### Key Constraints
- Apple Neural Engine: Simulate 8-bit word length for TPU constraints
- Matrix dimensions: Optimize for 256×256 MAC operations as per paper
- Polynomial degrees: Support up to 2¹⁴ as per paper experiments

### Implementation Notes
- Use `candle-coreml` with CoreML model compilation for matrix multiplication
- Maintain pure Rust fallback implementation for portability
- Evaluate `mlx-rs` in Phase 5 for potential performance improvements

## Dependencies

Current dependencies (check `Cargo.toml` for updates):
- Standard library only (initial phase)
- Planned: `num-traits`, `rayon`, `candle-coreml`, `criterion`
- Future evaluation: `mlx-rs` for potential performance improvements

**Important**: Keep dependencies minimal for cryptographic code. Audit all third-party crates.

## Dependency Management

### Adding Dependencies
Always use `cargo add` to ensure latest compatible versions:

```bash
# Core dependencies for polynomial arithmetic and parallel processing
cargo add num-traits
cargo add rayon

# Apple hardware acceleration (for Phase 3+)
cargo add candle-coreml

# Benchmarking (development dependency)
cargo add --dev criterion
```

### Updating Dependencies
```bash
cargo update          # Update all dependencies to latest compatible versions
cargo update --dry-run  # Preview updates without applying
```

### Verifying Dependencies
```bash
cargo tree            # Show dependency tree
cargo outdated        # Check for outdated dependencies
cargo audit          # Security audit (requires cargo-audit)
```

### Dependency Strategy
1. **Phase 1-2**: Use only `num-traits` and `rayon` for pure Rust implementation
2. **Phase 3**: Add `candle-coreml` for CoreML-based matrix multiplication
3. **Phase 5**: Evaluate `mlx-rs` as alternative to `candle-coreml`

## Documentation

- Use Rustdoc comments (`///`) for public API
- Include mathematical explanations in documentation
- Document performance characteristics (time/space complexity)
- Include examples showing typical usage

## Continuous Integration

GitHub workflows exist for:
- **Test**: `cargo test` on push/pull request
- **Code Quality**: Formatting and linting checks
- **Platform Coverage**: Multi-platform testing

Ensure all CI checks pass before considering work complete.

## Emergency Contacts

- **Project Context**: `specs/ctx.md`
- **Rustfmt Config**: `rustfmt.toml`
- **Editor Settings**: `.editorconfig`
- **Build Scripts**: `Justfile`

## Quick Reference

```bash
# Full development cycle
cargo check && cargo test && cargo fmt && cargo clippy -- -D warnings

# Run specific phase tests
cargo test --test polynomial_arithmetic

# Benchmark performance
cargo bench

# Generate documentation
cargo doc --open
```

**Remember**: This is a cryptographic library. Correctness is paramount over performance. All mathematical operations must be rigorously tested.
