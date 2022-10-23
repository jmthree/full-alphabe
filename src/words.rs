fn combine<'a, T>(i: &T, targets: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut results = Vec::new();
    if targets.is_empty() {
        results.push(vec![i]);
    }
    for t in targets {
        let mut n = t.clone();
        n.push(i);
        results.push(n);
    }
    results
}

pub fn combinations<'a, T>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut results = vec![];
    for r in input {
        let mut partial = vec![];
        for s in r {
            partial.extend(combine(s, &results));
        }
        results = partial;
    }
    results
}

#[cfg(test)]
mod tests {
    use super::combinations;

    #[test]
    fn combinations_for_empty() {
        let results = combinations(&vec![]);

        assert!(results.is_empty());
    }

    #[test]
    fn combinations_for_single_list() {
        let results = combinations(&vec![vec!["foo"]]);

        assert!(results.contains(&vec!["foo"]));
    }

    #[test]
    fn combinations_for_two_lists_single_items() {
        let results = combinations(&vec![vec!["foo"], vec!["bar"]]);

        assert!(results.contains(&vec!["foo", "bar"]));
    }

    #[test]
    fn combinations_for_two_lists_two_items() {
        let results = combinations(&vec![vec!["foo", "bar"], vec!["baz", "beep"]]);

        assert!(results.contains(&vec!["foo", "baz"]));
        assert!(results.contains(&vec!["foo", "beep"]));
        assert!(results.contains(&vec!["bar", "baz"]));
        assert!(results.contains(&vec!["bar", "beep"]));
    }

    #[test]
    fn combinations_for_two_lists_differing_items() {
        let results = combinations(&vec![vec!["foo", "bar"], vec!["baz"]]);

        assert!(results.contains(&vec!["foo", "baz"]));
        assert!(results.contains(&vec!["bar", "baz"]));
    }
}
