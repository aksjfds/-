use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn zero_judge(vecs: Vec<Vec<f64>>) -> PyResult<Vec<usize>> {
    // 传入一张28x28的图

    let mut index_to_delete = Vec::new();

    for (index, row) in vecs.iter().enumerate() {


        for j in row.iter() {
            if j != &0.0 {
                index_to_delete.push(index);
                break;
            }
        }

    }

    return Ok(index_to_delete);
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(zero_judge, m)?)?;
    Ok(())
}

// maturin develop
