use anyhow::{anyhow, Error, Result};

/// A [Multibase]-supported base. See [multibase::Base] for more information.
///
/// Excludes the identity (0x00) base as arbitrary bytes cannot be encoded to a valid UTF-8 string
/// in general.
///
/// [Multibase]: https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03
#[allow(missing_docs)]
#[derive(Debug)]
pub enum Base {
    Base2,
    Base8,
    Base10,
    Base16Lower,
    Base16Upper,
    Base32Lower,
    Base32Upper,
    Base32PadLower,
    Base32PadUpper,
    Base32HexLower,
    Base32HexUpper,
    Base32HexPadLower,
    Base32HexPadUpper,
    Base32Z,
    Base36Lower,
    Base36Upper,
    Base58Flickr,
    Base58Btc,
    Base64,
    Base64Pad,
    Base64Url,
    Base64UrlPad,
}

/// Wrap [multibase::Base] to exclude the identity (0x00) and avoid exporting from a pre-1.0 crate.
impl From<Base> for multibase::Base {
    fn from(base: Base) -> Self {
        match base {
            Base::Base2 => multibase::Base::Base2,
            Base::Base8 => multibase::Base::Base8,
            Base::Base10 => multibase::Base::Base10,
            Base::Base16Lower => multibase::Base::Base16Lower,
            Base::Base16Upper => multibase::Base::Base16Upper,
            Base::Base32Lower => multibase::Base::Base32Lower,
            Base::Base32Upper => multibase::Base::Base32Upper,
            Base::Base32PadLower => multibase::Base::Base32PadLower,
            Base::Base32PadUpper => multibase::Base::Base32PadUpper,
            Base::Base32HexLower => multibase::Base::Base32HexLower,
            Base::Base32HexUpper => multibase::Base::Base32HexUpper,
            Base::Base32HexPadLower => multibase::Base::Base32HexPadLower,
            Base::Base32HexPadUpper => multibase::Base::Base32HexPadUpper,
            Base::Base32Z => multibase::Base::Base32Z,
            Base::Base36Lower => multibase::Base::Base36Lower,
            Base::Base36Upper => multibase::Base::Base36Upper,
            Base::Base58Flickr => multibase::Base::Base58Flickr,
            Base::Base58Btc => multibase::Base::Base58Btc,
            Base::Base64 => multibase::Base::Base64,
            Base::Base64Pad => multibase::Base::Base64Pad,
            Base::Base64Url => multibase::Base::Base64Url,
            Base::Base64UrlPad => multibase::Base::Base64UrlPad,
        }
    }
}

/// Decodes the given `data` as [Multibase] with an inferred [`base`](Base).
///
/// [Multibase]: https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03
pub fn decode_multibase<T>(data: &T) -> Result<Vec<u8>>
where
    T: AsRef<str> + ?Sized,
{
    if data.as_ref().is_empty() {
        return Ok(Vec::new());
    }
    multibase::decode(&data)
        .map(|(_base, output)| output)
        .map_err(|e| anyhow!("{}", e))
}

/// Encodes the given `data` as [Multibase] with the given [`base`](Base), defaults to
/// [`Base::Base58Btc`] if omitted.
///
/// NOTE: [`encode_multibase`] with [`Base::Base58Btc`] is different from [`encode_b58`] as
/// the [Multibase] format prepends a base-encoding-character to the output.
///
/// [Multibase]: https://datatracker.ietf.org/doc/html/draft-multiformats-multibase-03
pub fn encode_multibase<T>(data: &T, base: Option<Base>) -> String
where
    T: AsRef<[u8]> + ?Sized,
{
    multibase::encode(multibase::Base::from(base.unwrap_or(Base::Base58Btc)), data)
}

/// Decodes the given `data` as base58-btc.
pub fn decode_b58<T>(data: &T) -> Result<Vec<u8>>
where
    T: AsRef<[u8]> + ?Sized,
{
    bs58::decode(data)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_vec()
        .map_err(|e| anyhow!("{}", e))
}

/// Encodes the given `data` as base58-btc.
pub fn encode_b58<T>(data: &T) -> String
where
    T: AsRef<[u8]> + ?Sized,
{
    bs58::encode(data)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_string()
}

/// Decodes the given `data` as base64.
pub fn decode_b64<T>(data: &T) -> Result<Vec<u8>>
where
    T: AsRef<[u8]> + ?Sized,
{
    base64::decode_config(data.as_ref(), base64::URL_SAFE).map_err(|e| anyhow!("{}", e))
}

/// Encodes the given `data` as base64.
pub fn encode_b64<T>(data: &T) -> String
where
    T: AsRef<[u8]> + ?Sized,
{
    base64::encode_config(data.as_ref(), base64::URL_SAFE)
}
