use std::collections::VecDeque;
use std::ops::Index;

macro_rules! recurrence {
    ($seq:ident [$ind:ident]: $sty:ty = $($inits:expr),*;...;$nth_expr:expr) => {{
        struct Recurrence {
            buf: VecDeque<$sty>,
            cur_index: usize,
            buf_size: usize,
        }

        impl Recurrence {
            fn new() -> Self {
                Self {
                    buf: Default::default(),
                    cur_index: 0,
                    buf_size: 0,
                }
            }

            fn add_init(&mut self, init_val: $sty) {
                self.buf.push_back(init_val)
            }

            fn set_size(&mut self) {
                self.buf_size = self.buf.len()
            }

            fn push(&mut self, val: $sty) {
                //self.buf.pop_front();
                self.buf.push_back(val);
                self.cur_index += 1;
            }
        }

        impl Index<usize> for Recurrence {
            type Output = $sty;

            fn index(&self, index: usize) -> &Self::Output {
                &self.buf[index]
            }
        }

        impl Iterator for Recurrence {
            type Item = $sty;

            fn next(&mut self) -> Option<Self::Item> {
                let $ind = self.cur_index;
                let res;
                if $ind < self.buf_size {
                    res = self.buf[$ind];
                    self.cur_index += 1;
                } else {
                    let $seq = self;
                    let next_val = $nth_expr;
                    $seq.push(next_val);
                    res = next_val
                };
                Some(res)
            }
        }

        let mut rec = Recurrence::new();
        $(
            {
                rec.add_init($inits)
            }
        )*
        rec.set_size();

        rec
    }};
}


fn main() {
    let fib = recurrence![a[n]: i64 = 0, 1; ...; a[n-1] + a[n-2]];
    for n in fib.take(10) { print!("{} ", n); }
    // 0 1 1 2 3 5 8 13 21 34

    println!();

    let other = recurrence![f[i]: f64 = 1.0; ...; f[i-1] * i as f64];
    for n in other.take(10) { print!("{} ", n); }
    //1 1 2 6 24 120 720 5040 40320 362880

}
