use {
    super::TryBinaryOperator,
    crate::{
        data::{NumericBinaryOperator, ValueError},
        prelude::Value,
        result::Result,
    },
    std::cmp::Ordering,
    Value::*,
};

impl PartialEq<Value> for f64 {
    fn eq(&self, other: &Value) -> bool {
        let lhs = *self;

        match *other {
            I8(rhs) => lhs == rhs as f64,
            I64(rhs) => lhs == rhs as f64,
            F64(rhs) => lhs == rhs,
            _ => false,
        }
    }
}

impl PartialOrd<Value> for f64 {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        match *other {
            I8(rhs) => self.partial_cmp(&(rhs as f64)),
            I64(rhs) => self.partial_cmp(&(rhs as f64)),
            F64(rhs) => self.partial_cmp(&rhs),
            _ => None,
        }
    }
}

impl TryBinaryOperator for f64 {
    type Rhs = Value;

    fn try_add(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => Ok(F64(lhs + rhs as f64)),
            I64(rhs) => Ok(F64(lhs + rhs as f64)),
            F64(rhs) => Ok(F64(lhs + rhs)),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: F64(lhs),
                operator: NumericBinaryOperator::Add,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }

    fn try_subtract(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => Ok(F64(lhs - rhs as f64)),
            I64(rhs) => Ok(F64(lhs - rhs as f64)),
            F64(rhs) => Ok(F64(lhs - rhs)),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: F64(lhs),
                operator: NumericBinaryOperator::Subtract,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }

    fn try_multiply(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => Ok(F64(lhs * rhs as f64)),
            I64(rhs) => Ok(F64(lhs * rhs as f64)),
            F64(rhs) => Ok(F64(lhs * rhs)),
            Interval(rhs) => Ok(Interval(lhs * rhs)),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: F64(lhs),
                operator: NumericBinaryOperator::Multiply,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }

    fn try_divide(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => Ok(F64(lhs / rhs as f64)),
            I64(rhs) => Ok(F64(lhs / rhs as f64)),
            F64(rhs) => Ok(F64(lhs / rhs)),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: F64(lhs),
                operator: NumericBinaryOperator::Divide,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }

    fn try_modulo(&self, rhs: &Self::Rhs) -> Result<Value> {
        let lhs = *self;

        match *rhs {
            I8(rhs) => Ok(F64(lhs % rhs as f64)),
            I64(rhs) => Ok(F64(lhs % rhs as f64)),
            F64(rhs) => Ok(F64(lhs % rhs)),
            Null => Ok(Null),
            _ => Err(ValueError::NonNumericMathOperation {
                lhs: F64(lhs),
                operator: NumericBinaryOperator::Modulo,
                rhs: rhs.clone(),
            }
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{TryBinaryOperator, Value::*},
        crate::data::{NumericBinaryOperator, ValueError},
        std::cmp::Ordering,
    };

    #[test]
    fn eq() {
        let base = 1.0_f64;

        assert_eq!(base, I8(1));
        assert_eq!(base, I64(1));
        assert_eq!(base, F64(1.0));

        assert_ne!(base, Bool(true));
    }

    #[test]
    fn partial_cmp() {
        let base = 1.0_f64;

        assert_eq!(base.partial_cmp(&I8(1)), Some(Ordering::Equal));
        assert_eq!(base.partial_cmp(&I64(1)), Some(Ordering::Equal));
        assert_eq!(base.partial_cmp(&F64(1.0)), Some(Ordering::Equal));

        assert_eq!(base.partial_cmp(&Bool(true)), None);
    }

    #[test]
    fn try_add() {
        let base = 1.0_f64;

        assert!(matches!(base.try_add(&I8(1)), Ok(F64(x)) if (x - 2.0).abs() < f64::EPSILON ));
        assert!(matches!(base.try_add(&I64(1)), Ok(F64(x)) if (x - 2.0).abs() < f64::EPSILON ));
        assert!(matches!(base.try_add(&F64(1.0)), Ok(F64(x)) if (x - 2.0).abs() < f64::EPSILON ));

        assert_eq!(
            base.try_add(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: F64(1.0),
                operator: NumericBinaryOperator::Add,
                rhs: Bool(true)
            }
            .into())
        );
    }

    #[test]
    fn try_subtract() {
        let base = 1.0_f64;

        assert!(matches!(base.try_subtract(&I8(1)), Ok(F64(x)) if (x - 0.0).abs() < f64::EPSILON ));
        assert!(
            matches!(base.try_subtract(&I64(1)), Ok(F64(x)) if (x - 0.0).abs() < f64::EPSILON )
        );
        assert!(
            matches!(base.try_subtract(&F64(1.0)), Ok(F64(x)) if (x - 0.0).abs() < f64::EPSILON )
        );

        assert_eq!(
            base.try_subtract(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: F64(1.0),
                operator: NumericBinaryOperator::Subtract,
                rhs: Bool(true)
            }
            .into())
        );
    }

    #[test]
    fn try_multiply() {
        let base = 1.0_f64;

        assert!(matches!(base.try_multiply(&I8(1)), Ok(F64(x)) if (x - 1.0).abs() < f64::EPSILON ));
        assert!(
            matches!(base.try_multiply(&I64(1)), Ok(F64(x)) if (x - 1.0).abs() < f64::EPSILON )
        );
        assert!(
            matches!(base.try_multiply(&F64(1.0)), Ok(F64(x)) if (x - 1.0).abs() < f64::EPSILON )
        );

        assert_eq!(
            base.try_multiply(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: F64(1.0),
                operator: NumericBinaryOperator::Multiply,
                rhs: Bool(true)
            }
            .into())
        );
    }

    #[test]
    fn try_divide() {
        let base = 1.0_f64;

        assert!(matches!(base.try_divide(&I8(1)), Ok(F64(x)) if (x - 1.0).abs() < f64::EPSILON ));
        assert!(matches!(base.try_divide(&I64(1)), Ok(F64(x)) if (x - 1.0).abs() < f64::EPSILON ));
        assert!(
            matches!(base.try_divide(&F64(1.0)), Ok(F64(x)) if (x - 1.0).abs() < f64::EPSILON )
        );

        assert_eq!(
            base.try_divide(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: F64(1.0),
                operator: NumericBinaryOperator::Divide,
                rhs: Bool(true)
            }
            .into())
        );
    }

    #[test]
    fn try_modulo() {
        let base = 1.0_f64;

        assert!(matches!(base.try_modulo(&I8(1)), Ok(F64(x)) if (x - 0.0).abs() < f64::EPSILON ));
        assert!(matches!(base.try_modulo(&I64(1)), Ok(F64(x)) if (x - 0.0).abs() < f64::EPSILON ));
        assert!(
            matches!(base.try_modulo(&F64(1.0)), Ok(F64(x)) if (x - 0.0).abs() < f64::EPSILON )
        );

        assert_eq!(
            base.try_modulo(&Bool(true)),
            Err(ValueError::NonNumericMathOperation {
                lhs: F64(1.0),
                operator: NumericBinaryOperator::Modulo,
                rhs: Bool(true)
            }
            .into())
        );
    }
}