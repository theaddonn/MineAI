extern crate cxx;
extern crate rust_bert;
extern crate tch;

use std::ptr;

use cxx::{CxxString, let_cxx_string};
use rust_bert::gpt_neo::{GptNeoConfigResources, GptNeoMergesResources, GptNeoModelResources, GptNeoVocabResources};
use rust_bert::pipelines::common::{ModelResource, ModelType};
use rust_bert::pipelines::text_generation::{TextGenerationConfig, TextGenerationModel};
use rust_bert::resources::RemoteResource;
use tch::{Cuda, Device};

use mineai::{MineAIGenerationResult, MineAIInitDevice, MineAIInitOptions, MineAIInitResult, MineAIObject};

mod mineai;

#[no_mangle]
pub extern "C" fn mine_ai_init(options: MineAIInitOptions) -> MineAIInitResult {
    println!("HELLO FROM RUST :3");

    let model_resource = ModelResource::Torch(Box::new(RemoteResource::from_pretrained(
        GptNeoModelResources::GPT_NEO_1_3B,
    )));

    let config_resource = Box::new(RemoteResource::from_pretrained(GptNeoConfigResources::GPT_NEO_1_3B));
    let vocab_resource = Box::new(RemoteResource::from_pretrained(GptNeoVocabResources::GPT_NEO_1_3B));

    let merges_resource = Some(RemoteResource::from_pretrained(GptNeoMergesResources::GPT_NEO_1_3B)).map(|r| Box::new(r) as Box<_>);

    let config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        min_length: options.min_length,
        max_length: if options.max_length < 0 { None } else { Some(options.max_length) },
        device: match options.device {
            MineAIInitDevice::CudaIfAvailable => {
                if Cuda::is_available() {
                    Device::Cuda(0)
                } else {
                    Device::Cpu
                }
            }
            MineAIInitDevice::CPU => { Device::Cpu }
            MineAIInitDevice::CUDA => { Device::Cuda(0) }
            MineAIInitDevice::Vulkan => { Device::Vulkan }
        },
        ..Default::default()
    };

    let model = match TextGenerationModel::new(config) {
        Ok(v) => { v }
        Err(e) => {
            let_cxx_string!(error = e.to_string());
            return MineAIInitResult { error: &*error, object: ptr::null() };
        }
    };

    MineAIInitResult {
        error: ptr::null(),
        object: &mut MineAIObject {
            model: &model,
        },
    }
}

#[no_mangle]
pub extern "C" fn mine_ai_generate(param_object: MineAIObject, prompt: CxxString) -> MineAIGenerationResult {
    let model = unsafe { ptr::read(param_object.model) };

    match model.generate(&["You are MineAI"], Some(prompt.to_string().as_str())) {
        Ok(v) => {
            let_cxx_string!(text = v.join(". "));

            MineAIGenerationResult {
                error: ptr::null(),
                text: &*text,
            }
        }
        Err(e) => {
            let_cxx_string!(error = e.to_string());

            MineAIGenerationResult {
                error: &*error,
                text: ptr::null(),
            }
        }
    }
}