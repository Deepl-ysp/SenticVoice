use pyo3::prelude::*;
use pyo3::types::{PyDict};
use std::path::Path;

pub struct TTSBridge {
    module: Py<PyModule>,
}

#[derive(Debug, Clone)]
pub struct SynthesisResult {
    pub audio_path: String,
    pub duration: f64,
    pub sample_rate: u32,
}

impl TTSBridge {
    pub fn new() -> PyResult<Self> {
        Python::with_gil(|py| {
            let module = PyModule::import_bound(py, "tts_engine")?;
            Ok(Self { module })
        })
    }

    pub fn synthesize(
        &self,
        text: &str,
        model_path: &str,
        emod_path: Option<&str>,
        output_path: &str,
    ) -> PyResult<()> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("text", text)?;
            kwargs.set_item("model_path", model_path)?;
            kwargs.set_item("output_path", output_path)?;

            if let Some(emod) = emod_path {
                kwargs.set_item("emod_path", emod)?;
            }

            module.call_method("synthesize", (), Some(&kwargs))?;
            Ok(())
        })
    }

    pub fn synthesize_with_params(
        &self,
        text: &str,
        model_path: &str,
        output_path: &str,
        emotion_params: Option<&str>,
    ) -> PyResult<()> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let kwargs = PyDict::new_bound(py);
            kwargs.set_item("text", text)?;
            kwargs.set_item("model_path", model_path)?;
            kwargs.set_item("output_path", output_path)?;

            if let Some(params) = emotion_params {
                kwargs.set_item("emotion_params", params)?;
            }

            module.call_method("synthesize", (), Some(&kwargs))?;
            Ok(())
        })
    }

    pub fn load_model(&self, model_path: &str) -> PyResult<String> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let result = module.call_method1("load_model", (model_path,))?;
            result.extract()
        })
    }

    pub fn unload_model(&self, model_id: &str) -> PyResult<()> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            module.call_method1("unload_model", (model_id,))?;
            Ok(())
        })
    }

    pub fn get_model_info(&self, model_path: &str) -> PyResult<serde_json::Value> {
        Python::with_gil(|py| {
            let module = self.module.bind(py);
            let result = module.call_method1("get_model_info", (model_path,))?;
            let json_str: String = result.extract()?;
            Ok(serde_json::from_str(&json_str).unwrap_or_default())
        })
    }

    pub fn get_audio_duration(path: &Path) -> PyResult<f64> {
        let reader = hound::WavReader::open(path)?;
        let duration = reader.duration() as f64 / reader.spec().sample_rate as f64;
        Ok(duration)
    }
}
