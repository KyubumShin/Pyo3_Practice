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


unsafe fn image_to_mat(img: PyReadonlyArrayDyn<u8>) -> Mat {
    let shape = img.shape();
    let slice = img.as_slice().unwrap();
    let img_mat = Mat::from_slice(slice).unwrap();
    let img_mat = img_mat.reshape_nd(shape[2] as i32, &[shape[0] as i32, shape[1] as i32]).unwrap();
    img_mat
}

unsafe fn mat_to_image(py:Python, img: Mat) -> Py<PyArray<u8, Ix3>> {
    let (channels, rows, cols) = (img.channels(), img.rows(), img.cols());
    let pyarray = PyArray::from_slice(py, img.as_slice::<u8>().unwrap());
    let pyarray = pyarray.reshape((rows as usize, cols as usize, channels as usize)).unwrap();
    pyarray.to_owned()
}

#[pyfunction]
fn image_processing(img: PyReadonlyArrayDyn<u8>, bboxs: Vec<Vec<i32>>) -> PyResult<Vec<Py<PyArray<u8, Ix3>>>> {
    let img = unsafe {
        let img_mat = image_to_mat(img);
        img_mat
    };
    let mut croped_imges = Vec::new();

    Python::with_gil(|py| {
        for bbox in bboxs {
            let rect_box: Rect = Rect::new(bbox[0], bbox[1], bbox[2], bbox[3]);
            let croped_img = Mat::roi(&img, rect_box).unwrap();
            let croped_img = unsafe {
                    mat_to_image(py, croped_img.clone()) // not continious So cloned
            };
            croped_imges.push(croped_img)
        }
    });
    Ok(croped_imges)
}

#[pyfunction]
unsafe fn image_load(path: String) -> PyResult<Py<PyArray<u8, Ix3>>> {
    let mat: Mat = imgcodecs::imread(&path, imgcodecs::IMREAD_COLOR).expect("Failed to Read");
    let (channels, rows, cols) = (mat.channels(), mat.rows(), mat.cols());

    Python::with_gil(|py| {
        let pyarray = PyArray::from_slice(py, mat.as_slice::<u8>().unwrap()); // mat -> pyarrray 1-D array
        let pyarray = pyarray.cast::<u8>(false).unwrap(); // pyarray type 지정
        let pyarray = pyarray.reshape((rows as usize, cols as usize, channels as usize))?; // reshape를 통한 이미지 변환
        Ok(pyarray.to_owned())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_numpy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(image_load, m)?)?;
    Ok(())
}