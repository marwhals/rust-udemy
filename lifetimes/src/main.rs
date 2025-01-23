/**
 -Lifetimes - annotations that make sure refs won't outlive the value they refer to
    - a lifetime is how long a binding can be used
    - a reference to a value can't out live that value
*/
/**
    Lifetimes - there is a type of ref called 'a', This first ref is of type 'a', This returned type ref is also of type 'a'
    ----Used to match the return types of references
    ----Omitting lifetime annotations is referred to as elision
*/
fn next_language<'a>(languages: &'a[String], current: &str) -> &'a str {
    //Rust assumes that the return ref will point at data referred to by one of the arguments
    //Rust will no analyse the body of your fuction to figure out whether the return ref is pointing at the second or first arg
    //In the case of &self rust assumes the returned ref will point at &self even with other refs in the signature
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}
// There is a type of ref called 'a', these are both refs or type 'a', this ref will point at one of the 'a' refs
fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_language(&languages, "go");
    let result2 = longest_language("123456", "go");

    println!("{}", result);
    println!("{}", result2);
}
