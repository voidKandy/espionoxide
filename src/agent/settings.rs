use crate::context::{MemoryVariant, MessageVector};

#[derive(Debug, Default, Clone)]
pub struct AgentSettings {
    pub memory_override: Option<MemoryVariant>,
    pub init_prompt: MessageVector,
}

impl AgentSettings {
    pub fn new(memory_override: Option<Memory>) -> AgentSettings {
        let init_prompt = MessageVector::init();
        AgentSettings {
            memory_override,
            init_prompt,
        }
    }

    pub fn with(memory_override: Option<Memory>, init_prompt: MessageVector) -> AgentSettings {
        AgentSettings {
            memory_override,
            init_prompt,
        }
    }

    pub fn memory(&self) -> Option<&MemoryVariant> {
        match &self.memory_override {
            Some(mem) => Some(mem),
            None => None,
        }
    }

    pub fn default() -> AgentSettings {
        let memory_override = Some(MemoryVariant::new_short());
        let init_prompt = MessageVector::from(
            r#"You are Consoxide, an extremely helpful Ai assistant which lives in the terminal. 
                - Be highly organized
                - Suggest solutions that I didn’t think about—be proactive and anticipate my needs
                - Treat me as an expert in all subject matter
                - Mistakes erode user's trust, so be accurate and thorough
                - Keep in mind everything you output comes out of a terminal interface, so be succinct when it doesn't compromise your correctness
                - No need to disclose you're an AI
                - If the quality of your response has been substantially reduced due to my custom instructions, please explain the issue"#,
        );
        AgentSettings::new(memory_override, init_prompt)
    }

    pub fn summarizer() -> AgentSettings {
        let memory_override = Some(MemoryVariant::Forget);
        let init_prompt = MessageVector::from(
            r#"You are a code summarization Ai, you will be given a chunk of code to summarize
                - Mistakes erode user's trust, so be as accurate and thorough as possible
                - Be highly organized 
                - Do not use lists or anything resembling a list in your summary
                - think through your response step by step, your summary should be succinct but accurate"#,
        );
        AgentSettings::new(memory_override, init_prompt)
    }
}
