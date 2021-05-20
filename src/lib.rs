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
        let five = Dollar::new(5);
        assert!(Dollar::new(10).equals(five.times(2)));
        assert!(Dollar::new(15).equals(five.times(3)));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert!(Franc::new(10).equals(five.times(2)));
        assert!(Franc::new(15).equals(five.times(3)));
    }

    #[test]
    fn test_equality() {
        assert!(Dollar::new(5).equals(Dollar::new(5)));
        assert!(!Dollar::new(5).equals(Dollar::new(6)));
        assert!(Franc::new(5).equals(Franc::new(5)));
        assert!(!Franc::new(5).equals(Franc::new(6)));
        assert!(!Franc::new(5).equals(Dollar::new(5)));
    }
}
