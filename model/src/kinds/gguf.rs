use super::{ModelResult, WeightsDeserialize, WeightsLoad};
use candle_core::quantized::gguf_file;

pub struct Gguf {
    inner: gguf_file::Content,
}

impl WeightsDeserialize for Gguf {
    fn deserialize<R: std::io::Read + std::io::Seek>(reader: &mut R) -> ModelResult<Self> {
        Ok(Self {
            inner: gguf_file::Content::read(reader)?,
        })
    }
}

impl WeightsLoad for Gguf {
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
