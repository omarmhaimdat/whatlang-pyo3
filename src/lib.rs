use pyo3::prelude::*;
use rayon::prelude::*;
use whatlang::{detect, detect_lang, detect_script};

use crate::utils::{colorize, get_progress_bar, lang_to_iso639_1, TermColor};

mod utils;

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

#[pyclass(name = "Lang")]
struct PyLang {
    #[pyo3(get)]
    lang: String,
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
            "Language: {} - Script: {} - Confidence: {} - Is reliable: {}",
            self.lang, self.script, self.confidence, self.is_reliable
        ))
    }

    /// Convert Language Code to ISO 639-1 code,
    /// e.g. "en" for English, this method changes the language code
    /// of the current object.
    /// https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
    /// # Example: "eng" -> "en"
    fn to_iso(&mut self) {
        self.lang = lang_to_iso639_1(self.lang.as_str()).to_string();
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
            "Name: {} - Languages: {}",
            self.name,
            self.langs.join(", ")
        ))
    }
}

#[pymethods]
impl PyLang {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}: {}",
            colorize("Language", TermColor::Green),
            self.lang
        ))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Language: {}", self.lang))
    }

    /// Convert Language Code to ISO 639-1 code,
    /// e.g. "en" for English, this method changes the language code
    /// of the current object.
    /// https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
    /// # Example: "eng" -> "en"
    fn to_iso(&mut self) {
        self.lang = lang_to_iso639_1(self.lang.as_str()).to_string();
    }
}

fn convert_to_py_info(info: whatlang::Info) -> PyInfo {
    PyInfo {
        lang: info.lang().code().to_string(),
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

fn convert_to_py_lang(lang: whatlang::Lang) -> PyLang {
    PyLang {
        lang: lang.code().to_string(),
    }
}

fn batch_detect(texts: Vec<&str>, n_jobs: i16) -> Vec<PyInfo> {
    // Get number of cores
    let mut n_cores: usize = num_cpus::get();
    if n_jobs > 0 && n_jobs < n_cores as i16 {
        n_cores = n_jobs as usize;
    }
    let message = format!(
        "{} texts using {} cores",
        texts.len(),
        format!("{}", colorize(&n_cores.to_string(), TermColor::Green))
    );
    let pb = get_progress_bar(texts.len() as u64, message.to_string());
    let mut results = Vec::new();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(n_cores)
        .build()
        .unwrap();
    pool.install(|| {
        results = texts
            .into_par_iter()
            .map(|text| {
                let info = detect(text).unwrap();
                pb.inc(1);
                convert_to_py_info(info)
            })
            .collect();
    });
    results

    // if parallel {
    //     texts
    //         .into_par_iter()
    //         .map(|text| convert_to_py_info(detect(text).unwrap()))
    //         .collect()
    // } else {
    //     texts
    //         .into_iter()
    //         .map(|text| convert_to_py_info(detect(text).unwrap()))
    //         .collect()
    // }
}

#[pyfunction]
#[pyo3(name = "detect")]
#[pyo3(text_signature = "(text: str) -> Info")]
fn py_detect(text: &str) -> PyResult<PyInfo> {
    let info = detect(text).unwrap();
    Ok(convert_to_py_info(info))
}

#[pyfunction]
#[pyo3(name = "detect_script")]
#[pyo3(text_signature = "(text: str) -> Info")]
fn py_detect_script(text: &str) -> PyResult<PyScript> {
    let script = detect_script(text).unwrap();
    Ok(convert_to_py_script(script))
}

#[pyfunction]
#[pyo3(name = "detect_lang")]
#[pyo3(text_signature = "(text: str) -> Lang")]
fn py_detect_lang(text: &str) -> PyResult<PyLang> {
    let lang = detect_lang(text).unwrap();
    Ok(convert_to_py_lang(lang))
}

#[pyfunction]
#[pyo3(name = "batch_detect")]
#[pyo3(text_signature = "(texts, n_jobs = -1)")]
fn py_batch_detect(texts: Vec<&str>, n_jobs: Option<i16>) -> PyResult<Vec<PyInfo>> {
    let n_jobs = n_jobs.unwrap_or(-1);
    Ok(batch_detect(texts, n_jobs))
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "whatlang")]
fn whatlang_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyInfo>()?;
    m.add_function(wrap_pyfunction!(py_detect, m)?)?;
    m.add_class::<PyScript>()?;
    m.add_function(wrap_pyfunction!(py_detect_script, m)?)?;
    m.add_class::<PyLang>()?;
    m.add_function(wrap_pyfunction!(py_detect_lang, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_detect, m)?)?;
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
