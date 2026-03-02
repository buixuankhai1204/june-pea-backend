use shared::BusinessRule;

/// Rule: an order must contain at least one item.
pub struct OrderMustHaveItemsRule {
    item_count: usize,
}

impl OrderMustHaveItemsRule {
    pub fn new(item_count: usize) -> Self {
        Self { item_count }
    }
}

impl BusinessRule for OrderMustHaveItemsRule {
    fn is_broken(&self) -> bool {
        self.item_count == 0
    }

    fn message(&self) -> &str {
        "An order must contain at least one item."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_broken_when_items_present() {
        assert!(!OrderMustHaveItemsRule::new(1).is_broken());
        assert!(!OrderMustHaveItemsRule::new(5).is_broken());
    }

    #[test]
    fn broken_when_no_items() {
        assert!(OrderMustHaveItemsRule::new(0).is_broken());
    }
}
