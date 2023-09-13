pub trait Messager {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messager> {
    messager: &'a T, // we need to specify lifetime of T because we don't know how long the LimitTracker will be used
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
    where T: Messager {
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messager, // same as messager: messager
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {
            self.messager.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messager.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messager.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessager {
        sent_messages: RefCell<Vec<String>>, // RefCell is used to allow us to modify the value inside the struct even though we have an immutable reference to the struct
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messager for MockMessager { // we need to implement Messager for MockMessager because LimitTracker::new() takes a reference to Messager
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message)); // we need to use borrow_mut() because we want to modify the value inside the struct
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messager.sent_messages.borrow().len(), 1); // we need to use borrow() because we only want to read the value inside the struct
    }
}