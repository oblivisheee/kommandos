use super::ModelResult;
pub trait WeightsDeserialize: Sized {
    fn deserialize<R: std::io::Read + std::io::Seek>(reader: &mut R) -> ModelResult<Self>;
}

pub trait WeightsLoad: Sized {
    fn load(
        &self,
        device: &candle_core::Device,
    ) -> ModelResult<std::collections::HashMap<String, candle_core::Tensor>>;
}

pub trait QuantizedWeightsLoad: Sized {
    fn load_quatized(&self, device: &candle_core::Device) -> ModelResult {}
}
