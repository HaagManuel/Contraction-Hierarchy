use std::ops::{Index, IndexMut};



// stores data in a vector and timestamps in a separate vectors
// all entries where time != current_time are considered empty and the default value is returned
pub struct TimestampedVector<T> {
    data: Vec<T>,
    timestamps: Vec<u32>,
    current_time: u32,
    default: T
}

impl<T: Copy> TimestampedVector<T> {
    pub fn new(size: usize, default_value: T) -> Self {
        TimestampedVector {
            data: vec![default_value; size],
            timestamps: vec![0; size],
            current_time: 1, //initially everything is empty
            default: default_value
        }
    }
    
    //does not need branch -> TODO test difference
    pub fn set(&mut self, index: usize, value: T) {
        self.data[index] = value;
        self.timestamps[index] = self.current_time;
    }

    pub fn reset(&mut self) {
        let (new_value, overflow) = self.current_time.overflowing_add(1);
        self.current_time = new_value;
        //reset manually
        if overflow { 
            self.data.fill(self.default);
        }
    }

    pub fn clone_data(&mut self) -> Vec<T> {
        //clean data before clone, otherwise invalid data after clone
        for i in 0..self.data.len() {
            if self.timestamps[i] != self.current_time {
               self.data[i] = self.default.clone(); 
            }
        }
        self.data.clone()
    }
}


impl<T: Clone> Index<usize> for TimestampedVector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if self.timestamps[index] == self.current_time {
            return &self.data[index]
        } 
        &self.default
    }
}

impl<T: Clone> IndexMut<usize> for TimestampedVector<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if self.timestamps[index] != self.current_time {
            self.timestamps[index] = self.current_time;
            self.data[index] = self.default.clone(); //must reset element before giving out mut reference
        }
        &mut self.data[index]
    }
}