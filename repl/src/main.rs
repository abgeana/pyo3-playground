use pyo3::{pymodule, types::PyModule, PyResult, Python};
use std::io::{self, Write};

#[pymodule]
fn rust_module(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    fn rust_function(_py: Python) {
        println!("hello from rust!");
    }
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    pyo3::append_to_inittab!(rust_module);

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    loop {
        print!(">>> ");
        stdout.flush()?;

        input.clear();
        if stdin.read_line(&mut input).is_ok() {
            let statement = input.replace('\n', "");
            Python::with_gil(|py| {
                let result = py.run(&statement, None, None);
                if let Err(e) = result {
                    println!("{}", e);
                }
            });
        }
    }
}
