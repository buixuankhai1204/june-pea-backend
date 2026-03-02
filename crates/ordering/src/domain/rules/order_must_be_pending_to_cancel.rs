use crate::domain::model::OrderStatus;
use shared::BusinessRule;

/// Rule: only a `Pending` order can transition to `Cancelled`.
pub struct OrderMustBePendingToCancelRule {
    current_status: OrderStatus,
}

impl OrderMustBePendingToCancelRule {
    pub fn new(current_status: OrderStatus) -> Self {
        Self { current_status }
    }
}

impl BusinessRule for OrderMustBePendingToCancelRule {
    fn is_broken(&self) -> bool {
        self.current_status != OrderStatus::Pending
    }

    fn message(&self) -> &str {
        "Only orders in Pending status can be cancelled."
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_broken_when_pending() {
        assert!(!OrderMustBePendingToCancelRule::new(OrderStatus::Pending).is_broken());
    }

    #[test]
    fn broken_when_cancelled() {
        assert!(OrderMustBePendingToCancelRule::new(OrderStatus::Cancelled).is_broken());
    }

    #[test]
    fn broken_when_completed() {
        assert!(OrderMustBePendingToCancelRule::new(OrderStatus::Completed).is_broken());
    }
}
