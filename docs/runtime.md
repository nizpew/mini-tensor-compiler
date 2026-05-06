# Runtime System

## Overview

The runtime system is responsible for executing optimized computation graphs on CPU hardware. It consists of two main components:

1. **Executor**: Traverses the graph and orchestrates execution
2. **Kernels**: Low-level implementations of operations

## Executor

The `Executor` manages the execution flow:

```rust
pub struct Executor {
    tensors: HashMap<String, ndarray::Array2<f32>>,
}
```

### Execution Process

1. **Graph Traversal**: Visit operations in topological order
2. **Kernel Dispatch**: Call appropriate kernel for each operation
3. **Memory Management**: Track intermediate tensors
4. **Error Handling**: Report execution errors

### Example Execution

```rust
let mut executor = Executor::new();
executor.execute(&graph)?;
```

## Kernels

Kernels are the actual computation implementations using `ndarray`:

### Matrix Multiplication
```rust
pub fn matmul(a: &Array2<f32>, b: &Array2<f32>) -> Array2<f32> {
    a.dot(b)
}
```

### ReLU Activation
```rust
pub fn relu(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| v.max(0.0))
}
```

### Element-wise Addition
```rust
pub fn add(a: &Array2<f32>, b: &Array2<f32>) -> Array2<f32> {
    a + b
}
```

### Fused Linear + ReLU
```rust
pub fn fused_linear_relu(x: &Array2<f32>, weight: &Array2<f32>, bias: &Array2<f32>) -> Array2<f32> {
    let linear = x.dot(weight) + bias;
    relu(&linear)
}
```

## Memory Management

The runtime uses a simple tensor registry:

- **Input Tensors**: Provided by the user
- **Intermediate Tensors**: Created during execution
- **Output Tensors**: Final results

## Performance Considerations

### Kernel Optimization

1. **Cache Locality**: Operations access memory sequentially
2. **Vectorization**: ndarray uses SIMD when possible
3. **Memory Allocation**: Minimize allocations during execution

### Fusion Benefits

Fused operations provide significant performance benefits:

- **Reduced Memory Traffic**: Intermediate results stay in CPU registers
- **Fewer Kernel Calls**: Lower overhead
- **Better Cache Utilization**: Improved temporal locality

## Future Extensions

### Multi-threading
- Parallel execution of independent operations
- Thread-safe kernel implementations

### SIMD Optimizations
- Hand-optimized kernels for specific operations
- Target-specific optimizations

### Memory Pool
- Pre-allocated memory buffers
- Reduced allocation overhead

### GPU Support
- CUDA/OpenCL backend
- Heterogeneous execution

## Example Usage

```rust
use mini_tensor_compiler::runtime::{Executor, kernels};
use ndarray::arr2;

// Create executor
let mut executor = Executor::new();

// Create input tensors
let input = arr2(&[[1.0, 2.0], [3.0, 4.0]]);
let weight = arr2(&[[0.5, 0.5], [0.5, 0.5]]);

// Execute graph
executor.execute(&graph)?;

// Access results
let output = executor.get_tensor("output");
```

## Error Handling

The runtime provides detailed error information:

- **Shape Mismatches**: Incompatible tensor dimensions
- **Type Errors**: Unsupported data types
- **Memory Errors**: Allocation failures
- **Execution Errors**: Kernel-specific failures
