//! All English copy for the public pages.
//! Edit the text in this file. You do not need to change any layout code.

pub const HERO_EYEBROW: &str = "Private by design";

pub const HERO_HEADLINE: &str = "Your personal records, locked in your own browser.";

pub const HERO_SUBTITLE: &str = "Store personal details and website logins in one place. Everything is encrypted on your device before it is uploaded, so the cloud only ever sees scrambled data.";

pub const WELCOME_MESSAGE: &str = "Welcome! I'm Vaulty, your friendly guard. Let's keep your secrets safe together.";

pub const BANNER_TEXT: &str = "Your master passphrase unlocks your vault. Keep it safe, because we cannot recover it for you.";

/// (title, description) pairs for the feature cards.
pub const FEATURES: &[(&str, &str)] = &[
    (
        "Encrypted in your browser",
        "Records are encrypted with a key derived from your master passphrase before they leave your device.",
    ),
    (
        "Two kinds of entries",
        "Save personal information or website and app logins, with custom titles and extra fields.",
    ),
    (
        "Search as you type",
        "Find any record instantly by its title or site name, ignoring capital letters.",
    ),
    (
        "Lock and auto-lock",
        "Lock your vault at any time. It also locks itself after a period of inactivity.",
    ),
    (
        "Works on phones",
        "Designed for Android and iPhone browsers, so you can open your vault on the go.",
    ),
    (
        "Backup you control",
        "Export an encrypted backup file and restore it later with your master passphrase.",
    ),
];

/// (question, answer) pairs for the FAQ accordion.
pub const FAQ_ITEMS: &[(&str, &str)] = &[
    (
        "Can anyone read my vault?",
        "No. Vault records are encrypted in your browser with your master passphrase before upload. The database stores only encrypted data for vault records.",
    ),
    (
        "What if I forget my master passphrase?",
        "Your vault cannot be decrypted without it. Recovering your account password or PIN does not recover the master passphrase.",
    ),
    (
        "Can I use my vault on another device?",
        "Yes. Sign in to your account on the new device, then enter the same master passphrase to unlock the vault.",
    ),
    (
        "Do I need to install an app?",
        "No. My Personal Vault runs in modern browsers on Android, iPhone, and desktop computers.",
    ),
    (
        "Should I test with real information?",
        "Please test with dummy data first. Use real records only after you have finished testing and understand the security notes.",
    ),
];

pub const FOOTER_TEXT: &str = "My Personal Vault, built with Rust and WebAssembly.";

pub const FOOTER_NOTE: &str = "Your master passphrase is never sent to the server.";