use ndarray::Array2;

pub fn matmul(a: &Array2<f32>, b: &Array2<f32>) -> Array2<f32> {
    a.dot(b)
}

pub fn relu(x: &Array2<f32>) -> Array2<f32> {
    x.mapv(|v| v.max(0.0))
}

pub fn add(a: &Array2<f32>, b: &Array2<f32>) -> Array2<f32> {
    a + b
}

pub fn fused_linear_relu(x: &Array2<f32>, weight: &Array2<f32>, bias: &Array2<f32>) -> Array2<f32> {
    let linear = x.dot(weight) + bias;
    relu(&linear)
}
