extern crate pyo3;
extern crate blake3;
extern crate memmap2;

use pyo3::prelude::*;
use pyo3::types::PyString;

#[pyfunction]
fn blake3_hash(_py: Python, filename: Option<&str>) -> String {
    let hasher = || {
        if let Some(file) = filename {
            if let Ok(fs) = std::fs::File::open(file) {
                let mut hasher = blake3::Hasher::new();

                let mmap_file = unsafe { 
                    match memmap2::Mmap::map(&fs) {
                        Ok(mm) => mm,
                        Err(_) => todo!(),
                    }
                };

                let cursor = std::io::Cursor::new(mmap_file);
                hasher.update_rayon(cursor.get_ref());
                Some(hasher.finalize())
            } else { None }
        } else { None }
    };

    match hasher() {
        Some(hash) => {
            hash.to_string()
        },
        None => "".to_string()
    }

}

#[pyfunction]
fn hello(name: &str) -> PyResult<String> {
    Ok(format!("hello there {}...from Rust!", &name))
}

#[pyfunction]
fn edit_string(string: Py<PyString>) {
    let string: Py<PyString> = Py::clone(&string);

    println!("{:#?}", string);

}

#[pymodule]
fn rusty_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(edit_string, m)?)?;
    m.add_function(wrap_pyfunction!(blake3_hash, m)?)?;

    Ok(())
}
