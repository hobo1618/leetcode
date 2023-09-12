fn main() {

    // You are given two strings word1 and word2. 
    // Merge the strings by adding letters in alternating order, starting with word1.
    // If a string is longer than the other, append the additional letters onto the end of the merged string.
    // 
    // Return the merged string.


    //     Example 1
    //     Input: word1 = "abc", word2 = "pqr"
    //     Output: "apbqcr"
    //
    //     Example 2
    //     Input: word1 = "ab", word2 = "pqrs"
    //     Output: "apbqrs"
    //
    //     Example 3
    //     Input: word1 = "abcd", word2 = "pq"
    //     Output: "apbqcd"
    //
    //     Constraints:
    //     1 <= word1.length, word2.length <= 100
    //     word1 and word2 consist of lowercase English letters.

    fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();
        let mut word1 = word1.chars();
        let mut word2 = word2.chars();
        loop {
            match (word1.next(), word2.next()) {
                (Some(c1), Some(c2)) => {
                    result.push(c1);
                    result.push(c2);
                },
                (Some(c1), None) => {
                    result.push(c1);
                    result.push_str(&word1.collect::<String>());
                    break;
                },
                (None, Some(c2)) => {
                    result.push(c2);
                    result.push_str(&word2.collect::<String>());
                    break;
                },
                (None, None) => break,
            }
        }

        result
    }

    let res = merge_alternately(String::from("poiu"), String::from("abdefghij"));

    println!("res is {}", res);
}
