/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // Always push to q1.
        self.q1.enqueue(elem);
    }

    pub fn pop(&mut self) -> Result<T, &str> {
     // Ensure q2 is empty before starting.
     while self.q2.dequeue().is_ok() {}

     // If both queues are empty, the stack is empty.
     if self.q1.is_empty() {
         return Err("Stack is empty");
     }
 
     // Move all elements from q1 to q2, except the last one.
     while self.q1.size() > 1 {
         let elem = self.q1.dequeue().unwrap();
         self.q2.enqueue(elem);
     }
 
     // The last element in q1 is the one we need to pop.
     let last_elem = self.q1.dequeue().unwrap();
 
     // Swap the names of q1 and q2 so that q1 is always the one with elements.
     std::mem::swap(&mut self.q1, &mut self.q2);
 
     // Return the last element from the original q1.
     Ok(last_elem)
    }

    pub fn is_empty(&self) -> bool {
        // The stack is empty if q1 is empty (no elements to pop).
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}