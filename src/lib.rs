use pyo3::prelude::*;
use rayon::prelude::*;
use whatlang::{detect, detect_lang, detect_script};

use crate::utils::{colorize, get_progress_bar, lang_to_iso639_1, TermColor};

mod tests;
mod utils;


#[pyclass(name = "Script", dict)]
struct PyScript {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    langs: Vec<String>,
}

#[derive(Debug, Clone)]
#[pyclass(name = "Lang")]
struct PyLang {
    #[pyo3(get)]
    lang: String,
}

// Create a python enum class similare to whatlang::Lang
#[derive(Clone, Debug)]
#[pyclass(name = "Info", dict)]
struct PyInfo {
    #[pyo3(get)]
    __lang: PyLang,
    #[pyo3(get)]
    script: String,
    #[pyo3(get, set)]
    confidence: f64,
    #[pyo3(get)]
    is_reliable: bool,
}

#[pymethods]
impl PyInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}: {} - {}: {} - {}: {} - {}: {}",
            colorize("Language", TermColor::Green),
            self.__lang.lang,
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
            self.__lang.lang, self.script, self.confidence, self.is_reliable
        ))
    }

    #[getter]
    fn lang(&self) -> PyResult<String> {
        Ok(self.__lang.lang.clone())
    }

    /// Convert Language Code to ISO 639-1 code,
    /// e.g. "en" for English, this method changes the language code
    /// of the current object.
    /// https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
    /// # Example: "eng" -> "en"
    fn to_iso(&mut self) {
        // Check if the language code is already ISO 639-1
        if self.__lang.lang.len() == 2 {
            return;
        }
        self.__lang.lang = lang_to_iso639_1(self.__lang.lang.as_str()).to_string();
    }
}

#[pymethods]
impl PyScript {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!(
            "{}: {} - {}: {}",
            colorize("Name", TermColor::Green),
            self.name,
            colorize("Languages", TermColor::Cyan),
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
        Ok(format!("{}", self.lang))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Language: {}", self.lang))
    }

    #[getter]
    fn iso(&self) -> PyResult<String> {
        Ok(lang_to_iso639_1(self.lang.as_str()).to_string())
    }

    /// Convert Language Code to ISO 639-1 code,
    /// e.g. "en" for English, this method changes the language code
    /// of the current object.
    /// https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
    /// # Example: "eng" -> "en"
    fn to_iso(&mut self) {
        // Check if the language code is already ISO 639-1
        if self.lang.len() == 2 {
            return;
        }
        self.lang = lang_to_iso639_1(self.lang.as_str()).to_string();
    }
}

fn convert_to_py_info(info: whatlang::Info) -> PyInfo {
    PyInfo {
        __lang: PyLang {
            lang: info.lang().code().to_string(),
        },
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
        lang: lang.code().to_string()
    }
}

/// Detect language of a list of texts
/// # Arguments
/// * `texts` - A list of texts
/// * `n_jobs` - Number of cores to use, if <= 0 use all cores
/// # Example
/// ```python
/// >>> from whatlang import batch_detect
/// >>> texts = ["Hello world", "Bonjour le monde"]
/// >>> batch_detect(texts, 1)
/// [Language: eng - Script: Latn - Confidence: 0.999 - Is reliable: true, Language: fra - Script: Latn - Confidence: 0.999 - Is reliable: true]
/// ```
fn batch_detect(texts: Vec<&str>, n_jobs: i16) -> Vec<PyInfo> {
    // Get number of cores
    let mut n_cores: usize = num_cpus::get();
    if n_jobs > 0 && n_jobs < n_cores as i16 {
        n_cores = n_jobs as usize;
    }
    let message = format!(
        "{} texts using {} cores",
        texts.len(),
        format!("{}", colorize(&n_cores.to_string(), TermColor::Red))
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
}

#[pyfunction]
#[pyo3(name = "detect")]
#[pyo3(text_signature = "(text: str)")]
fn py_detect(text: &str) -> PyResult<PyInfo> {
    let info = detect(text);
    match info {
        None => {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Language could not be detected",
            ))
        }
        Some(info) => return Ok(convert_to_py_info(info)),
    }
}

#[pyfunction]
#[pyo3(name = "detect_script")]
#[pyo3(text_signature = "(text: str)")]
fn py_detect_script(text: &str) -> PyResult<PyScript> {
    let script = detect_script(text);
    match script {
        Some(script) => return Ok(convert_to_py_script(script)),
        None => {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Script could not be detected",
            ))
        }
    }
}

#[pyfunction]
#[pyo3(name = "detect_lang")]
#[pyo3(text_signature = "(text: str)")]
fn py_detect_lang(text: &str) -> PyResult<PyLang> {
    let lang = detect_lang(text);
    match lang {
        Some(lang) => return Ok(convert_to_py_lang(lang)),
        None => {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Language could not be detected",
            ))
        }
    }
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

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", env!("CARGO_PKG_AUTHORS"))?;
    m.add("__description__", env!("CARGO_PKG_DESCRIPTION"))?;
    m.add("__license__", env!("CARGO_PKG_LICENSE"))?;

    m.add_class::<PyInfo>()?;
    m.add_function(wrap_pyfunction!(py_detect, m)?)?;
    m.add_class::<PyScript>()?;
    m.add_function(wrap_pyfunction!(py_detect_script, m)?)?;
    m.add_class::<PyLang>()?;
    m.add_function(wrap_pyfunction!(py_detect_lang, m)?)?;
    m.add_function(wrap_pyfunction!(py_batch_detect, m)?)?;
    Ok(())
}
