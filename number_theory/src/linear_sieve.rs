use std::collections::HashSet;

// param: to >= 2
pub fn linear_sieve(to: u32) -> Vec<u32> {
    let mut composites = HashSet::new();
    let mut primes = Vec::new();

    for i in 2..=to {
        // 只有筛掉的数才认为是合数
        // 未加入的数一律假设为质数
        if !composites.contains(&i) {
            primes.push(i);
        }

        for &prime in primes.iter() {
            if i * prime > to {
                break;
            }

            // 质数一定会筛掉自己的平方
            composites.insert(i * prime);

            // 若非最小质因数筛选，则终止
            if i % prime == 0 {
                break;
            }
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::linear_sieve;

    #[test]
    fn front_100() {
        assert_eq!(
            linear_sieve(100),
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97,
            ]
        );
    }

    #[test]
    fn illegal_usage() {
        assert_eq!(linear_sieve(0), []);
        assert_eq!(linear_sieve(1), []);
    }

    #[test]
    fn once() {
        assert_eq!(linear_sieve(2), [2]);
    }
}
