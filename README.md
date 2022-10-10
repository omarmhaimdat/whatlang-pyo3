# Python Binding for WhatLang

![PyPI](https://img.shields.io/pypi/v/popups) ![PyPI - Python Version](https://img.shields.io/pypi/pyversions/popups)
![License](https://img.shields.io/pypi/l/popups) ![PyPI - Downloads](https://img.shields.io/pypi/dm/popups)

WhatLang is a Python library for detecting the language of a text. It is based on the [WhatLang](https://github.com/greyblake/whatlang-rs) Rust library.

## Installation

```bash
pip install whatlang-pyo3
```

## Usage

### Detect language

```python
>>> from whatlang import detect, detect_script
>>> detect("This is written in English")
Language: eng - Script: Latin - Confidence: 0.11450955767632877 - Is reliable: false
>>> detect.to_iso()
Language: en - Script: Latin - Confidence: 0.11450955767632877 - Is reliable: false
```

### Detect script

```python
>>> detect_script("This is written in English")
Name: Latin - Languages: spa, eng, por, ind, fra, deu, jav, vie, ita, tur, pol, ron, hrv, nld, uzb, hun, aze, ces, zul, swe, aka, sna, afr, fin, slk, tgl, tuk, dan, nob, cat, lit, slv, epo, lav, est, lat
```