fn main() {
    let s = String::from("hello, world");
    let word_index = first_word(&s);
    println!("{}", word_index);

   let word = first_word_slice(&s[..]);
   println!("{}", word);

   let s_literal = "hello world";
   let word2 = first_word_slice(s_literal);
   // or
   // let word2 = first_word_slice(&s_literal[..]);
   println!("{}", word2);

   // cannot borrow as mutable because it is also borrowed as immutable
   //s.clear();

   let a = [1, 2, 3, 4, 5];
   let slice = &a[1..3];
   println!("{}", slice[0]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
