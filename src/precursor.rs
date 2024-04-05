use pyo3::prelude::*;

/// Precursor information.
#[pyclass(get_all, set_all)]
#[derive(Debug)]
pub struct Precursor {
    pub mz: f64,
    pub rt: f64,
    pub im: f64,
    pub charge: usize,
    pub intensity: f64,
}

#[pymethods]
impl Precursor {
    pub fn __repr__(&self) -> String {
        format!(
            "Precursor(mz={}, rt={}, im={}, charge={}, intensity={})",
            self.mz, self.rt, self.im, self.charge, self.intensity
        )
    }
}

impl Default for Precursor {
    fn default() -> Self {
        Precursor {
            mz: 0.0,
            rt: 0.0,
            im: 0.0,
            charge: 0,
            intensity: 0.0,
        }
    }
}
