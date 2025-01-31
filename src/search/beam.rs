/// Beam search
pub struct BeamSearch<N, SC, S, IN>
where
    SC: Ord + Clone,
    S: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, SC)>,
{
    beam_size: usize,
    successors: S,
    cur: Vec<(N, SC)>,
    next: Vec<(N, SC)>,
}

impl<N, SC, S, IN> BeamSearch<N, SC, S, IN>
where
    SC: Ord + Clone,
    S: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, SC)>,
{
    pub fn new(beam_size: usize, start: IN, successors: S) -> Self {
        assert!(beam_size > 0, "Beam size cannot be 0.");

        let mut cur = Vec::with_capacity(beam_size);
        let next = Vec::with_capacity(beam_size);

        cur.extend(start);

        Self {
            cur,
            next,
            beam_size,
            successors,
        }
    }

    pub fn beam_size(&self) -> usize {
        self.beam_size
    }
}

impl<N, SC, S, IN> Iterator for BeamSearch<N, SC, S, IN>
where
    SC: Ord + Clone,
    S: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, SC)>,
{
    type Item = (N, SC);

    fn next(&mut self) -> Option<(N, SC)> {
        loop {
            if let Some((cur, score)) = self.cur.pop() {
                for next in (self.successors)(&cur) {
                    self.next.push(next);
                }

                return Some((cur, score));
            }

            // Truncate the beam if it is too wide
            if self.next.len() > self.beam_size {
                self.next
                    .select_nth_unstable_by_key(self.beam_size, |(_, score)| {
                        std::cmp::Reverse(score.clone())
                    });

                self.next.truncate(self.beam_size);
            }

            std::mem::swap(&mut self.cur, &mut self.next);

            if self.cur.is_empty() {
                break;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_beam_search() {
        let mut seen = Set::new(0u32);

        let successors = |n: &i32| {
            if n.abs() < 5 && !seen.test_bit((5 + n) as u32) {
                seen.set_bit((5 + n) as u32);
                vec![(n + 1, 2), (n - 1, 1)]
            } else {
                vec![]
            }
        };

        let mut bs = BeamSearch::new(1, vec![(0, 0)], successors);
        let mut visited_states = Vec::new();

        while let Some((cur, _score)) = bs.next() {
            visited_states.push(cur);
        }

        assert_eq!(visited_states, vec![0, 1, 2, 3, 4, 5]);
    }
}
