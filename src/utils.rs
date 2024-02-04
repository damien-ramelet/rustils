use std::cmp::Ordering;

#[derive(Debug)]
struct NegInf;

impl Eq for NegInf { }

impl PartialEq<Self> for NegInf{
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl PartialEq<usize> for NegInf {
    fn eq(&self, _other: &usize) -> bool {
        false
    }
}

impl PartialEq<NegInf> for usize {
    fn eq(&self, _other: &NegInf) -> bool {
        false
    }
}

impl PartialOrd<usize> for NegInf{
    fn partial_cmp(&self, _other: &usize) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl PartialOrd<NegInf> for usize {
    fn partial_cmp(&self, _other: &NegInf) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl PartialOrd<NegInf> for NegInf {
    fn partial_cmp(&self, _other: &NegInf) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}
#[derive(Debug)]
struct PosInf;

impl PartialEq for PosInf{
    fn eq(&self, _other: &PosInf) -> bool {
        false
    }
}

impl PartialEq<usize> for PosInf{
    fn eq(&self, _other: &usize) -> bool {
        false
    }
}

impl PartialEq<PosInf> for usize{
    fn eq(&self, _other: &PosInf) -> bool {
        false
    }
}

impl Eq for PosInf{ }

impl PartialOrd<usize> for PosInf{
    fn partial_cmp(&self, _other: &usize) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl PartialOrd<PosInf> for usize {
    fn partial_cmp(&self, _other: &PosInf) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl PartialOrd<PosInf> for PosInf{
    fn partial_cmp(&self, _other: &PosInf) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}


#[cfg(test)]
mod tests {
    use crate::utils::NegInf;
    use crate::utils::PosInf;
    #[test]
    fn test_negative_infinite_is_almays_lower() {
        assert!(1 > NegInf);
        assert!(NegInf < 2);
        assert_ne!(NegInf, NegInf);
        assert!(NegInf < NegInf);
    }

    #[test]
    fn test_positive_infinite_is_always_greater() {
        assert!(PosInf > 1);
        assert!(10 < PosInf);
        assert_ne!(PosInf, PosInf);
        assert!(PosInf > PosInf);
    }
}
