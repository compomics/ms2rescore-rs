mod file_types;
mod parse_mzdata;
mod parse_timsrust;
mod precursor;

use std::collections::HashMap;

use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;

use file_types::{match_file_type, SpectrumFileType};
use precursor::Precursor;

/// Get mapping of spectrum identifiers to precursor information.
#[pyfunction]
pub fn get_precursor_info(spectrum_path: String) -> PyResult<HashMap<String, Precursor>> {
    let file_type = match_file_type(&spectrum_path);

    let precursors = match file_type {
        SpectrumFileType::MascotGenericFormat | SpectrumFileType::MzML => {
            parse_mzdata::parse_precursor_info(&spectrum_path, file_type)
        }
        SpectrumFileType::BrukerRaw => parse_timsrust::parse_precursor_info(&spectrum_path),
        // SpectrumFileType::ThermoRaw => parse_with_mzdata_thermo(&spectrum_path, file_type),
        SpectrumFileType::Unknown => return Err(PyOSError::new_err("Unsupported file type")),
    };

    match precursors {
        Ok(precursors) => Ok(precursors),
        Err(e) => Err(PyOSError::new_err(e.to_string())),
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn ms2rescore_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Precursor>()?;
    m.add_function(wrap_pyfunction!(get_precursor_info, m)?)?;
    Ok(())
}
