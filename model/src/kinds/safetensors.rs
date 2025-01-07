use super::{ModelResult, WeightsDeserialize, WeightsLoad};
use candle_core::safetensors::{BufferedSafetensors, Load};
use safetensors::tensor::TensorView;
pub struct SafeTensors {
    inner: BufferedSafetensors,
}
impl SafeTensors {
    pub fn get(&self, name: &str) -> ModelResult<TensorView> {
        Ok(self.inner.get(name)?)
    }
}
impl WeightsDeserialize for SafeTensors {
    fn deserialize<R: std::io::Read>(reader: &mut R) -> ModelResult<Self> {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;
        Ok(Self {
            inner: BufferedSafetensors::new(buffer)?,
        })
    }
}

impl WeightsLoad for SafeTensors {
    fn load(
        &self,
        device: &candle_core::Device,
    ) -> ModelResult<std::collections::HashMap<String, candle_core::Tensor>> {
        self.inner
            .tensors()
            .into_iter()
            .map(|(name, tensor)| Ok((name, tensor.load(device)?)))
            .collect()
    }
}
