use super::{
    enums::DeviceType,
    types::{DeviceLocation, DeviceResult},
};
use candle_core::Device as CandleDevice;
pub struct Device {
    device: CandleDevice,
}

impl Device {
    pub fn init() -> DeviceResult<Self> {
        let device_type = Self::detect_device();
        Self::with_device_type(device_type)
    }

    pub fn with_device_type(device_type: DeviceType) -> DeviceResult<Device> {
        let device = match device_type {
            DeviceType::Metal(ordinal) => CandleDevice::new_metal(ordinal)?,
            DeviceType::CUDA(ordinal) => CandleDevice::new_cuda(ordinal)?,
            DeviceType::CPU => CandleDevice::Cpu,
        };
        Ok(Device { device })
    }
    pub fn metal(ordinal: usize) -> DeviceResult<Device> {
        Ok(Device {
            device: CandleDevice::new_metal(ordinal)?,
        })
    }

    //TODO: Test once I'll move to Windows
    pub fn cuda(ordinal: usize) -> DeviceResult<Device> {
        Ok(Device {
            device: CandleDevice::new_cuda(ordinal)?,
        })
    }

    //TODO: Test once I'll move to Windows
    pub fn cuda_with_stream(ordinal: usize) -> DeviceResult<Device> {
        Ok(Device {
            device: CandleDevice::new_cuda_with_stream(ordinal)?,
        })
    }

    pub fn cpu() -> DeviceResult<Device> {
        Ok(Device {
            device: CandleDevice::Cpu,
        })
    }

    //TODO: Rework ordinal system and change 0 to automatical detection and distribution
    fn detect_device() -> DeviceType {
        let cuda = super::utils::check_if_cuda();
        let metal = super::utils::check_if_metal();
        if cuda {
            return DeviceType::CUDA(0);
        } else if metal {
            return DeviceType::Metal(0);
        }
        DeviceType::CPU
    }

    pub fn sync(&self) -> DeviceResult<()> {
        Ok(self.device.synchronize()?)
    }

    pub fn set_seed(&self, seed: u64) -> DeviceResult<()> {
        Ok(self.device.set_seed(seed)?)
    }

    pub fn is_cpu(&self) -> bool {
        self.device.is_cpu()
    }

    pub fn is_cuda(&self) -> bool {
        self.device.is_cuda()
    }

    pub fn is_metal(&self) -> bool {
        self.device.is_metal()
    }

    pub fn supports_bf16(&self) -> bool {
        self.device.supports_bf16()
    }

    pub fn bf16_default_to_f32(&self) -> candle_core::DType {
        self.device.bf16_default_to_f32()
    }

    pub fn same_device(&self, rhc: &Device) -> bool {
        self.device.same_device(&rhc.device)
    }

    pub fn location(&self) -> DeviceLocation {
        self.device.location()
    }
}
