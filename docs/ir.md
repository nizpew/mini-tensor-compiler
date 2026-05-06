# Intermediate Representation (IR)

## Overview

The Mini Tensor Compiler uses a graph-based intermediate representation to model tensor computations. This IR is designed to be:

- Simple and easy to understand
- Extensible for new operations
- Suitable for optimization passes
- Efficient for runtime execution

## Core Concepts

### Tensors

Tensors are the fundamental data units in our IR:

```rust
pub struct Tensor {
    pub name: String,
    pub shape: Vec<usize>,
    pub dtype: DataType,
}
```

- **Name**: Unique identifier for the tensor
- **Shape**: Multi-dimensional array dimensions
- **Data Type**: Supported types (Float32, Float64, Int32, Int64, Bool)

### Operations

Operations represent computational transformations:

```rust
pub enum OpType {
    MatMul,
    ReLU,
    Add,
    Softmax,
    FusedLinearReLU,
}
```

Each operation has:
- **Name**: Unique identifier
- **Type**: The specific operation
- **Inputs**: List of input tensor names
- **Outputs**: List of output tensor names
- **Attributes**: Operation-specific parameters

### Graph

The graph connects operations in a computation flow:

```rust
pub struct Graph {
    operations: Vec<Operation>,
    tensors: HashMap<String, Tensor>,
    operation_counter: usize,
}
```

## Graph Construction

Operations are added sequentially and automatically connected:

```rust
let mut graph = Graph::new();
graph.add_op("MatMul");  // Creates MatMul operation
graph.add_op("Add");     // Connected to MatMul output
graph.add_op("ReLU");    // Connected to Add output
```

This creates the graph: `MatMul -> Add -> ReLU`

## Optimization-Friendly Design

The IR is designed to make optimizations easy:

1. **Explicit Dependencies**: Input/output relationships are clear
2. **Operation Attributes**: Metadata for optimization decisions
3. **Mutable Structure**: Easy to transform and rewrite
4. **Type Safety**: Compile-time guarantees about operation types

## Example Graph

Consider this computation:
```rust
let mut graph = Graph::new();
graph.add_op("MatMul");
graph.add_op("Add");
graph.add_op("ReLU");
```

Results in:
```
Operations:
- matmul_0: MatMul
  Inputs: []
  Outputs: ["matmul_0_output"]
- add_1: Add
  Inputs: ["matmul_0_output"]
  Outputs: ["add_1_output"]
- relu_2: ReLU
  Inputs: ["add_1_output"]
  Outputs: ["relu_2_output"]
```

After fusion optimization:
```
Operations:
- fused_linear_relu_0: FusedLinearReLU
  Inputs: []
  Outputs: ["relu_2_output"]
```
