use super::frame::Frame;

const DEFAULT_STACK_DEPTH : usize = 2_500_000;

pub struct CallStack {
    pub stack : Vec<Frame>,
    pub max_depth : usize
}

impl CallStack {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn push(&mut self, frame : Frame) {
        self.stack.push(frame);
    }
}

impl Default for CallStack {
    fn default() -> Self {
        let max_depth = DEFAULT_STACK_DEPTH;
        Self {
            stack: Vec::with_capacity(max_depth),
            max_depth
        }
    }
}
