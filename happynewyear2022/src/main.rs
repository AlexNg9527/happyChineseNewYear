use regex::Regex;

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn main() {
    let greetings = ["world hello", "year happy new", "大 年 吉 虎"];
    // println!("{:?}", greetings);
    println!("************greetings************\n");
    let han = Regex::new(r"^[\u4e00-\u9fa5]$").unwrap();
    for words in greetings.iter() {
        let mut vec: Vec<&str> = words.split_whitespace().collect(); 
        if han.is_match(vec[0]){
            vec.sort_unstable_by(|a, b| b.cmp(a));         
        }
        else {
            vec.sort_unstable();
        }
        for x in vec
        { 
            print!("{} ", uppercase_first_letter(x)) 
        } 
        println!("~\n");

    }
}     
