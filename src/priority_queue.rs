use std::cmp::Ordering;

struct NegInf;

impl PartialEq for NegInf{
    fn eq(&self, other: &NegInf) -> bool {
        false
    }
}

impl PartialEq<usize> for NegInf { 
    fn eq(&self, other: &usize) -> bool {
        false
    }
}

impl PartialEq<NegInf> for usize {
    fn eq(&self, other: &NegInf) -> bool {
        false
    }
}

impl Eq for NegInf { }

impl PartialOrd<usize> for NegInf{
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl PartialOrd<NegInf> for usize {
    fn partial_cmp(&self, other: &NegInf) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl PartialOrd<NegInf> for NegInf {
    fn partial_cmp(&self, other: &NegInf) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

struct PosInf;

impl PartialEq for PosInf{
    fn eq(&self, other: &PosInf) -> bool {
        false
    }
}

impl PartialEq<usize> for PosInf{ 
    fn eq(&self, other: &usize) -> bool {
        false
    }
}

impl PartialEq<PosInf> for usize{
    fn eq(&self, other: &PosInf) -> bool {
        false
    }
}

impl Eq for PosInf{ }

impl PartialOrd<usize> for PosInf{
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

impl PartialOrd<PosInf> for usize {
    fn partial_cmp(&self, other: &PosInf) -> Option<Ordering> {
        Some(Ordering::Less)
    }
}

impl PartialOrd<PosInf> for PosInf{
    fn partial_cmp(&self, other: &PosInf) -> Option<Ordering> {
        Some(Ordering::Greater)
    }
}

struct PriorityQueue {
    vec: Vec<(usize, String)>,
}

impl PriorityQueue {
    fn push(&mut self, element: (usize, &str)) {
        let mut inserted = false;
        for (index, inserted_element) in self.vec.iter().enumerate() {
            let (priority, _) = inserted_element;
            let priority_to_instert = element.0;
            if priority_to_instert < *priority {
                self.vec.insert(index, (priority_to_instert, String::from(element.1)));
                inserted = true;
                break;
            }
        }
        if inserted == false {
            self.vec.insert(self.vec.len(), (element.0, String::from(element.1)));
        }
    }

    fn pop(&mut self) -> (usize, String) {
        self.vec.remove(0)
    }
}


#[cfg(test)]
mod tests {
    use crate::priority_queue::NegInf;
    use crate::priority_queue::PosInf;
    use crate::priority_queue::PriorityQueue;

    #[test]
    fn test_negative_infinite_is_almays_lower() {
        let neg_inf = NegInf { };
        assert!(1 > neg_inf);
        assert!(neg_inf < 2);
        assert!(neg_inf != neg_inf);
        assert!(neg_inf < neg_inf);
    }

    #[test]
    fn test_positive_infinite_is_always_greater() {
        let pos_inf = PosInf { };
        assert!(pos_inf > 1);
        assert!(10 < pos_inf);
        assert!(pos_inf != pos_inf);
        assert!(pos_inf > pos_inf);
    }

    #[test]
    fn test_priority_queue_insert_elements_as_intended() {
        let mut priority_queue = PriorityQueue { vec: vec![] };
        priority_queue.push((1000, "one thousand"));
        priority_queue.push((10, "ten"));
        priority_queue.push((3, "three"));
        priority_queue.push((100, "one hundred"));

        assert_eq!(priority_queue.pop(), (3, String::from("three")));
        assert_eq!(priority_queue.pop(), (10, String::from("ten")));
        assert_eq!(priority_queue.pop(), (100, String::from("one hundred")));
        assert_eq!(priority_queue.pop(), (1000, String::from("one thousand")));
    }
}
