use cxx::CxxString;
use rust_bert::pipelines::text_generation::TextGenerationModel;

#[repr(C)]
pub enum MineAIInitDevice {
    CudaIfAvailable,
    CPU,
    CUDA,
    Vulkan,
}

#[repr(C)]
pub struct MineAIInitOptions {
    pub device: MineAIInitDevice,
    pub min_length: i64,
    pub max_length: i64,
}

#[repr(C)]
pub struct MineAIInitResult {
    pub error: *const CxxString,
    pub object: *const MineAIObject,
}

#[repr(C)]
pub struct MineAIGenerationResult {
    pub error: *const CxxString,
    pub text: *const CxxString,
}

#[repr(C)]
pub struct MineAIObject {
    pub model: *const TextGenerationModel,
}
