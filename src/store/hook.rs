use std::collections::HashMap;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HookEvent {
    PreLock,
    PostLock,
    PreUnlock,
    PostUnlock,
    PostSet,
    PostDelete,
    PostRotate,
}

impl std::fmt::Display for HookEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HookEvent::PreLock => "pre-lock",
            HookEvent::PostLock => "post-lock",
            HookEvent::PreUnlock => "pre-unlock",
            HookEvent::PostUnlock => "post-unlock",
            HookEvent::PostSet => "post-set",
            HookEvent::PostDelete => "post-delete",
            HookEvent::PostRotate => "post-rotate",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hook {
    pub event: HookEvent,
    pub command: String,
    pub enabled: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HookStore {
    pub hooks: Vec<Hook>,
}

impl HookStore {
    pub fn add_hook(&mut self, event: HookEvent, command: String) {
        self.hooks.push(Hook { event, command, enabled: true });
    }

    pub fn remove_hook(&mut self, event: &HookEvent, command: &str) -> bool {
        let before = self.hooks.len();
        self.hooks.retain(|h| !(&h.event == event && h.command == command));
        self.hooks.len() < before
    }

    pub fn list_hooks(&self) -> &[Hook] {
        &self.hooks
    }

    pub fn run_hooks(&self, event: &HookEvent, env: &HashMap<String, String>) -> Vec<anyhow::Result<()>> {
        self.hooks
            .iter()
            .filter(|h| &h.event == event && h.enabled)
            .map(|h| {
                let parts: Vec<&str> = h.command.split_whitespace().collect();
                if parts.is_empty() {
                    return Err(anyhow::anyhow!("Empty hook command"));
                }
                let status = Command::new(parts[0])
                    .args(&parts[1..])
                    .envs(env)
                    .status()
                    .map_err(|e| anyhow::anyhow!("Hook '{}' failed to start: {}", h.command, e))?;
                if status.success() {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("Hook '{}' exited with status: {}", h.command, status))
                }
            })
            .collect()
    }
}
