#!/bin/bash

echo "🚀 Running Mini Tensor Compiler Benchmarks"
echo "=========================================="

# Run cargo benchmarks
echo "📊 Running Criterion benchmarks..."
cargo bench

# Run examples with timing
echo ""
echo "⏱️  Timing examples..."
echo "Running simple_graph:"
time cargo run --example simple_graph --release

echo ""
echo "Running fusion_demo:"
time cargo run --example fusion_demo --release

echo ""
echo "✅ Benchmarking completed!"
