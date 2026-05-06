use mini_tensor_compiler::runtime::Executor;
use mini_tensor_compiler::runtime::kernels;
use ndarray::arr2;

#[test]
fn test_kernels() {
    let a = arr2(&[[1.0, 2.0], [3.0, 4.0]]);
    let b = arr2(&[[5.0, 6.0], [7.0, 8.0]]);
    
    let result = kernels::matmul(&a, &b);
    assert_eq!(result.shape(), &[2, 2]);
    
    let relu_result = kernels::relu(&a);
    assert_eq!(relu_result[[0, 0]], 1.0);
    assert_eq!(relu_result[[0, 1]], 2.0);
    
    let add_result = kernels::add(&a, &b);
    assert_eq!(add_result[[0, 0]], 6.0);
    assert_eq!(add_result[[0, 1]], 8.0);
}

#[test]
fn test_executor_creation() {
    let _executor = Executor::new();
    // Just test that executor can be created
}
