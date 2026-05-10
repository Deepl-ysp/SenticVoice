use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub confidence: f64,
}

pub struct AnnotatorBridge {
    module: Py<PyModule>,
}

impl AnnotatorBridge {
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let module = PyModule::import_bound(py, "annotator")?;
            Ok(Self { module })
        })
    }

    pub fn transcribe(&self, audio_path: &str, language: &str) -> PyResult<TranscriptionResult> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("audio_path", audio_path)?;
            kwargs.set_item("language", language)?;

            let result = module.call_method("transcribe", (), Some(&kwargs))?;

            let dict: &Bound<'_, PyDict> = result.downcast()?;

            let text: String = dict
                .get_item("text")?
                .unwrap()
                .extract()?;

            let raw_segments: Vec<serde_json::Value> = dict
                .get_item("segments")?
                .unwrap()
                .extract()?;

            let segments: Vec<Segment> = raw_segments
                .into_iter()
                .map(|s| {
                    Segment {
                        start: s.get("start").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        end: s.get("end").and_then(|v| v.as_f64()).unwrap_or(0.0),
                        text: s.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        confidence: s.get("confidence").and_then(|v| v.as_f64()).unwrap_or(1.0),
                    }
                })
                .collect();

            Ok(TranscriptionResult { text, segments })
        })
    }

    pub fn batch_transcribe(
        &self,
        audio_paths: Vec<String>,
        language: &str,
    ) -> PyResult<Vec<TranscriptionResult>> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("audio_paths", audio_paths)?;
            kwargs.set_item("language", language)?;

            let result = module.call_method("batch_transcribe", (), Some(&kwargs))?;
            let results: Vec<serde_json::Value> = result.extract()?;
            let transcriptions: Vec<TranscriptionResult> = results
                .into_iter()
                .map(|r| TranscriptionResult {
                    text: r.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    segments: r
                        .get("segments")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|s| {
                                    Some(Segment {
                                        start: s.get("start")?.as_f64()?,
                                        end: s.get("end")?.as_f64()?,
                                        text: s.get("text")?.as_str()?.to_string(),
                                        confidence: s.get("confidence")?.as_f64()?,
                                    })
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                })
                .collect();

            Ok(transcriptions)
        })
    }

    pub fn align_text(
        &self,
        audio_path: &str,
        text: &str,
    ) -> PyResult<Vec<Segment>> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("audio_path", audio_path)?;
            kwargs.set_item("text", text)?;

            let result = module.call_method("align_text", (), Some(&kwargs))?;
            let alignments: Vec<serde_json::Value> = result.extract()?;

            let segments: Vec<Segment> = alignments
                .into_iter()
                .map(|a| Segment {
                    start: a.get("start").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    end: a.get("end").and_then(|v| v.as_f64()).unwrap_or(0.0),
                    text: a.get("text").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                    confidence: a.get("confidence").and_then(|v| v.as_f64()).unwrap_or(1.0),
                })
                .collect();

            Ok(segments)
        })
    }
}
