/// Simple circular buffer implementation
///
/// This is actually a wrapper around `Vec<T>`
pub struct CircularQueue<T> {
    /// The elements within this buffer
    elements: Vec<T>,
    /// The current position within the buffer
    cursor: usize,
}

impl<T> CircularQueue<T> {
    /// Create a new buffer, defaulting the cursor to the start of `elements`
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            cursor: 0,
            elements,
        }
    }

    /// Get the next element
    pub fn next(&mut self) -> &T {
        if !self.is_cursor_valid() {
            self.cursor = 0;
        }

        self.elements.get(self.cursor)
            .inspect(|_| self.cursor += 1)
            .expect(&format!(
                    "Could not get current value, cursor: '{}' element_count: '{}'",
                    self.cursor,
                    self.elements.len()
            ))
    }

    fn is_cursor_valid(&self) -> bool {
        let el_count = self.elements.len();

        //TODO: no elements this breaks
        return self.cursor <= el_count && self.cursor > 0
    }
}

impl<T> From<Vec<T>> for CircularQueue<T> {
    fn from(value: Vec<T>) -> Self {
        Self::new(value)
    }
}

