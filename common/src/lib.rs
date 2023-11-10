use hex::{FromHex, FromHexError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn from_hex_string(color: &str) -> Result<Self, FromHexError> {
        let [red, green, blue] = <[u8; 3]>::from_hex(color)?;
        Ok(Self::from_rgb(red, green, blue))
    }

    pub fn to_hex_string(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    pub fn to_tuple(self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hex_string() {
        assert_eq!(
            Color::from_rgb(215, 0, 34).to_hex_string(),
            "d70022".to_string()
        );
    }

    #[test]
    fn from_hex_string() {
        assert_eq!(
            Color::from_hex_string("d70022").unwrap(),
            Color::from_rgb(215, 0, 34)
        );
    }
}
