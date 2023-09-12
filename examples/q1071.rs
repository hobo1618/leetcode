fn gcd_of_strings(str1: String, str2: String) -> String {
    let (mut str1, mut str2) = (str1, str2);

        // ensures str1.len() >= str2.len()
    if str1.len() < str2.len() {
        std::mem::swap(&mut str1, &mut str2);
    }

    let mut str1 = str1.as_bytes();
    let mut str2 = str2.as_bytes();

    loop {
        if str1.starts_with(str2) {
            // ABABABAB starts with ABABAB
            str1 = &str1[str2.len()..];
            if str1.is_empty() {
                break
            }
            // now AB < ABABAB
            // so swap => 
                // str1 = ABABAB
                // str2 = AB
            if str1.len() < str2.len() {
                std::mem::swap(&mut str1, &mut str2);
            }
        } else {
            return "".to_string()
        }
    }
    
    String::from_utf8(str2.to_vec()).unwrap()
}

fn main() {
        // For two strings s and t, we say "t divides s"
        // if and only if s = t + ... + t (i.e., t is concatenated with itself 
        // one or more times).
        // 
        // Given two strings str1 and str2,
        // return the largest string x such that x divides both str1 and str2.
        // 
        // Example 1:
        // 
        // Input: str1 = "ABCABC", str2 = "ABC"
        // Output: "ABC"
        // 
        // Example 2:
        // 
        // Input: str1 = "ABABAB", str2 = "ABAB"
        // Output: "AB"
        // 
        // Example 3:
        // 
        // Input: str1 = "LEET", str2 = "CODE"
        // Output: ""


    {
        // let str1 = "ABCABC".to_string();
        // let str2 = "ABC".to_string();
        // let result = gcd_of_strings(str1, str2);
        // println!("result = {:?}", result);
    }

    {
        let str1 = "AB".to_string();
        let str2 = "ABAB".to_string();
        // let str1 = "CODE".to_string();
        // let str2 = "LEET".to_string();
        let result = gcd_of_strings(str1, str2);
        println!("result = {:?}", result);  

    }
}
