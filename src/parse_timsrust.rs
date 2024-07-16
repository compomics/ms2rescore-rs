use std::collections::HashMap;

use crate::ms2_spectrum::MS2Spectrum;
use crate::precursor::Precursor;

impl From<timsrust::ms_data::Precursor> for Precursor {
    fn from(precursor: timsrust::ms_data::Precursor) -> Self {
        Precursor {
            mz: precursor.mz,
            rt: precursor.rt,
            im: precursor.im,
            charge: precursor.charge.unwrap_or(0),
            intensity: precursor.intensity.unwrap_or(0.0),
        }
    }
}

impl From<timsrust::ms_data::Spectrum> for MS2Spectrum {
    fn from(spectrum: timsrust::ms_data::Spectrum) -> Self {
        MS2Spectrum::new(
            spectrum.index.to_string(),
            spectrum.mz_values.iter().map(|mz| *mz as f32).collect(),
            spectrum
                .intensities
                .iter()
                .map(|intensity| *intensity as f32)
                .collect(),
            Some(Precursor::from(spectrum.precursor)),
        )
    }
}

/// Parse precursor info from spectrum files with timsrust
pub fn parse_precursor_info(
    spectrum_path: &str,
) -> Result<HashMap<String, Precursor>, std::io::Error> {
    let reader = timsrust::FileReader::new(spectrum_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(reader
        .read_all_spectra()
        .into_iter()
        .filter(|spectrum| {
            matches!(
                spectrum.precursor,
                timsrust::ms_data::Precursor { .. }
            )
        })
        .map(|spectrum| {
            (
                spectrum.index.to_string(),
                Precursor::from(spectrum.precursor),
            )
        })
        .collect::<HashMap<String, Precursor>>())
}

/// Read MS2 spectra from spectrum files with timsrust
pub fn read_ms2_spectra(spectrum_path: &str) -> Result<Vec<MS2Spectrum>, std::io::Error> {
    let reader = timsrust::FileReader::new(spectrum_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    Ok(reader
        .read_all_spectra()
        .into_iter()
        .map(MS2Spectrum::from)
        .collect())
}
