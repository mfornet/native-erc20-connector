use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Struct to represent bytes (Vec<u8>) and serialize them as base64-encoded string.
#[derive(Debug, Clone)]
pub struct BytesBase64 {
    bytes: Vec<u8>,
}

impl From<Vec<u8>> for BytesBase64 {
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl Into<Vec<u8>> for BytesBase64 {
    fn into(self) -> Vec<u8> {
        self.bytes
    }
}

impl Serialize for BytesBase64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&base64::encode(&self.bytes))
    }
}

impl<'de> Deserialize<'de> for BytesBase64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = base64::decode(&s).map_err(serde::de::Error::custom)?;
        Ok(Self { bytes })
    }
}
