const QUEUE_SIZE: usize = 1024;
const DELETION_SIZE: usize = 16;

pub struct Queue<T>
where
    T: Copy,
{
    list: [T; QUEUE_SIZE],
    start: usize,
    end: usize,
}

impl<T> Queue<T>
where
    T: Copy,
{
    pub fn new(init: T) -> Queue<T> {
        Queue {
            list: [init; QUEUE_SIZE],
            start: 0,
            end: 0,
        }
    }

    fn shift_back(&mut self, amount: usize) {
        if self.start < amount {
            return;
        }

        for i in amount..QUEUE_SIZE {
            self.list[i - amount] = self.list[i];
        }

        self.start -= amount;
        self.end -= amount;
    }

    pub fn queue(&mut self, t: T) {
        if self.end == QUEUE_SIZE {
            if self.start > 0 {
                let start = self.start;
                self.shift_back(start);
            } else {
                self.start = DELETION_SIZE;
                let start = self.start;
                self.shift_back(start);
            }
        }

        self.list[self.end] = t;
        self.end += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.start == self.end {
            return None;
        }

        let t = self.list[self.start];
        self.start += 1;
        Some(t)
    }
}
