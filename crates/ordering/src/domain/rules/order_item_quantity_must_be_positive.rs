use shared::BusinessRule;

/// Rule: every order item must have a quantity greater than zero.
pub struct OrderItemQuantityMustBePositiveRule {
    quantity: i32,
}

impl OrderItemQuantityMustBePositiveRule {
    pub fn new(quantity: i32) -> Self {
        Self { quantity }
    }
}

impl BusinessRule for OrderItemQuantityMustBePositiveRule {
    fn is_broken(&self) -> bool {
        self.quantity <= 0
    }

    fn message(&self) -> &str {
        "Each order item must have a quantity greater than zero."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_broken_when_positive() {
        assert!(!OrderItemQuantityMustBePositiveRule::new(1).is_broken());
        assert!(!OrderItemQuantityMustBePositiveRule::new(100).is_broken());
    }

    #[test]
    fn broken_when_zero() {
        assert!(OrderItemQuantityMustBePositiveRule::new(0).is_broken());
    }

    #[test]
    fn broken_when_negative() {
        assert!(OrderItemQuantityMustBePositiveRule::new(-5).is_broken());
    }
}
