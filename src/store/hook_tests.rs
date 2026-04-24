#[cfg(test)]
mod tests {
    use super::super::hook::*;
    use std::collections::HashMap;

    fn make_env() -> HashMap<String, String> {
        HashMap::new()
    }

    #[test]
    fn test_add_and_list_hooks() {
        let mut store = HookStore::default();
        store.add_hook(HookEvent::PostSet, "echo set".to_string());
        store.add_hook(HookEvent::PostDelete, "echo delete".to_string());
        assert_eq!(store.list_hooks().len(), 2);
    }

    #[test]
    fn test_remove_hook() {
        let mut store = HookStore::default();
        store.add_hook(HookEvent::PostSet, "echo set".to_string());
        let removed = store.remove_hook(&HookEvent::PostSet, "echo set");
        assert!(removed);
        assert_eq!(store.list_hooks().len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_hook() {
        let mut store = HookStore::default();
        let removed = store.remove_hook(&HookEvent::PostSet, "echo set");
        assert!(!removed);
    }

    #[test]
    fn test_run_hooks_success() {
        let mut store = HookStore::default();
        store.add_hook(HookEvent::PostSet, "echo hello".to_string());
        let results = store.run_hooks(&HookEvent::PostSet, &make_env());
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
    }

    #[test]
    fn test_run_hooks_wrong_event() {
        let mut store = HookStore::default();
        store.add_hook(HookEvent::PostSet, "echo hello".to_string());
        let results = store.run_hooks(&HookEvent::PostDelete, &make_env());
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_run_hooks_invalid_command() {
        let mut store = HookStore::default();
        store.add_hook(HookEvent::PostRotate, "__nonexistent_cmd_xyz__".to_string());
        let results = store.run_hooks(&HookEvent::PostRotate, &make_env());
        assert_eq!(results.len(), 1);
        assert!(results[0].is_err());
    }

    #[test]
    fn test_hook_event_display() {
        assert_eq!(HookEvent::PreLock.to_string(), "pre-lock");
        assert_eq!(HookEvent::PostUnlock.to_string(), "post-unlock");
        assert_eq!(HookEvent::PostRotate.to_string(), "post-rotate");
    }

    #[test]
    fn test_disabled_hook_not_run() {
        let mut store = HookStore::default();
        store.add_hook(HookEvent::PostSet, "echo hello".to_string());
        store.hooks[0].enabled = false;
        let results = store.run_hooks(&HookEvent::PostSet, &make_env());
        assert_eq!(results.len(), 0);
    }
}
