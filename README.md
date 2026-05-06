# Mini Tensor Compiler 

A lightweight tensor compiler and optimization framework written in Rust.


<img width="1110" height="632" alt="image" src="https://github.com/user-attachments/assets/19b92f2c-8e44-466c-8889-5b1cb6306e37" />



## Features

- **Graph Representation**: Define computation graphs with tensors and operations
- **Optimization Passes**: Fusion, constant folding, and dead code elimination
- **Runtime Execution**: CPU kernels for common operations
- **ONNX Support**: Import and parse ONNX models (planned)
- **Benchmarking**: Performance profiling with Criterion

## Quick Start

```bash
# Run the main example
cargo run

# Run specific examples
cargo run --example simple_graph
cargo run --example fusion_demo

# Run tests
cargo test

# Run benchmarks
cargo bench
```

## Example Usage

```rust
use mini_tensor_compiler::graph::Graph;

let mut graph = Graph::new();

graph.add_op("MatMul");
graph.add_op("Add");
graph.add_op("ReLU");

graph.print();
// Output: MatMul -> Add -> ReLU
```

## Architecture

- `src/graph/`: Core graph representation (tensors, operations, graph)
- `src/passes/`: Optimization passes (fusion, constant folding, dead code)
- `src/runtime/`: Execution engine and CPU kernels
- `src/optimizer/`: Optimization pipeline manager
- `src/parser/`: ONNX model parsing
- `src/benchmark/`: Performance profiling tools

## Project Structure

```
mini_tensor_compiler/
├── src/
│   ├── graph/          # Core graph representation
│   ├── passes/         # Optimization passes
│   ├── runtime/        # Execution engine
│   ├── optimizer/      # Optimization pipeline
│   ├── parser/         # ONNX parsing
│   ├── benchmark/      # Profiling tools
│   └── utils/          # Utilities
├── examples/           # Usage examples
├── tests/              # Unit tests
├── benches/            # Performance benchmarks
└── docs/               # Documentation
```

## License

MIT
