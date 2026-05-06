use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mini_tensor_compiler::graph::Graph;

fn bench_graph_creation(c: &mut Criterion) {
    c.bench_function("create_small_graph", |b| {
        b.iter(|| {
            let mut graph = Graph::new();
            graph.add_op(black_box("MatMul"));
            graph.add_op(black_box("Add"));
            graph.add_op(black_box("ReLU"));
            graph
        })
    });
    
    c.bench_function("create_large_graph", |b| {
        b.iter(|| {
            let mut graph = Graph::new();
            for _ in 0..100 {
                graph.add_op(black_box("MatMul"));
                graph.add_op(black_box("ReLU"));
            }
            graph
        })
    });
}

fn bench_graph_operations(c: &mut Criterion) {
    let mut graph = Graph::new();
    for _ in 0..50 {
        graph.add_op("MatMul");
        graph.add_op("ReLU");
    }
    
    c.bench_function("graph_printing", |b| {
        b.iter(|| {
            graph.print();
        })
    });
}

criterion_group!(benches, bench_graph_creation, bench_graph_operations);
criterion_main!(benches);
