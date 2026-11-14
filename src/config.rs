//! Public build-time settings.
//!
//! These values are read when the WebAssembly bundle is compiled. GitHub Actions
//! passes them as environment variables (configured in Part 6). Everything here
//! is PUBLIC: anything compiled into the frontend can be read by anyone who opens
//! the site. Never put secrets, service-role keys or passwords in this file.
#![allow(dead_code)]

pub const APP_NAME: &str = "My Personal Vault";

/// Supabase project URL, for example https://abcd1234.supabase.co
pub const SUPABASE_URL: &str = match option_env!("VAULT_SUPABASE_URL") {
    Some(value) => value,
    None => "",
};

/// Supabase anon (public) key. Row Level Security protects the data, not this key.
pub const SUPABASE_ANON_KEY: &str = match option_env!("VAULT_SUPABASE_ANON_KEY") {
    Some(value) => value,
    None => "",
};

/// Base URL of the Cloudflare Worker, for example https://ohmy-vault-api.YOUR-SUBDOMAIN.workers.dev
pub const WORKER_URL: &str = match option_env!("VAULT_WORKER_URL") {
    Some(value) => value,
    None => "",
};

/// Seconds of inactivity before an unlocked vault locks itself.
pub const INACTIVITY_LOCK_SECONDS: u32 = 300;

/// Returns true when the public build variables were supplied at compile time.
pub fn is_configured() -> bool {
    !SUPABASE_URL.is_empty() && !SUPABASE_ANON_KEY.is_empty() && !WORKER_URL.is_empty()
}