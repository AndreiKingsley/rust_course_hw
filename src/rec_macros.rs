use std::collections::VecDeque;
use std::ops::Index;

macro_rules! recurrence {
    ($seq:ident [$ind:ident]: $sty:ty = $($inits:expr),*;...;$nth_expr:expr) => {{
        struct Recurrence<T> {
            buf: VecDeque<T>,
            cur_index: usize,
            buf_size: usize,
        }

        impl<T> Recurrence<T> {
            fn new() -> Self {
                let buf = VecDeque::new();
                Self {
                    buf: Default::default(),
                    cur_index: 0,
                    buf_size: 0,
                }
            }

            fn add_init(&mut self, init_val: T) {
                self.buf.push_back(init_val)
            }

            fn set_size(&mut self) {
                self.buf_size = self.buf.len()
            }

            fn push(&mut self, val: T) {
                self.buf.pop_front();
                self.buf.push_back(val);
                self.cur_index += 1;
            }
        }

        impl<T> Index for Recurrence<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.buf[index-self.cur_index]
            }
        }

        impl<T> Iterator for Recurrence<T> {
            type Item = T;

            fn next(&mut self) -> Option<Self::Item> {
                let $ind = self.cur_index;
                if $ind < self.buf_size {
                    Some(self.buf[$ind])
                } else {
                    let $seq = &mut self.buf;
                    let next_val = $nth_expr
                    $seq.push(next_val);
                    Some(next_val)
                }
            }
        }

        let rec = Reccurence::new()
        $(
            {
                rec.add_init($inits)
            }
        )*
        rec.set_size()

        rec
    }};
}
