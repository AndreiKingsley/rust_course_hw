use std::ops::Rem;

pub struct Matcher<T: ToString + Copy, S: ToString> {
    predicate: fn(T) -> bool,
    substitute_with: S,
}

impl<T: ToString + Copy, S: ToString> Matcher<T, S> {
    pub fn new(_predicate: fn(T) -> bool, _substitute_with: S) -> Matcher<T, S> {
        Self {
            predicate: _predicate,
            substitute_with: _substitute_with,
        }
    }

    pub fn make_match(&self, value: T) -> String {
        let status: bool = (self.predicate)(value);
        if status {
            self.substitute_with.to_string()
        } else {
            "".to_string()
        }
    }
}

impl<T: 'static + ToString + Copy, S: 'static + ToString> Default for Fizzy<T, S> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Fizzy<T: ToString + Copy, S: 'static + ToString> {
    matchers: Vec<Matcher<T, S>>,
}

impl<T: 'static + ToString + Copy, S: 'static + ToString> Fizzy<T, S> {
    pub fn new() -> Self {
        Self { matchers: vec![] }
    }

    // можете использовать `mut self` в сигнатуре, если вам нравится
    pub fn add_matcher(self, _matcher: Matcher<T, S>) -> Self {
        let mut matchers = self.matchers;
        matchers.push(_matcher);
        Self { matchers }
    }

    /// Применяет набор Matchers к каждому элементу итератора
    pub fn apply<I: Iterator<Item = T>>(self, _iter: I) -> impl Iterator<Item = String> {
        _iter.map(move |x| {
            let matching_res: String =
                self.matchers
                    .iter()
                    .fold("".to_string(), |mut acc, matcher| {
                        acc.push_str(&*matcher.make_match(x));
                        acc
                    });
            if matching_res.is_empty() {
                x.to_string()
            } else {
                matching_res
            }
        })
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T, String>
where
    T: 'static + ToString + Copy + Rem<Output = T> + From<u8> + std::cmp::PartialEq,
{
    Fizzy {
        matchers: vec![
            Matcher {
                predicate: |x| x % T::from(3) == T::from(0),
                substitute_with: "fizz".to_string(),
            },
            Matcher {
                predicate: |x| x % T::from(5) == T::from(0),
                substitute_with: "buzz".to_string(),
            },
        ],
    }
}
