//! This module provides the `Provider` trait, which defines the interface for
//! interacting with different AI providers.

#[cfg(feature = "openai")]
pub mod openai;
#[cfg(feature = "openai")]
pub use openai::OpenAI;

// Public OpenAI-compatible provider
#[cfg(feature = "openaicompatible")]
pub mod openai_compatible;
#[cfg(feature = "openaicompatible")]
pub use openai_compatible::OpenAICompatible;

#[cfg(feature = "anthropic")]
pub mod anthropic;
#[cfg(feature = "anthropic")]
pub use anthropic::Anthropic;

#[cfg(feature = "groq")]
pub mod groq;
#[cfg(feature = "groq")]
pub use groq::Groq;

#[cfg(feature = "google")]
pub mod google;
#[cfg(feature = "google")]
pub use google::Google;

#[cfg(feature = "vercel")]
pub mod vercel;
#[cfg(feature = "vercel")]
pub use vercel::Vercel;

#[cfg(feature = "openrouter")]
pub mod openrouter;
#[cfg(feature = "openrouter")]
pub use openrouter::Openrouter;

#[cfg(feature = "mistral")]
pub mod mistral;
#[cfg(feature = "mistral")]
pub use mistral::Mistral;

#[cfg(feature = "amazon-bedrock")]
pub mod amazon_bedrock;
#[cfg(feature = "amazon-bedrock")]
pub use amazon_bedrock::AmazonBedrock;

#[cfg(feature = "togetherai")]
pub mod togetherai;
#[cfg(feature = "togetherai")]
pub use togetherai::TogetherAI;

#[cfg(feature = "xai")]
pub mod xai;
#[cfg(feature = "xai")]
pub use xai::XAI;

// Internal module for OpenAI Chat Completions API compatible providers
#[cfg(feature = "openaichatcompletions")]
pub(crate) mod openai_chat_completions;

// [codegen]
#[cfg(feature = "302ai")]
#[path = "302ai/mod.rs"]
pub mod ai_302;
#[cfg(feature = "302ai")]
pub use ai_302::Ai302;

#[cfg(feature = "abacus")]
pub mod abacus;
#[cfg(feature = "abacus")]
pub use abacus::Abacus;

#[cfg(feature = "aihubmix")]
pub mod aihubmix;
#[cfg(feature = "aihubmix")]
pub use aihubmix::Aihubmix;

#[cfg(feature = "alibaba")]
pub mod alibaba;
#[cfg(feature = "alibaba")]
pub use alibaba::Alibaba;

#[cfg(feature = "alibaba-cn")]
#[path = "alibaba-cn/mod.rs"]
pub mod alibaba_cn;
#[cfg(feature = "alibaba-cn")]
pub use alibaba_cn::AlibabaCn;

#[cfg(feature = "bailing")]
pub mod bailing;
#[cfg(feature = "bailing")]
pub use bailing::Bailing;

#[cfg(feature = "baseten")]
pub mod baseten;
#[cfg(feature = "baseten")]
pub use baseten::Baseten;

#[cfg(feature = "berget")]
pub mod berget;
#[cfg(feature = "berget")]
pub use berget::Berget;

#[cfg(feature = "chutes")]
pub mod chutes;
#[cfg(feature = "chutes")]
pub use chutes::Chutes;

#[cfg(feature = "cloudflare-ai-gateway")]
#[path = "cloudflare-ai-gateway/mod.rs"]
pub mod cloudflare_ai_gateway;
#[cfg(feature = "cloudflare-ai-gateway")]
pub use cloudflare_ai_gateway::CloudflareAiGateway;

#[cfg(feature = "cloudflare-workers-ai")]
#[path = "cloudflare-workers-ai/mod.rs"]
pub mod cloudflare_workers_ai;
#[cfg(feature = "cloudflare-workers-ai")]
pub use cloudflare_workers_ai::CloudflareWorkersAi;

#[cfg(feature = "cortecs")]
pub mod cortecs;
#[cfg(feature = "cortecs")]
pub use cortecs::Cortecs;

#[cfg(feature = "deepseek")]
pub mod deepseek;
#[cfg(feature = "deepseek")]
pub use deepseek::Deepseek;

#[cfg(feature = "fastrouter")]
pub mod fastrouter;
#[cfg(feature = "fastrouter")]
pub use fastrouter::Fastrouter;

#[cfg(feature = "fireworks-ai")]
#[path = "fireworks-ai/mod.rs"]
pub mod fireworks_ai;
#[cfg(feature = "fireworks-ai")]
pub use fireworks_ai::FireworksAi;

#[cfg(feature = "firmware")]
pub mod firmware;
#[cfg(feature = "firmware")]
pub use firmware::Firmware;

#[cfg(feature = "friendli")]
pub mod friendli;
#[cfg(feature = "friendli")]
pub use friendli::Friendli;

#[cfg(feature = "github-copilot")]
#[path = "github-copilot/mod.rs"]
pub mod github_copilot;
#[cfg(feature = "github-copilot")]
pub use github_copilot::GithubCopilot;

#[cfg(feature = "github-models")]
#[path = "github-models/mod.rs"]
pub mod github_models;
#[cfg(feature = "github-models")]
pub use github_models::GithubModels;

#[cfg(feature = "helicone")]
pub mod helicone;
#[cfg(feature = "helicone")]
pub use helicone::Helicone;

#[cfg(feature = "huggingface")]
pub mod huggingface;
#[cfg(feature = "huggingface")]
pub use huggingface::Huggingface;

#[cfg(feature = "iflowcn")]
pub mod iflowcn;
#[cfg(feature = "iflowcn")]
pub use iflowcn::Iflowcn;

#[cfg(feature = "inception")]
pub mod inception;
#[cfg(feature = "inception")]
pub use inception::Inception;

#[cfg(feature = "inference")]
pub mod inference;
#[cfg(feature = "inference")]
pub use inference::Inference;

#[cfg(feature = "io-net")]
#[path = "io-net/mod.rs"]
pub mod io_net;
#[cfg(feature = "io-net")]
pub use io_net::IoNet;

#[cfg(feature = "jiekou")]
pub mod jiekou;
#[cfg(feature = "jiekou")]
pub use jiekou::Jiekou;

#[cfg(feature = "kuae-cloud-coding-plan")]
#[path = "kuae-cloud-coding-plan/mod.rs"]
pub mod kuae_cloud_coding_plan;
#[cfg(feature = "kuae-cloud-coding-plan")]
pub use kuae_cloud_coding_plan::KuaeCloudCodingPlan;

#[cfg(feature = "llama")]
pub mod llama;
#[cfg(feature = "llama")]
pub use llama::Llama;

#[cfg(feature = "lmstudio")]
pub mod lmstudio;
#[cfg(feature = "lmstudio")]
pub use lmstudio::Lmstudio;

#[cfg(feature = "lucidquery")]
pub mod lucidquery;
#[cfg(feature = "lucidquery")]
pub use lucidquery::Lucidquery;

#[cfg(feature = "moark")]
pub mod moark;
#[cfg(feature = "moark")]
pub use moark::Moark;

#[cfg(feature = "modelscope")]
pub mod modelscope;
#[cfg(feature = "modelscope")]
pub use modelscope::Modelscope;

#[cfg(feature = "moonshotai")]
pub mod moonshotai;
#[cfg(feature = "moonshotai")]
pub use moonshotai::Moonshotai;

#[cfg(feature = "moonshotai-cn")]
#[path = "moonshotai-cn/mod.rs"]
pub mod moonshotai_cn;
#[cfg(feature = "moonshotai-cn")]
pub use moonshotai_cn::MoonshotaiCn;

#[cfg(feature = "morph")]
pub mod morph;
#[cfg(feature = "morph")]
pub use morph::Morph;

#[cfg(feature = "nano-gpt")]
#[path = "nano-gpt/mod.rs"]
pub mod nano_gpt;
#[cfg(feature = "nano-gpt")]
pub use nano_gpt::NanoGpt;

#[cfg(feature = "nebius")]
pub mod nebius;
#[cfg(feature = "nebius")]
pub use nebius::Nebius;

#[cfg(feature = "nova")]
pub mod nova;
#[cfg(feature = "nova")]
pub use nova::Nova;

#[cfg(feature = "novita-ai")]
#[path = "novita-ai/mod.rs"]
pub mod novita_ai;
#[cfg(feature = "novita-ai")]
pub use novita_ai::NovitaAi;

#[cfg(feature = "nvidia")]
pub mod nvidia;
#[cfg(feature = "nvidia")]
pub use nvidia::Nvidia;

#[cfg(feature = "ollama-cloud")]
#[path = "ollama-cloud/mod.rs"]
pub mod ollama_cloud;
#[cfg(feature = "ollama-cloud")]
pub use ollama_cloud::OllamaCloud;

#[cfg(feature = "opencode")]
pub mod opencode;
#[cfg(feature = "opencode")]
pub use opencode::Opencode;

#[cfg(feature = "ovhcloud")]
pub mod ovhcloud;
#[cfg(feature = "ovhcloud")]
pub use ovhcloud::Ovhcloud;

#[cfg(feature = "poe")]
pub mod poe;
#[cfg(feature = "poe")]
pub use poe::Poe;

#[cfg(feature = "requesty")]
pub mod requesty;
#[cfg(feature = "requesty")]
pub use requesty::Requesty;

#[cfg(feature = "scaleway")]
pub mod scaleway;
#[cfg(feature = "scaleway")]
pub use scaleway::Scaleway;

#[cfg(feature = "siliconflow")]
pub mod siliconflow;
#[cfg(feature = "siliconflow")]
pub use siliconflow::Siliconflow;

#[cfg(feature = "siliconflow-cn")]
#[path = "siliconflow-cn/mod.rs"]
pub mod siliconflow_cn;
#[cfg(feature = "siliconflow-cn")]
pub use siliconflow_cn::SiliconflowCn;

#[cfg(feature = "stackit")]
pub mod stackit;
#[cfg(feature = "stackit")]
pub use stackit::Stackit;

#[cfg(feature = "stepfun")]
pub mod stepfun;
#[cfg(feature = "stepfun")]
pub use stepfun::Stepfun;

#[cfg(feature = "submodel")]
pub mod submodel;
#[cfg(feature = "submodel")]
pub use submodel::Submodel;

#[cfg(feature = "synthetic")]
pub mod synthetic;
#[cfg(feature = "synthetic")]
pub use synthetic::Synthetic;

#[cfg(feature = "upstage")]
pub mod upstage;
#[cfg(feature = "upstage")]
pub use upstage::Upstage;

#[cfg(feature = "vultr")]
pub mod vultr;
#[cfg(feature = "vultr")]
pub use vultr::Vultr;

#[cfg(feature = "wandb")]
pub mod wandb;
#[cfg(feature = "wandb")]
pub use wandb::Wandb;

#[cfg(feature = "xiaomi")]
pub mod xiaomi;
#[cfg(feature = "xiaomi")]
pub use xiaomi::Xiaomi;

#[cfg(feature = "zai")]
pub mod zai;
#[cfg(feature = "zai")]
pub use zai::Zai;

#[cfg(feature = "zai-coding-plan")]
#[path = "zai-coding-plan/mod.rs"]
pub mod zai_coding_plan;
#[cfg(feature = "zai-coding-plan")]
pub use zai_coding_plan::ZaiCodingPlan;

#[cfg(feature = "zenmux")]
pub mod zenmux;
#[cfg(feature = "zenmux")]
pub use zenmux::Zenmux;

#[cfg(feature = "zhipuai")]
pub mod zhipuai;
#[cfg(feature = "zhipuai")]
pub use zhipuai::Zhipuai;

#[cfg(feature = "zhipuai-coding-plan")]
#[path = "zhipuai-coding-plan/mod.rs"]
pub mod zhipuai_coding_plan;
#[cfg(feature = "zhipuai-coding-plan")]
pub use zhipuai_coding_plan::ZhipuaiCodingPlan;
// [end-codegen]
