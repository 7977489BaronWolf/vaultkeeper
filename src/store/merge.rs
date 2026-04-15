use std::collections::HashMap;

/// Strategy for resolving key conflicts during a merge
#[derive(Debug, Clone, PartialEq)]
pub enum MergeStrategy {
    /// Keep the value from the base (destination) store
    KeepBase,
    /// Overwrite with the value from the incoming (source) store
    Overwrite,
    /// Only add keys that don't exist in the base
    AddOnly,
}

/// Result of a merge operation
#[derive(Debug, Default)]
pub struct MergeResult {
    pub added: Vec<String>,
    pub overwritten: Vec<String>,
    pub skipped: Vec<String>,
}

/// Merge `incoming` into `base` using the given strategy.
/// Returns the merged map and a summary of changes.
pub fn merge_envs(
    base: &mut HashMap<String, String>,
    incoming: &HashMap<String, String>,
    strategy: &MergeStrategy,
) -> MergeResult {
    let mut result = MergeResult::default();

    for (key, value) in incoming {
        if base.contains_key(key) {
            match strategy {
                MergeStrategy::Overwrite => {
                    base.insert(key.clone(), value.clone());
                    result.overwritten.push(key.clone());
                }
                MergeStrategy::KeepBase | MergeStrategy::AddOnly => {
                    result.skipped.push(key.clone());
                }
            }
        } else {
            match strategy {
                MergeStrategy::KeepBase | MergeStrategy::Overwrite | MergeStrategy::AddOnly => {
                    base.insert(key.clone(), value.clone());
                    result.added.push(key.clone());
                }
            }
        }
    }

    result.added.sort();
    result.overwritten.sort();
    result.skipped.sort();

    result
}
