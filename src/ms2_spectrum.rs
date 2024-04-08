use pyo3::prelude::*;

use crate::precursor::Precursor;

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone)]
pub struct MS2Spectrum {
    pub identifier: String,
    pub mz: Vec<f32>,
    pub intensity: Vec<f32>,
    pub precursor: Option<Precursor>,
}

impl MS2Spectrum {
    pub fn new(
        identifier: String,
        mz: Vec<f32>,
        intensity: Vec<f32>,
        precursor: Option<Precursor>,
    ) -> Self {
        MS2Spectrum {
            identifier,
            mz,
            intensity,
            precursor,
        }
    }
}

#[pymethods]
impl MS2Spectrum {
    fn __repr__(&self) -> String {
        format!(
            "MS2Spectrum(identifier='{}', mz=[..], intensity=[..], precursor={:?})",
            self.identifier, self.precursor
        )
    }
}
