pub enum SpectrumFileType {
    MascotGenericFormat,
    MzML,
    BrukerRaw,
    // ThermoRaw,
    Unknown,
}

pub fn match_file_type(spectrum_path: &str) -> SpectrumFileType {
    let extension = spectrum_path.split('.').last().unwrap_or("").to_lowercase();
    match extension.as_str() {
        "mgf" => SpectrumFileType::MascotGenericFormat,
        "mzml" => SpectrumFileType::MzML,
        "d" | "ms2" => SpectrumFileType::BrukerRaw,
        // "raw" => SpectrumFileType::ThermoRaw,
        _ => match (
            folder_contains_extension(spectrum_path, "bin"),
            folder_contains_extension(spectrum_path, "parquet"),
        ) {
            (true, true) => SpectrumFileType::BrukerRaw,
            _ => SpectrumFileType::Unknown,
        },
    }
}

fn folder_contains_extension(input: impl AsRef<std::path::Path>, extension: &str) -> bool {
    let folder_path: std::path::PathBuf = input.as_ref().to_path_buf();
    if !folder_path.is_dir() {
        return false;
    }
    if let Ok(entries) = std::fs::read_dir(folder_path) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == extension {
                    return true;
                }
            }
        }
    }
    false
}
