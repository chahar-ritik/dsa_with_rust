use std::collections::VecDeque;

fn main() {
    let result = bfs(3, 5, 4);
    println!("Minimum steps = {}", result);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn bfs(cap1: usize, cap2: usize, target: usize) -> i32 {
  
    if target > cap1.max(cap2) || target % gcd(cap1, cap2) != 0 {
        return -1;
    }

    let mut visited = vec![vec![false; cap2 + 1]; cap1 + 1];
    let mut queue = VecDeque::new();

    queue.push_back((0, 0, 0));

    while let Some((a, b, step)) = queue.pop_front() {

        if visited[a][b] {
            continue;
        }
        visited[a][b] = true;

        if a == target || b == target {
            return step;
        }

        // fill A
        queue.push_back((cap1, b, step + 1));

        // fill B
        queue.push_back((a, cap2, step + 1));

        // empty A
        queue.push_back((0, b, step + 1));

        // empty B
        queue.push_back((a, 0, step + 1));

        // pour A -> B
        let pour = std::cmp::min(a, cap2 - b);
        queue.push_back((a - pour, b + pour, step + 1));

        // pour B -> A
        let pour = std::cmp::min(b, cap1 - a);
        queue.push_back((a + pour, b - pour, step + 1));
    }

    -1
}