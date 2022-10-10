use pyo3::prelude::*;
use whatlang::{detect, detect_script};

// convert String to colored String with ANSI escape codes
pub enum TermColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

pub fn colorize(text: &str, color: TermColor) -> String {
    let color_code = match color {
        TermColor::Red => 31,
        TermColor::Green => 32,
        TermColor::Yellow => 33,
        TermColor::Blue => 34,
        TermColor::Magenta => 35,
        TermColor::Cyan => 36,
        TermColor::White => 37,
    };
    format!("\x1b[{}m{}\x1b[0m", color_code, text)
}

// Create a python enum class similare to whatlang::Lang
#[pyclass(name = "Info", dict)]
struct PyInfo {
    #[pyo3(get)]
    lang: String,
    #[pyo3(get)]
    script: String,
    #[pyo3(get, set)]
    confidence: f64,
    #[pyo3(get)]
    is_reliable: bool,
}

#[pyclass(name = "Script", dict)]
struct PyScript {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    langs: Vec<String>,
}

#[pymethods]
impl PyInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}: {} - {}: {} - {}: {} - {}: {}",
            colorize("Language", TermColor::Green),
            self.lang,
            colorize("Script", TermColor::Blue),
            self.script,
            colorize("Confidence", TermColor::Yellow),
            self.confidence,
            colorize("Is reliable", TermColor::Magenta),
            self.is_reliable
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "{}: {} - {}: {} - {}: {} - {}: {}",
            colorize("Language", TermColor::Green),
            self.lang,
            colorize("Script", TermColor::Green),
            self.script,
            colorize("Confidence", TermColor::Green),
            self.confidence,
            colorize("Is reliable", TermColor::Green),
            self.is_reliable
        ))
    }
}

#[pymethods]
impl PyScript {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}: {} - {}: {}",
            colorize("Name", TermColor::Green),
            self.name,
            colorize("Languages", TermColor::Blue),
            self.langs.join(", ")
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "{}: {} - {}: {}",
            colorize("Name", TermColor::Green),
            self.name,
            colorize("Languages", TermColor::Blue),
            self.langs.join(", ")
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

fn convert_to_py_script(script: whatlang::Script) -> PyScript {
    PyScript {
        name: script.name().to_string(),
        langs: script
            .langs()
            .iter()
            .map(|l| l.code().to_string())
            .collect(),
    }
}

#[pyfunction]
#[pyo3(name = "detect")]
#[pyo3(text_signature = "(text: str) -> Info")]
fn detect_lang(text: &str) -> PyResult<PyInfo> {
    let info = detect(text).unwrap();
    Ok(convert_to_py_info(info))
}

#[pyfunction]
#[pyo3(name = "detect_script")]
#[pyo3(text_signature = "(text: str) -> Info")]
fn detect_script_lang(text: &str) -> PyResult<PyScript> {
    let script = detect_script(text).unwrap();
    Ok(convert_to_py_script(script))
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "whatlang")]
fn whatlang_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyInfo>()?;
    m.add_function(wrap_pyfunction!(detect_lang, m)?)?;
    m.add_class::<PyScript>()?;
    m.add_function(wrap_pyfunction!(detect_script_lang, m)?)?;
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
