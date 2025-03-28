// 557. Reverse Words in a String III: https://leetcode.com/problems/reverse-words-in-a-string-iii

fn main() {
    println!(
        "{}",
        reverse_words("Let's take LeetCode contest".to_string())
    );
}

/* I had already solved this problem in typescript before.
    It was through this problem that I realized the difference between C and JavaScript.
    In JavaScript, the solution with the best time complexity for the problem is actually the worst in practice,
    while the best-performing solution relies on built-in functions that may have a higher time complexity—but one that runs in C.
    My advice in JavaScript master built-in functions.

    My solutions
        - Typescript: https://leetcode.com/problems/reverse-words-in-a-string-iii/submissions/1511440804/
            Time complexity: O(n)
            Space complexity: O(n)

        - Rust: https://leetcode.com/problems/reverse-words-in-a-string-iii/submissions/1589355422/
            Time complexity: O(n)
            Space complexity: O(1)
*/
fn reverse_words(s: String) -> String {
    let mut s = s;
    let bytes = unsafe { s.as_bytes_mut() };

    let mut i = 0;
    let mut j = 0;

    while i <= bytes.len() {
        match bytes.get(i) {
            // Some('whitespace')
            Some(32) => {
                reverse_word(&mut bytes[j..i]);
                j = i + 1
            }
            None => {
                reverse_word(&mut bytes[j..]);
                break;
            }
            _ => {}
        }

        i += 1;
    }

    s
}

fn reverse_word(w: &mut [u8]) {
    let mut l = 0;
    let mut r = w.len() - 1;

    while l < r {
        w.swap(l, r);

        l += 1;
        r -= 1;
    }
}
