mod file_types;
mod parse_mzdata;
mod parse_timsrust;
mod precursor;
mod ms2_spectrum;

use std::collections::HashMap;

use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;

use file_types::{match_file_type, SpectrumFileType};
use precursor::Precursor;
use ms2_spectrum::MS2Spectrum;

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

/// Get MS2 spectra from a spectrum file.
#[pyfunction]
pub fn get_ms2_spectra(spectrum_path: String) -> PyResult<Vec<ms2_spectrum::MS2Spectrum>> {
    let file_type = match_file_type(&spectrum_path);

    let spectra = match file_type {
        SpectrumFileType::MascotGenericFormat | SpectrumFileType::MzML => {
            parse_mzdata::read_ms2_spectra(&spectrum_path, file_type)
        }
        SpectrumFileType::BrukerRaw => parse_timsrust::read_ms2_spectra(&spectrum_path),
        // SpectrumFileType::ThermoRaw => parse_with_mzdata_thermo(&spectrum_path, file_type),
        SpectrumFileType::Unknown => return Err(PyOSError::new_err("Unsupported file type")),
    };

    match spectra {
        Ok(spectra) => Ok(spectra),
        Err(e) => Err(PyOSError::new_err(e.to_string())),
    }
}


/// A Python module implemented in Rust.
#[pymodule]
fn ms2rescore_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Precursor>()?;
    m.add_class::<MS2Spectrum>()?;
    m.add_function(wrap_pyfunction!(get_precursor_info, m)?)?;
    m.add_function(wrap_pyfunction!(get_ms2_spectra, m)?)?;
    Ok(())
}
