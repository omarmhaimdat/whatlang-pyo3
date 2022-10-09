use pyo3::prelude::*;
use whatlang::{detect, Lang, Script};

#[pyclass(name = "Info")]
struct PyInfo {
    #[pyo3(get, set)]
    lang: String,
    #[pyo3(get, set)]
    script: String,
    #[pyo3(get, set)]
    confidence: f64,
}

#[pymethods]
impl PyInfo {
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{} {} {}", self.lang, self.script, self.confidence))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{} {} {}", self.lang, self.script, self.confidence))
    }
}

#[pyfunction]
#[pyo3(name = "detect")]
#[pyo3(text_signature = "(text)")]
fn detect_lang(text: &str) -> PyResult<PyInfo> {
    let info = detect(text).unwrap();
    let lang = info.lang().code().to_string();
    let script = info.script().to_string();
    let confidence = info.confidence();
    Ok(PyInfo {
        script,
        lang,
        confidence,
    })
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
