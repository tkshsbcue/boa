//! Base64 utility methods (`atob` and `btoa`).
//!
//! See <https://html.spec.whatwg.org/multipage/webappapis.html#atob>.
#![allow(clippy::needless_pass_by_value)]

use boa_engine::realm::Realm;
use boa_engine::{Context, JsError, JsNativeError, JsResult, boa_module, js_string};

#[cfg(test)]
mod tests;

/// A forgiving Base64 engine that accepts input with or without padding,
/// matching the [forgiving-base64 decode](https://infra.spec.whatwg.org/#forgiving-base64-decode)
/// algorithm used by `atob`.
const FORGIVING: base64::engine::GeneralPurpose = base64::engine::GeneralPurpose::new(
    &base64::alphabet::STANDARD,
    base64::engine::general_purpose::GeneralPurposeConfig::new()
        .with_decode_padding_mode(base64::engine::DecodePaddingMode::Indifferent),
);

/// Builds the `InvalidCharacterError` thrown by `atob` and `btoa`.
///
/// The [HTML specification][spec] requires these functions to throw an
/// `InvalidCharacterError` `DOMException`. Boa does not implement
/// `DOMException`, so this returns an ordinary `Error` object whose `name`
/// own property is set to `"InvalidCharacterError"`. This keeps the thrown
/// value a real `Error` instance (so `instanceof Error` and `e.name` work as
/// expected) instead of a bare string.
///
/// [spec]: https://html.spec.whatwg.org/multipage/webappapis.html#atob
fn invalid_character_error(message: &'static str, context: &mut Context) -> JsError {
    let error = JsNativeError::error()
        .with_message(message)
        .into_opaque(context);
    error
        .set(
            js_string!("name"),
            js_string!("InvalidCharacterError"),
            false,
            context,
        )
        .expect("setting `name` on a fresh ordinary error object cannot fail");
    JsError::from_opaque(error.into())
}

/// JavaScript module containing the `atob` and `btoa` functions.
#[boa_module]
pub mod js_module {
    use super::{FORGIVING, invalid_character_error};
    use base64::Engine as _;
    use boa_engine::{Context, JsResult};

    /// The [`btoa()`][mdn] method creates a Base64-encoded ASCII string from
    /// a binary string (i.e., a string in which each character is treated as
    /// a byte of binary data).
    ///
    /// # Errors
    /// Throws an `InvalidCharacterError` (a plain `Error`, since Boa does not
    /// implement `DOMException`) if the string contains any character whose
    /// code point is greater than `0xFF`.
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Window/btoa
    #[allow(clippy::cast_possible_truncation)]
    pub fn btoa(data: String, context: &mut Context) -> JsResult<String> {
        let mut bytes: Vec<u8> = Vec::with_capacity(data.len());
        for c in data.chars() {
            let cp = c as u32;
            if cp > 0xFF {
                return Err(invalid_character_error(
                    "The string to be encoded contains characters outside of the Latin1 range.",
                    context,
                ));
            }
            bytes.push(cp as u8);
        }

        Ok(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }

    /// The [`atob()`][mdn] method decodes a string of data which has been
    /// encoded using Base64 encoding.
    ///
    /// # Errors
    /// Throws an `InvalidCharacterError` (a plain `Error`, since Boa does not
    /// implement `DOMException`) if the input is not valid Base64.
    ///
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Window/atob
    pub fn atob(data: String, context: &mut Context) -> JsResult<String> {
        let cleaned: String = data
            .chars()
            .filter(|c| !matches!(c, ' ' | '\t' | '\n' | '\x0C' | '\r'))
            .collect();

        let bytes = FORGIVING.decode(cleaned.as_bytes()).map_err(|_| {
            invalid_character_error(
                "The string to be decoded is not correctly encoded.",
                context,
            )
        })?;

        Ok(bytes.into_iter().map(char::from).collect())
    }
}

/// Register the `atob` and `btoa` functions in the global context.
///
/// # Errors
/// Returns an error if the functions cannot be registered.
pub fn register(realm: Option<Realm>, context: &mut Context) -> JsResult<()> {
    js_module::boa_register(realm, context)
}
