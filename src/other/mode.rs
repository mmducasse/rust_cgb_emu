#[derive(Clone, Copy)]
pub enum CompatibilityMode {
    DmgOnly,
    CgbBackward,
    CgbOnly,
}

impl CompatibilityMode {
    pub fn is_cgb_only(self) -> bool {
        return matches!(self, CompatibilityMode::CgbOnly);
    }

    pub fn is_cgb(self) -> bool {
        return matches!(
            self,
            CompatibilityMode::CgbOnly | CompatibilityMode::CgbBackward
        );
    }
}
