
pub fn unique_strings(input: Vec<&str>) -> Vec<&str> {
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();
    for s in input {
        if seen.insert(s) {
            result.push(s);
        }
    }
    result
}

pub fn unique_strings_with_count(input: Vec<&str>, min_count: i32) -> Vec<(&str, i32)> {
    let mut count_map: std::collections::HashMap<&str, i32> =
        std::collections::HashMap::with_capacity(input.len());
    // Count occurrences with improved HashMap initialization
    for s in input {
        *count_map.entry(s).or_insert(0) += 1;
    }
    // Filter the results before sorting to reduce overhead
    let mut result: Vec<_> = count_map
        .into_iter()
        .filter(|&(_, count)| count >= min_count)
        .collect();
    // Sort the filtered results
    result.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    result
}

trait AllFill {
    // トレイトメソッドを定義します。
    fn all_fill<F>(&self, f: F) -> Vec<&str>
    where
        F: Fn(&String) -> &str;
}

// Vec<String> に対する実装を行います。
impl AllFill for Vec<String> {
    fn all_fill<F>(&self, f: F) -> Vec<&str>
    where
        F: Fn(&String) -> &str,
    {
        self.iter().map(f).collect()
    }
}

trait VecToStr {
    fn vec_to_str(&self) -> Vec<&str>;
}

impl VecToStr for Vec<String> {
    fn vec_to_str(&self) -> Vec<&str> {
        self.iter().map(|v| v.as_str()).collect()
    }
}
