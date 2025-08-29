pub struct MemRingBuf {
    buf: Vec<u8>,
    start: usize,
    end: usize,
    full: bool,
}

impl MemRingBuf {
    pub fn new(capacity: usize) -> Self {
        MemRingBuf {
            buf: vec![0; capacity],
            start: 0,
            end: 0,
            full: false,
        }
    }

    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    pub fn len(&self) -> usize {
        if self.full {
            self.buf.len()
        } else if self.end >= self.start {
            self.end - self.start
        } else {
            self.buf.len() - self.start + self.end
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.full && (self.start == self.end)
    }

    pub fn is_full(&self) -> bool {
        self.full
    }

    pub fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
        self.full = false;
    }

    pub fn try_write(&mut self, data: &[u8]) -> bool {
        if self.remaining_space() < data.len() {
            return false;
        }
        let (first, second) = self.remaining_space_slice();
        if data.len() <= first.len() {
            first[..data.len()].copy_from_slice(data);
            self.end = (self.end + data.len());
            if self.end == self.start {
                self.full = true;
            }
            return true;
        }

        true
    }

    // 返回剩余空间
    pub fn remaining_space(&self) -> usize {
        self.buf.len() - self.len()
    }

    // 获取剩余空间位置
    fn remaining_space_slice(&mut self) -> (&mut [u8], Option<&mut [u8]>) {
        if self.full {
            (&mut [], None)
        } else if self.start == 0 {
            (&mut self.buf[self.end..], None)
        } else if self.end >= self.start {
            let (left, right) = self.buf.split_at_mut(self.start);
            (&mut right[self.end - self.start..], Some(left))
        } else {
            (&mut self.buf[self.end..self.start], None)
        }
    }
}
