use inquire::{error::CustomUserError, required, Text};

fn main() {
    let answer = Text::new("What exercise would you like to do today?")
        .with_autocomplete(&suggester)
        .with_validator(required!())
        .prompt();

    match answer {
        Ok(answer) => println!("Hello {}", answer),
        Err(_) => println!("No answer provided"),
    }
}

fn suggester(val: &str) -> Result<Vec<String>, CustomUserError> {
    let suggestions = [
      "merge-strings-alternately",
      "greatest-common-divisor-of-strings",
      "kids-with-the-greatest-number-of-candies",
      "can-place-flowers",
      "reverse-vowels-of-a-string",
      "reverse-words-in-a-string",
      "product-of-array-except-self",
      "increasing-triplet-subsequence",
      "string-compression",
      "move-zeroes",
      "is-subsequence",
      "container-with-most-water",
      "max-number-of-k-sum-pairs",
      "maximum-average-subarray-i",
      "maximum-number-of-vowels-in-a-substring-of-given-length",
      "max-consecutive-ones-iii",
      "longest-subarray-of-1s-after-deleting-one-element",
      "find-the-highest-altitude",
      "find-pivot-index",
      "find-the-difference-of-two-arrays",
      "unique-number-of-occurrences",
      "determine-if-two-strings-are-close",
      "equal-row-and-column-pairs",
      "removing-stars-from-a-string",
      "asteroid-collision",
      "decode-string",
      "number-of-recent-calls",
      "dota2-senate",
      "delete-the-middle-node-of-a-linked-list",
      "odd-even-linked-list",
      "reverse-linked-list",
      "maximum-twin-sum-of-a-linked-list",
      "maximum-depth-of-binary-tree",
      "leaf-similar-trees",
      "count-good-nodes-in-binary-tree",
      "path-sum-iii",
      "longest-zigzag-path-in-a-binary-tree",
      "lowest-common-ancestor-of-a-binary-tree",
      "binary-tree-right-side-view",
      "maximum-level-sum-of-a-binary-tree",
      "search-in-a-binary-search-tree",
      "delete-node-in-a-bst",
      "keys-and-rooms",
      "number-of-provinces",
      "reorder-routes-to-make-all-paths-lead-to-the-city-zero",
      "evaluate-division",
      "nearest-exit-from-entrance-in-maze",
      "rotting-oranges",
      "kth-largest-element-in-an-array",
      "smallest-number-in-infinite-set",
      "maximum-subsequence-score",
      "total-cost-to-hire-k-workers",
      "guess-number-higher-or-lower",
      "successful-pairs-of-spells-and-potions",
      "find-peak-element",
      "koko-eating-bananas",
      "letter-combinations-of-a-phone-number",
      "combination-sum-iii",
      "n-th-tribonacci-number",
      "min-cost-climbing-stairs",
      "house-robber",
      "domino-and-tromino-tiling",
      "unique-paths",
      "longest-common-subsequence",
      "best-time-to-buy-and-sell-stock-with-transaction-fee",
      "edit-distance",
      "counting-bits",
      "single-number",
      "minimum-flips-to-make-a-or-b-equal-to-c",
      "implement-trie-prefix-tree",
      "search-suggestions-system",
      "non-overlapping-intervals",
      "minimum-number-of-arrows-to-burst-balloons",
      "daily-temperatures",
      "online-stock-span"
    ];

    let val_lower = val.to_lowercase();

    Ok(suggestions
        .iter()
        .filter(|s| s.to_lowercase().contains(&val_lower))
        .map(|s| String::from(*s))
        .collect())
}
