use llama_core::model::LlamaModel;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeCapabilities {
    pub memory_size: u64,
    pub num_cores: Option<u64>,
    pub hardware_type: String, // e.g. "cpu", "cuda", "mps"
}

pub struct NodeService {
    model_list: Vec<LlamaModel>,
    capabilities: NodeCapabilities,
}

impl NodeService {
    pub fn new(model_list: Vec<LlamaModel>) -> Self {
        let capabilities = NodeCapabilities {
            memory_size: 0,
            num_cores: None,
            hardware_type: "".to_string(),
        };
        Self { model_list, capabilities }
    }
}
