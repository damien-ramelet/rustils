use std::cmp::Ordering;

#[derive(Debug)]
#[allow(dead_code)]
enum PriorityScore {
    Int { value: usize},
    NegInf,
    PosInf,
}

impl PartialEq for PriorityScore {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (PriorityScore::Int {value}, PriorityScore::Int {value: other_value}) => value.eq(other_value),
            _ => false,
        }
    }
}

impl PartialOrd for PriorityScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self, other) {
            (PriorityScore::NegInf, _) => Some(Ordering::Less),
            (PriorityScore::PosInf, _) => Some(Ordering::Greater),
            (PriorityScore::Int { value}, PriorityScore::Int{value: other_value}) => value.partial_cmp(other_value),
            (PriorityScore::Int { value: _ }, PriorityScore::NegInf) => Some(Ordering::Greater),
            (PriorityScore::Int { value: _}, PriorityScore::PosInf) => Some(Ordering::Less),
        }
    }
}

#[derive(Debug)]
struct PriorityQueue {
    vec: Vec<(PriorityScore, String)>,
}

#[allow(dead_code)]
impl PriorityQueue {
    fn push(&mut self, element: (PriorityScore, &str)) {
        let length = self.vec.len();
        if length == 0 {
            self.vec.insert(0, (element.0, String::from(element.1)));
            return
        }
        for (index, inserted_element) in self.vec.iter().enumerate() {
            let (priority, _) = inserted_element;
            if element.0 < *priority {
                self.vec.insert(index, (element.0, String::from(element.1)));
                break
            } else if index + 1 == length {
                self.vec.insert(length, (element.0, String::from(element.1)));
                break
            }
        }
    }

    fn pop(&mut self) -> (PriorityScore, String) {
        self.vec.remove(0)
    }
}


#[cfg(test)]
mod tests {
    use crate::priority_queue::{PriorityQueue, PriorityScore};
    #[test]
    fn test_priority_queue_insert_elements_as_intended() {
        let mut priority_queue = PriorityQueue { vec: vec![] };
        priority_queue.push((PriorityScore::PosInf, "positive infinite"));
        priority_queue.push((PriorityScore::NegInf, "negative infinite"));
        priority_queue.push((PriorityScore::NegInf, "negative infinite"));
        priority_queue.push((PriorityScore::Int { value: 100}, "one hundred"));
        priority_queue.push((PriorityScore::Int { value: 10}, "ten"));
        priority_queue.push((PriorityScore::Int { value: 3}, "three"));
        priority_queue.push((PriorityScore::PosInf, "positive infinite"));
        priority_queue.push((PriorityScore::Int { value: 1000}, "one thousand"));
        priority_queue.push((PriorityScore::NegInf, "negative infinite"));
        priority_queue.push((PriorityScore::NegInf, "negative infinite"));

        assert_eq!(priority_queue.pop().1, String::from("negative infinite"));
        assert_eq!(priority_queue.pop().1, String::from("negative infinite"));
        assert_eq!(priority_queue.pop().1, String::from("negative infinite"));
        assert_eq!(priority_queue.pop().1, String::from("negative infinite"));
        assert_eq!(priority_queue.pop(), (PriorityScore::Int{value: 3}, String::from("three")));
        assert_eq!(priority_queue.pop(), (PriorityScore::Int{value: 10}, String::from("ten")));
        assert_eq!(priority_queue.pop(), (PriorityScore::Int{value: 100}, String::from("one hundred")));
        assert_eq!(priority_queue.pop(), (PriorityScore::Int{value :1000}, String::from("one thousand")));
        assert_eq!(priority_queue.pop().1, String::from("positive infinite"));
        assert_eq!(priority_queue.pop().1, String::from("positive infinite"));
    }
}
