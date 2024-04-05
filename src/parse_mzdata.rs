use std::collections::HashMap;
use std::fs::File;

use mzdata::io::{MGFReader, MzMLReader};

use crate::file_types::SpectrumFileType;
use crate::precursor::Precursor;

impl From<mzdata::spectrum::MultiLayerSpectrum> for Precursor {
    fn from(spectrum: mzdata::spectrum::MultiLayerSpectrum) -> Self {
        let precursor = &spectrum.description.precursor;
        match precursor {
            Some(precursor) => Precursor {
                mz: precursor.ions[0].mz,
                rt: spectrum
                    .description
                    .acquisition
                    .first_scan()
                    .map(|s| s.start_time)
                    .unwrap_or(0.0),
                im: get_im_from_spectrum_description(&spectrum)
                    .or(get_im_from_selected_ion(&spectrum))
                    .or(get_im_from_first_scan(&spectrum))
                    .unwrap_or(0.0),
                charge: get_charge_from_spectrum(&spectrum).unwrap_or(0),
                intensity: precursor.ions[0].intensity as f64,
            },
            None => Precursor::default(),
        }
    }
}

/// Parse precursor info from spectrum files with mzdata
pub fn parse_precursor_info(
    spectrum_path: &str,
    file_type: SpectrumFileType,
) -> Result<HashMap<String, Precursor>, std::io::Error> {
    let file = File::open(spectrum_path)?;
    match file_type {
        SpectrumFileType::MascotGenericFormat => Ok(MGFReader::new(file)
            .filter_map(|spectrum| {
                spectrum.description.precursor.as_ref()?;
                Some((spectrum.description.id.clone(), Precursor::from(spectrum)))
            })
            .collect::<HashMap<String, Precursor>>()),

        SpectrumFileType::MzML => Ok(MzMLReader::new(file)
            .filter_map(|spectrum| {
                if spectrum.description.ms_level != 2 {
                    return None;
                }
                spectrum.description.precursor.as_ref()?;
                Some((spectrum.description.id.clone(), Precursor::from(spectrum)))
            })
            .collect::<HashMap<String, Precursor>>()),

        _ => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unsupported file type for mzdata",
        )),
    }
}

// pub fn parse_precursor_info_thermo(
//     spectrum_path: &str,
//     file_type: SpectrumFileType,
// ) -> Result<HashMap<String, Precursor>, std::io::Error> {
//     let reader = mzdata::io::ThermoRawReader::open_path(spectrum_path)?;
//     Ok(reader
//         .into_iter()
//         .filter(|spectrum| {
//             (spectrum.description.ms_level == 2) && (spectrum.description.precursor.is_some())
//         })
//         .map(|spectrum| (spectrum.description.id, Precursor::from(spectrum)))
//         .collect::<HashMap<String, Precursor>>())
// }

fn get_charge_from_spectrum(spectrum: &mzdata::spectrum::MultiLayerSpectrum) -> Option<usize> {
    spectrum
        .description
        .precursor
        .as_ref()
        .and_then(|p| p.ions.first())
        .and_then(|i| i.charge.map(|c| c as usize))
        .or_else(|| {
            spectrum
                .description
                .params
                .iter()
                .find(|p| p.name == "charge")
                .map(|p| p.value.strip_suffix('+').unwrap_or(&p.value))
                .map(|v| v.parse::<usize>().unwrap_or(0))
        })
}

/// Try to get ion mobility from the spectrum description parameters.
fn get_im_from_spectrum_description(
    spectrum: &mzdata::spectrum::MultiLayerSpectrum,
) -> Option<f64> {
    spectrum
        .description
        .params
        .iter()
        .find(|p| {
            (p.name == "ion_mobility")
                || (p.name == "inverse reduced ion mobility")
                || (p.name == "reverse ion mobility")
        })
        .map(|p| p.value.clone())
        .map(|v| match v.parse::<f64>() {
            Ok(v) => Some(v),
            Err(_) => None,
        })?
}

/// Try to get ion mobility from the selected ion parameters.
fn get_im_from_selected_ion(spectrum: &mzdata::spectrum::MultiLayerSpectrum) -> Option<f64> {
    spectrum
        .description
        .precursor
        .as_ref()
        .and_then(|p| p.ions.first())
        .and_then(|i| i.params.as_ref())
        .and_then(|p| {
            p.iter()
                .find(|p| {
                    (p.name == "ion_mobility")
                        || (p.name == "inverse reduced ion mobility")
                        || (p.name == "reverse ion mobility")
                })
                .map(|p| p.value.clone())
                .map(|v| match v.parse::<f64>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                })
        })?
}

/// Try to get ion mobility from the first scan parameters.
fn get_im_from_first_scan(spectrum: &mzdata::spectrum::MultiLayerSpectrum) -> Option<f64> {
    spectrum
        .description
        .acquisition
        .first_scan()
        .and_then(|s| s.params.as_ref())
        .and_then(|p| {
            p.iter()
                .find(|p| {
                    (p.name == "ion_mobility")
                        || (p.name == "inverse reduced ion mobility")
                        || (p.name == "reverse ion mobility")
                })
                .map(|p| p.value.clone())
                .map(|v| match v.parse::<f64>() {
                    Ok(v) => Some(v),
                    Err(_) => None,
                })
        })?
}
