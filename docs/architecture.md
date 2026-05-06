# Architecture

## Overview

The Mini Tensor Compiler is organized into several key modules that work together to provide a complete compilation and optimization pipeline for tensor computations.

## Core Modules

### Graph Module (`src/graph/`)

The heart of the compiler, responsible for:

- **Tensor Representation**: Defines tensors with shape, data type, and metadata
- **Operation Representation**: Defines supported operations (MatMul, ReLU, Add, Softmax, etc.)
- **Graph Structure**: Manages the computation graph, connections between operations

Key types:
- `Tensor`: Represents multi-dimensional arrays
- `Operation`: Represents computational operations
- `Graph`: Manages the overall computation graph

### Passes Module (`src/passes/`)

Implements optimization passes that transform the graph:

- **Fusion Pass**: Combines compatible operations (e.g., MatMul + Add + ReLU → FusedLinearReLU)
- **Constant Folding**: Pre-computes constant expressions
- **Dead Code Elimination**: Removes unused operations

All passes implement the `Pass` trait:
```rust
trait Pass {
    fn run(&self, graph: &mut Graph);
    fn name(&self) -> &'static str;
}
```

### Runtime Module (`src/runtime/`)

Handles execution of optimized graphs:

- **Executor**: Traverses the graph and executes operations
- **Kernels**: CPU implementations of operations using ndarray

### Optimizer Module (`src/optimizer/`)

Manages the optimization pipeline:

- **Pass Manager**: Coordinates execution of multiple passes
- **Optimizer**: High-level interface for running optimizations

## Data Flow

```
ONNX Model → Parser → Graph → Optimizer → Executor → Results
```

1. **Parsing**: ONNX models are parsed into internal graph representation
2. **Optimization**: Multiple passes transform the graph for better performance
3. **Execution**: Runtime executes the optimized graph

## Design Principles

- **Modularity**: Each module has clear responsibilities
- **Extensibility**: Easy to add new operations and optimization passes
- **Performance**: Focus on efficient graph representation and execution
- **Correctness**: Comprehensive testing and validation
