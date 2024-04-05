use std::collections::HashMap;

use crate::precursor::Precursor;

impl From<timsrust::Precursor> for Precursor {
    fn from(precursor: timsrust::Precursor) -> Self {
        Precursor {
            mz: precursor.mz,
            rt: precursor.rt,
            im: precursor.im,
            charge: precursor.charge,
            intensity: precursor.intensity,
        }
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
                timsrust::QuadrupoleEvent::Precursor { .. }
            )
        })
        .map(|spectrum| {
            (
                spectrum.index.to_string(),
                Precursor::from(spectrum.precursor.unwrap_as_precursor()),
            )
        })
        .collect::<HashMap<String, Precursor>>())
}
