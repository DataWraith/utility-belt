/// Beam search
pub struct BeamSearch<N, C>
where
    N: Clone,
    C: Ord + Clone,
{
    cur: Vec<(N, C)>,
    next: Vec<(N, C)>,
    beam_size: usize,
}

impl<N, C> BeamSearch<N, C>
where
    N: Clone,
    C: Ord + Clone,
{
    pub fn new<IN>(beam_size: usize, start: IN) -> Self
    where
        IN: IntoIterator<Item = (N, C)>,
    {
        assert!(beam_size > 0, "Beam size cannot be 0.");

        let mut cur = Vec::with_capacity(beam_size);
        let next = Vec::with_capacity(beam_size);

        cur.extend(start);

        Self {
            cur,
            next,
            beam_size,
        }
    }

    pub fn beam_size(&self) -> usize {
        self.beam_size
    }

    pub fn next<S, IN>(&mut self, mut successors: S) -> Option<(N, C)>
    where
        S: FnMut(&N) -> IN,
        IN: IntoIterator<Item = (N, C)>,
    {
        loop {
            if let Some((cur, score)) = self.cur.pop() {
                for next in successors(&cur) {
                    self.next.push(next.clone());
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
    use crate::prelude::bv;

    #[test]
    fn test_beam_search() {
        let mut seen = bv::bitarr![u32, bv::Lsb0; 0; 1];
        seen.set(0, true);

        let mut successors = |n: &i32| {
            if n.abs() < 5 && seen.get((5 + n) as usize).as_deref() == Some(&false) {
                seen.set((5 + n) as usize, true);
                vec![(n + 1, 2), (n - 1, 1)]
            } else {
                vec![]
            }
        };

        let mut bs = BeamSearch::new(1, vec![(0, 0)]);
        let mut visited_states = Vec::new();

        while let Some((cur, _score)) = bs.next(&mut successors) {
            visited_states.push(cur);
        }

        assert_eq!(visited_states, vec![0, 1, 2, 3, 4, 5]);
    }
}
