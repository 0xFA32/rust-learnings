use std::collections::HashMap;



fn hash_map_update() {
    let test = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in test.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);    
}

fn median() {
    let mut test = vec![1, 10, 100, 0, 2, 4, 6, 1000, 5];
    test.sort();

    // For testing purpose. Not to calculate actual median.
    println!("Median = {}", &test[test.len() / 2]);
}

fn mode() {
    let test = vec![1, 10, 10, 2, 2, 4, 4, 10, 10];
    let mut mode = HashMap::new();
    for value in test {
        let value = mode.entry(value).or_insert(0);
        *value += 1;
    }

    if mode.len() == 0 {
        println!("No mode");
        return;
    }

    let mut ans = 0;
    let mut ans_count = 0;
    for (value, count) in mode {
        if ans_count < count {
            ans_count = count;
            ans = value;
        }
    }

    println!("Ans = {} and count = {}", ans, ans_count);
}

fn string_stuff() {
    let one_string = String::from("first");
    let another_string = String::from("apple");
    let yet_another_string = String::from("xlpqrtaeiou");

    convert_better_version(&one_string);
    convert_better_version(&another_string);
    convert_better_version(&yet_another_string);

    fn convert(input: &str) {
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let mut cur = String::from("");
        let mut index = 0;
        for i in input.chars() {
            if vowels.contains(&i) {
                let mut ans = String::from(&input[index..]);
                ans.push_str("-hay");
                ans.push_str(&cur);
                
                println!("Ans = {}", ans);
                return;
            }

            cur.push(i);
            index += i.len_utf8();
        }

        println!("Cur = {}", cur);
    }

    fn convert_better_version(input: &str) {
        let first_vowel = input.char_indices()
            .find(|(_, c)| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .map(|(i, _)| i);

        match first_vowel {
            Some(index) => {
                let (start, end) = input.split_at(index);
                // println!("{}-hay{}", &input[index..], &input[..index]);
                println!("{}-hay{}", end, start);
            },
            None => {
                println!("{}", input);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_map_update_test() {
        hash_map_update();
    }

    #[test]
    fn median_and_mode_test() {
        median();
        mode();
    }

    #[test]
    fn string_stuff_test() {
        string_stuff();
    }
}
