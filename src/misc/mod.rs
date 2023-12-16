use std::hash::Hash;

use crate::prelude::HashMap;

pub fn path_contraction<N, FN, IN>(start: &N, mut successor: FN, iterations: usize) -> N
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> N,
{
    let mut memo: HashMap<N, (N, usize)> = HashMap::default();
    let mut cur = start.clone();
    let mut cur_iter = 0;

    loop {
        if cur_iter == iterations {
            return cur;
        }

        // Step 1
        let (next1, iters_step1) = if let Some((next1, length1)) = memo.get(&cur) {
            (next1.clone(), *length1)
        } else {
            let next1 = successor(&cur);
            (next1, 1)
        };

        // Step 2
        let (next2, iters_step2) = if let Some((next2, length2)) = memo.get(&next1) {
            (next2.clone(), *length2)
        } else {
            let next2 = successor(&next1);
            (next2, 1)
        };

        // Combine
        if cur_iter + iters_step1 + iters_step2 <= iterations {
            memo.insert(cur, (next2.clone(), iters_step1 + iters_step2));
            cur = next2;
            cur_iter += iters_step1 + iters_step2;
        } else if cur_iter + iters_step1 <= iterations {
            memo.insert(cur, (next1.clone(), iters_step1));
            cur = next1;
            cur_iter += iters_step1;
        } else {
            memo.insert(cur.clone(), (successor(&cur), cur_iter + 1));
            cur_iter += 1;
        }
    }
}
