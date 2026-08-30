//! Local inference boundary. Model execution is intentionally absent in Phase 0.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelTier {
    Tiny,
    Small,
    Medium,
}

#[must_use]
pub const fn default_context_tokens(tier: ModelTier) -> usize {
    match tier {
        ModelTier::Tiny => 1_024,
        ModelTier::Small => 2_048,
        ModelTier::Medium => 4_096,
    }
}

#[cfg(test)]
mod tests {
    use super::{default_context_tokens, ModelTier};

    #[test]
    fn tiers_have_increasing_context() {
        assert!(default_context_tokens(ModelTier::Tiny)
            < default_context_tokens(ModelTier::Medium));
    }
}

