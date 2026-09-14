//! Generador de secretos con CSPRNG (`OsRng`).

use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::wordlist::WORDS;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateOptions {
    /// "password" | "passphrase" | "hex" | "base64" | "uuid"
    pub kind: String,
    #[serde(default)]
    pub length: Option<usize>,
    #[serde(default)]
    pub words: Option<usize>,
    #[serde(default)]
    pub uppercase: Option<bool>,
    #[serde(default)]
    pub digits: Option<bool>,
    #[serde(default)]
    pub symbols: Option<bool>,
    #[serde(default)]
    pub avoid_ambiguous: Option<bool>,
    #[serde(default)]
    pub separator: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedSecret {
    pub value: String,
    /// Bits de entropía estimados.
    pub entropy_bits: f64,
}

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.?";
const AMBIGUOUS: &str = "Il1O0o";

pub struct SecretService;

impl SecretService {
    pub fn generate(opts: &GenerateOptions) -> Result<GeneratedSecret, String> {
        match opts.kind.as_str() {
            "password" => Ok(Self::password(opts)),
            "passphrase" => Ok(Self::passphrase(opts)),
            "hex" => Ok(Self::hex(opts.length.unwrap_or(32))),
            "base64" => Ok(Self::base64(opts.length.unwrap_or(32))),
            "uuid" => Ok(GeneratedSecret {
                value: Uuid::new_v4().to_string(),
                entropy_bits: 122.0,
            }),
            other => Err(format!("Tipo de secreto desconocido: {other}")),
        }
    }

    fn password(opts: &GenerateOptions) -> GeneratedSecret {
        let len = opts.length.unwrap_or(20).clamp(6, 256);
        let avoid = opts.avoid_ambiguous.unwrap_or(true);

        let mut pool = String::from(LOWER);
        if opts.uppercase.unwrap_or(true) {
            pool.push_str(UPPER);
        }
        if opts.digits.unwrap_or(true) {
            pool.push_str(DIGITS);
        }
        if opts.symbols.unwrap_or(true) {
            pool.push_str(SYMBOLS);
        }
        if avoid {
            pool.retain(|c| !AMBIGUOUS.contains(c));
        }

        let chars: Vec<char> = pool.chars().collect();
        let mut rng = rand::rngs::OsRng;
        let value: String = (0..len)
            .map(|_| chars[rng.gen_range(0..chars.len())])
            .collect();

        let entropy_bits = len as f64 * (chars.len() as f64).log2();
        GeneratedSecret { value, entropy_bits }
    }

    fn passphrase(opts: &GenerateOptions) -> GeneratedSecret {
        let count = opts.words.unwrap_or(6).clamp(3, 12);
        let sep = opts.separator.clone().unwrap_or_else(|| "-".to_string());

        let mut rng = rand::rngs::OsRng;
        let picked: Vec<&str> = (0..count)
            .map(|_| *WORDS.choose(&mut rng).unwrap())
            .collect();

        let mut value = picked.join(&sep);
        if opts.digits.unwrap_or(true) {
            value.push_str(&sep);
            value.push_str(&rng.gen_range(10..100).to_string());
        }
        if opts.uppercase.unwrap_or(false) {
            value = capitalize_words(&value, &sep);
        }

        let entropy_bits = count as f64 * (WORDS.len() as f64).log2();
        GeneratedSecret { value, entropy_bits }
    }

    fn hex(bytes: usize) -> GeneratedSecret {
        let n = bytes.clamp(4, 256);
        let mut buf = vec![0u8; n];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut buf);
        GeneratedSecret {
            value: buf.iter().map(|b| format!("{b:02x}")).collect(),
            entropy_bits: n as f64 * 8.0,
        }
    }

    fn base64(bytes: usize) -> GeneratedSecret {
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let n = bytes.clamp(4, 256);
        let mut buf = vec![0u8; n];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut buf);
        GeneratedSecret {
            value: URL_SAFE_NO_PAD.encode(&buf),
            entropy_bits: n as f64 * 8.0,
        }
    }
}

fn capitalize_words(s: &str, sep: &str) -> String {
    s.split(sep)
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts(kind: &str) -> GenerateOptions {
        GenerateOptions {
            kind: kind.into(),
            length: None,
            words: None,
            uppercase: None,
            digits: None,
            symbols: None,
            avoid_ambiguous: None,
            separator: None,
        }
    }

    #[test]
    fn password_has_requested_length() {
        let mut o = opts("password");
        o.length = Some(24);
        assert_eq!(SecretService::generate(&o).unwrap().value.chars().count(), 24);
    }

    #[test]
    fn passphrase_word_count() {
        let mut o = opts("passphrase");
        o.words = Some(5);
        o.digits = Some(false);
        let v = SecretService::generate(&o).unwrap().value;
        assert_eq!(v.split('-').count(), 5);
    }

    #[test]
    fn hex_is_hex() {
        let mut o = opts("hex");
        o.length = Some(16);
        let v = SecretService::generate(&o).unwrap().value;
        assert_eq!(v.len(), 32);
        assert!(v.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
