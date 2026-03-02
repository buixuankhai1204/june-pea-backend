use crate::error::AppError;

/// Every domain business rule implements this trait.
pub trait BusinessRule {
    /// Returns `true` when the rule is violated.
    fn is_broken(&self) -> bool;
    /// Human-readable description of the violation.
    fn message(&self) -> &str;
}

/// Evaluates a rule and converts a violation into `AppError::Validation`.
pub fn ensure(rule: &dyn BusinessRule) -> Result<(), AppError> {
    if rule.is_broken() {
        Err(AppError::Validation(rule.message().to_string()))
    } else {
        Ok(())
    }
}
