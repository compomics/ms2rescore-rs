use std::collections::HashMap;

use mzdata::params::ParamValue;
use mzdata::mz_read;

use crate::ms2_spectrum::MS2Spectrum;
use crate::precursor::Precursor;

impl From<&mzdata::spectrum::MultiLayerSpectrum> for Precursor {
    fn from(spectrum: &mzdata::spectrum::MultiLayerSpectrum) -> Self {
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
                im: get_im_from_spectrum_description(spectrum)
                    .or(get_im_from_selected_ion(spectrum))
                    .or(get_im_from_first_scan(spectrum))
                    .unwrap_or(0.0),
                charge: get_charge_from_spectrum(spectrum).unwrap_or(0),
                intensity: precursor.ions[0].intensity as f64,
            },
            None => Precursor::default(),
        }
    }
}

impl From<mzdata::spectrum::MultiLayerSpectrum> for MS2Spectrum {
    fn from(spectrum: mzdata::spectrum::MultiLayerSpectrum) -> Self {
        let identifier: String = spectrum.description.id.to_string();
        let precursor = Precursor::from(&spectrum);
        let mzdata_centroid_spectrum = spectrum.into_centroid().unwrap();
        let (mz, intensity): (Vec<f32>, Vec<f32>) = mzdata_centroid_spectrum
            .peaks
            .iter()
            .map(|peak| (peak.mz as f32, peak.intensity))
            .unzip();

        MS2Spectrum::new(identifier, mz, intensity, Some(precursor))
    }
}

/// Parse precursor info from spectrum files with mzdata
pub fn parse_precursor_info(
    spectrum_path: &str,
) -> Result<HashMap<String, Precursor>, std::io::Error> {
    mz_read!(spectrum_path.as_ref(), reader => {
        reader.filter(|spectrum| spectrum.description.ms_level == 2)
            .filter_map(|spectrum| {
                spectrum.description.precursor.as_ref()?;
                Some((spectrum.description.id.clone(), Precursor::from(&spectrum)))
            })
            .collect::<HashMap<String, Precursor>>()
    })
}

/// Read MS2 spectra from spectrum files with mzdata
pub fn read_ms2_spectra(
    spectrum_path: &str,
) -> Result<Vec<MS2Spectrum>, std::io::Error> {
    mz_read!(spectrum_path.as_ref(), reader => {
        reader.filter(|spectrum| spectrum.description.ms_level == 2)
            .map(MS2Spectrum::from)
            .collect::<Vec<MS2Spectrum>>()
    })
}

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
                .map(|p| p.value.to_string())
                .and_then(|v| {
                    v.strip_suffix('+')
                        .unwrap_or(v.as_str())
                        .parse::<usize>()
                        .ok()
                })
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
        .and_then(|p| p.value.to_f64().ok())
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
                .and_then(|p| p.value.to_f64().ok())
        })
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
                .and_then(|p| p.value.to_f64().ok())
        })
}
