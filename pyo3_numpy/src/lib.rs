use std::u8;

use pyo3::prelude::*;
use numpy::{PyArray, Ix3};
use numpy::pyo3::Python;
use opencv::prelude::*;
use opencv::imgcodecs;

// pub use convert::*;
// mod convert;
mod with_opencv;
pub use with_opencv::*;


#[pyfunction]
unsafe fn image_load(path: String) -> PyResult<Py<PyArray<u8, Ix3>>> {
    let mat: Mat = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR).expect("Failed to Read");
    let (channels, rows, cols) = (mat.channels(), mat.rows(), mat.cols());

    Python::with_gil(|py| {
        let pyarray = PyArray::from_slice(py, mat.as_slice::<u8>().unwrap()); // mat -> pyarrray 1-D array
        let pyarray = pyarray.cast::<u8>(false).unwrap(); // pyarray type 지정
        let array = pyarray.reshape((rows as usize, cols as usize, channels as usize))?; // reshape를 통한 이미지 변환
        Ok(array.to_owned())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_numpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(image_load, m)?)?;
    Ok(())
}