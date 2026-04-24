pub mod acl;
pub mod alias;
pub mod audit;
pub mod backup;
pub mod compress;
pub mod env_group;
pub mod fmt;
pub mod history;
pub mod hook;
pub mod lint;
pub mod merge;
pub mod namespace;
pub mod pin;
pub mod plain;
pub mod policy;
pub mod profile;
pub mod quota;
pub mod rename;
pub mod schema;
pub mod search;
pub mod snapshot;
pub mod tags;
pub mod template;
pub mod ttl;
pub mod validate;
pub mod watch;

#[cfg(test)]
mod acl_tests;
#[cfg(test)]
mod alias_tests;
#[cfg(test)]
mod audit_tests;
#[cfg(test)]
mod backup_tests;
#[cfg(test)]
mod compress_tests;
#[cfg(test)]
mod env_group_tests;
#[cfg(test)]
mod fmt_tests;
#[cfg(test)]
mod history_tests;
#[cfg(test)]
mod hook_tests;
#[cfg(test)]
mod lint_tests;
#[cfg(test)]
mod merge_tests;
#[cfg(test)]
mod namespace_tests;
#[cfg(test)]
mod pin_tests;
#[cfg(test)]
mod policy_tests;
#[cfg(test)]
mod profile_tests;
#[cfg(test)]
mod quota_tests;
#[cfg(test)]
mod schema_tests;
#[cfg(test)]
mod search_tests;
#[cfg(test)]
mod snapshot_tests;
#[cfg(test)]
mod tags_tests;
#[cfg(test)]
mod template_tests;
#[cfg(test)]
mod ttl_tests;
#[cfg(test)]
mod validate_tests;
#[cfg(test)]
mod watch_tests;
#[cfg(test)]
mod tests;
