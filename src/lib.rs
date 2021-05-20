#[derive(Debug)]
pub struct Dollar {
}

#[derive(Debug)]
pub struct Franc {
}

struct Money {
    amount: u32
}

impl Money {
    pub fn times(&self, multiplier: u32) -> Money {
        Money { amount: self.amount * multiplier }
    }
    pub fn equals(&self, target: Money) -> bool {
        self.amount == target.amount
    }
    pub fn dollar(amount: u32) -> Money {
        Money { amount: amount }
    }
    pub fn franc(amount: u32) -> Money {
        Money { amount: amount }
    }
}

trait MoneyTrait {
    fn new(amount: u32) -> Money;
}

impl MoneyTrait for Dollar {
    fn new(amount: u32) -> Money {
        Money { amount: amount }
    }
}

impl Franc {
    fn new(amount: u32) -> Money {
        Money { amount: amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert!(Money::dollar(10).equals(five.times(2)));
        assert!(Money::dollar(15).equals(five.times(3)));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Money::franc(5);
        assert!(Money::franc(10).equals(five.times(2)));
        assert!(Money::franc(15).equals(five.times(3)));
    }

    #[test]
    fn test_equality() {
        assert!(Money::dollar(5).equals(Money::dollar(5)));
        assert!(!Money::dollar(5).equals(Money::dollar(6)));
        assert!(Money::franc(5).equals(Money::franc(5)));
        assert!(!Money::franc(5).equals(Money::franc(6)));
        //TODO assert!(!Franc::new(5).equals(Dollar::new(5)));
    }
}
