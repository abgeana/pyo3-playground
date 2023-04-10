use numpy::PyArray1;
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
            let dtype = arg.getattr("dtype").unwrap();
            println!("got a numpy array of dtype {}", dtype);
            let numpy_array = match dtype.to_string().as_str() {
                "int8" => arg.downcast::<PyArray1<i8>>().unwrap(),
                _ => panic!("unknown dtype :("),
            };
            println!("numpy array has shape {:?}", numpy_array.shape());
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
