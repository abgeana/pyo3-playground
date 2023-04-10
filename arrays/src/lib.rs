use pyo3::prelude::*;

#[pyfunction]
fn rust_function(arg: &PyAny) -> PyResult<()> {
    let t = arg.get_type();
    match t.to_string().as_str() {
        "<class 'list'>" => {
            println!(
                "got a python list with length = {}",
                arg.call_method0("__len__").unwrap()
            );
        }
        "<class 'numpy.ndarray'>" => {
            println!(
                "got a numpy ndarray with shape[0] = {:?}",
                arg.getattr("shape").unwrap().get_item(0).unwrap()
            );
        }
        u => println!("argument type {} is unsupported", u),
    }
    Ok(())
}

#[pymodule]
fn rust_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_function, m)?)?;
    Ok(())
}
