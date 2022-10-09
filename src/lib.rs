use pyo3::prelude::*;
use whatlang::detect;

// Create a python enum class similare to whatlang::Lang
#[pyclass(name = "Info")]
struct PyInfo {
    #[pyo3(get)]
    lang: String,
    #[pyo3(get)]
    script: String,
    #[pyo3(get)]
    confidence: f64,
    #[pyo3(get)]
    is_reliable: bool,
}

#[pymethods]
impl PyInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "Language: {} Script: {} Confidence: {} Is reliable: {}",
            self.lang, self.script, self.confidence, self.is_reliable
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Language: {} Script: {} Confidence: {} Is reliable: {}",
            self.lang, self.script, self.confidence, self.is_reliable
        ))
    }
}

fn lang_to_iso639_1(lang: &str) -> String {
    // convert whatlang::Lang code to iso639-1 code
    match lang {
        "afr" => "af",
        "ara" => "ar",
        "bul" => "bg",
        "bos" => "bs",
        "cat" => "ca",
        "ces" => "cs",
        "dan" => "da",
        "deu" => "de",
        "ell" => "el",
        "eng" => "en",
        "epo" => "eo",
        "est" => "et",
        "eus" => "eu",
        "fin" => "fi",
        "fra" => "fr",
        "glg" => "gl",
        "heb" => "he",
        "hin" => "hi",
        "hrv" => "hr",
        "hun" => "hu",
        "ind" => "id",
        "ile" => "is",
        "ita" => "it",
        "jpn" => "ja",
        "kor" => "ko",
        "lat" => "la",
        "lit" => "lt",
        "lav" => "lv",
        "mkd" => "mk",
        "mlt" => "mt",
        "nld" => "nl",
        "nob" => "nb",
        "nno" => "nn",
        "pol" => "pl",
        "por" => "pt",
        "rum" => "ro",
        "rus" => "ru",
        "slk" => "sk",
        "slv" => "sl",
        "spa" => "es",
        "swe" => "sv",
        "tel" => "te",
        "tur" => "tr",
        "ukr" => "uk",
        "vie" => "vi",
        _ => "unknown",
    }
    .to_string()
}

fn convert_to_py_info(info: whatlang::Info) -> PyInfo {
    PyInfo {
        lang: lang_to_iso639_1(info.lang().code().to_string().as_str()),
        script: info.script().to_string(),
        confidence: info.confidence(),
        is_reliable: info.is_reliable(),
    }
}

#[pyfunction]
#[pyo3(name = "detect")]
#[pyo3(text_signature = "(text: str) -> Info")]
fn detect_lang(text: &str) -> PyResult<PyInfo> {
    let info = detect(text).unwrap();
    Ok(convert_to_py_info(info))
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "whatlang")]
fn whatlang_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyInfo>()?;
    m.add_function(wrap_pyfunction!(detect_lang, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use whatlang::{detect, Lang, Script};
    #[test]
    fn test_detect() {
        let text = "This is written in English";
        let info = detect(text).unwrap();
        assert_eq!(info.lang(), Lang::Eng);
    }

    #[test]
    fn test_script() {
        let text = "This is written in English";
        let info = detect(text).unwrap();
        assert_eq!(info.script(), Script::Latin);
    }

    #[test]
    fn test_is_reliable() {
        let text = "This is written in English";
        let info = detect(text).unwrap();
        assert_eq!(info.is_reliable(), true);
    }
}
