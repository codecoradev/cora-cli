/// Known provider presets with their expected env vars and defaults.
pub struct ProviderPreset {
    pub name: &'static str,
    pub env_key: &'static str, // env var for API key
    pub env_url: &'static str, // env var for custom base URL
    pub default_model: &'static str,
    pub default_base_url: &'static str,
}

pub const PRESETS: &[ProviderPreset] = &[
    ProviderPreset {
        name: "openai",
        env_key: "OPENAI_API_KEY",
        env_url: "OPENAI_BASE_URL",
        default_model: "gpt-4o-mini",
        default_base_url: "https://api.openai.com/v1",
    },
    ProviderPreset {
        name: "anthropic",
        env_key: "ANTHROPIC_API_KEY",
        env_url: "ANTHROPIC_BASE_URL",
        default_model: "claude-3-haiku-20240307",
        default_base_url: "https://api.anthropic.com/v1",
    },
    ProviderPreset {
        name: "groq",
        env_key: "GROQ_API_KEY",
        env_url: "GROQ_BASE_URL",
        default_model: "llama-3.1-8b-instant",
        default_base_url: "https://api.groq.com/openai/v1",
    },
    ProviderPreset {
        name: "ollama",
        env_key: "OLLAMA_API_KEY",
        env_url: "OLLAMA_HOST",
        default_model: "llama3.1",
        default_base_url: "http://localhost:11434/v1",
    },
    ProviderPreset {
        name: "zai",
        env_key: "ZAI_API_KEY",
        env_url: "ZAI_BASE_URL",
        default_model: "glm-5.1",
        default_base_url: "https://api.z.ai/api/coding/paas/v4",
    },
];

/// Check if a provider preset has its API key available in the environment.
pub fn preset_has_key(preset: &ProviderPreset) -> bool {
    // Ollama doesn't require an API key — it's always "available" if the host is reachable,
    // but for detection purposes we treat it as available if OLLAMA_HOST is set OR if no
    // key env var is needed (localhost access).
    if preset.name == "ollama" {
        return std::env::var("OLLAMA_HOST").is_ok()
            || std::env::var("OLLAMA_API_KEY").is_ok()
            || std::env::var("CORA_PROVIDER").is_ok_and(|v| v == "ollama");
    }
    std::env::var(preset.env_key).is_ok()
}

/// Return all presets that have their API key detected in the environment.
pub fn detected_presets() -> Vec<&'static ProviderPreset> {
    PRESETS.iter().filter(|p| preset_has_key(p)).collect()
}
