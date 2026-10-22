use serde::Serialize;

/// Code indicating the activities carried out during a control. This data type
/// is related to Annex 1C requirements 126, 274, 299, 327, and 350.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct ControlType(u8);

impl ControlType {
    pub const NONE: Self = Self(0);
    // Card downloaded/not downloaded during this control activity,
    pub const CARD_DOWNLOADED: Self = Self(0x80);
    /// VU downloaded/not downloaded during this control activity,
    pub const VU_DOWNLOADED: Self = Self(0x40);
    /// Printing done/no printing done during this control activity,
    pub const PRINTING_DONE: Self = Self(0x20);
    /// Display used/no display used during this control activity,
    pub const DISPLAY_USED: Self = Self(0x10);
    /// Calibration parameters checked/not checked during this control activity,
    /// Defined for generation 2; reserved in generation 1.
    pub const CALIBRATION_PARAMETERS: Self = Self(0x08);

    pub fn bits(self) -> u8 {
        self.0
    }

    pub fn contains(self, flags: Self) -> bool {
        self.0 & flags.0 == flags.0
    }
}

impl From<u8> for ControlType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ControlType> for u8 {
    fn from(value: ControlType) -> Self {
        value.bits()
    }
}

impl std::ops::BitOr for ControlType {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combined_controls_keep_every_flag() {
        let control = ControlType::from(0xC0);
        assert!(control.contains(ControlType::CARD_DOWNLOADED));
        assert!(control.contains(ControlType::VU_DOWNLOADED));
        assert!(!control.contains(ControlType::PRINTING_DONE));
        assert_eq!(control, ControlType::CARD_DOWNLOADED | ControlType::VU_DOWNLOADED);
        assert_eq!(serde_json::to_string(&control).unwrap(), "192");

        let control = ControlType::from(0x30);
        assert!(control.contains(ControlType::PRINTING_DONE | ControlType::DISPLAY_USED));
        assert_eq!(serde_json::to_string(&control).unwrap(), "48");
    }

    #[test]
    fn every_control_byte_is_preserved_including_reserved_bits() {
        for raw in u8::MIN..=u8::MAX {
            let control = ControlType::from(raw);
            assert_eq!(u8::from(control), raw);
            assert_eq!(serde_json::to_string(&control).unwrap(), raw.to_string());
        }
    }
}
