pub trait FallBackOn<I> {
    fn fallback(self, then: I) -> FallBack<I>;
}

pub struct FallBack<I> {
    first: I,
    then: I,
}

impl<I, T> FallBackOn<I> for I
where
    I: Iterator<Item = T>,
{
    fn fallback(self, then: I) -> FallBack<I> {
        FallBack { first: self, then }
    }
}

impl<I, T> Iterator for FallBack<I>
where
    I: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.first.next().or(self.then.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3]
                .into_iter()
                .fallback(vec![0, 0, 0, 4, 5].into_iter())
                .collect::<Vec<_>>()
        )
    }
}
