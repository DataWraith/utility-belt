/// Brent's cycle detection algorithm
fn brents_algorithm<S, F>(mut successor: F, start: S) -> (usize, usize)
where
    F: FnMut(&S) -> S,
    S: PartialEq + Clone,
{
    // Main phase: search successive powers of two
    let mut power = 1;
    let mut cycle_length = 1;
    let mut tortoise = start.clone();
    let mut hare = successor(&start);

    while tortoise != hare {
        if power == cycle_length {
            // Start a new power of two
            tortoise = hare.clone();
            power *= 2;
            cycle_length = 0;
        }

        hare = successor(&hare);
        cycle_length += 1;
    }

    // Find the position of the first repetition of length λ
    tortoise = start.clone();
    hare = start;

    // Move the hare until the distance between the hare and tortoise is λ.
    for _ in 0..cycle_length {
        hare = successor(&hare);
    }

    // Now move the tortoise and hare at same speed until they agree
    let mut cycle_start = 0;

    while tortoise != hare {
        tortoise = successor(&tortoise);
        hare = successor(&hare);
        cycle_start += 1;
    }

    (cycle_start, cycle_length)
}

