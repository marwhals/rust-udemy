use std::fs;
use std::io::Error;

/** Notes on computer memory
- Stack stores metadata about a data structure
- Heap stores the actual data

Corner Case
- If a data structure owns another data structure, the child's meta data will be placed on the heap

Notes on error handling
- Use match of 'if let' statement when you're ready to meaningfully deal with the error
- Call unwrap()/expect() on the Result for debugging or if a crash is desired on Err()
- Use the try operator '?' to unwrap or propagate the Result when you don't have any way to handle the error in the current function

*/

fn main() -> Result<(), Error> { //Rust main can return this type
    // let text = fs::read_to_string("logs.txt");

    // println!("{:#?}", text);

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong);
        }
    }

    match validate_email(String::from("abc@234.com")) {
        Ok(..) => println!("email is valid"), //Convention is .. when value is not used
        Err(reason_this_failed) => println!("{}", reason_this_failed),
    }

    string_test(
        "red".to_string(),
        &String::from("red"),
        String::from("red").as_str(),
    );

    match fs::read_to_string("error_handling/logs.txt") {
        Ok(text_that_was_read) => {
            let error_logs = extract_errors(text_that_was_read.as_str());

            match fs::write("error_handling/errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote errors.txt"),
                Err(reason_write_failed) => {
                    println!("Writing of errors.txt failed: {}", reason_write_failed);
                }
            }
        }
        Err(why_this_failed) => {
            println!("{}", why_this_failed)
        }
    }

    //Alternative to the nested matches above
    let text = fs::read_to_string("error_handling/logs.txt").expect("failed to read logs.txt");
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n")).expect("failed to write errors.txt");

    // The '?' operator on operations that can fail
    let text = fs::read_to_string("error_handling/errors.txt")?;
    println!("{}", text.len());

    Ok(())

}
fn string_test(
    a: String, // Stack and heap - used when we want ownership of text, use when we want to grow/shrink the data
    b: &String, // Stack and heap with a read only reference on the heap // Rarely used, Rust will automatically turn &String into &str
    c: &str,
)
//String slice reference on the Stack pointing to a string on the heap, lets you refer to text in the data segment without heap allocation, allows you to take string slice that is already on the heap. Used when ownership is not required and only a read only reference is requried.
{
}
type tupleExample = (i32, f64, String);
fn validate_email(email: String) -> Result<(), Error> {
    //Either
    if email.contains("@") {
        Ok(()) //Convention is empty tuple when nothing is retuned
    } else {
        Err(Error::other("emails must have an @"))
    }
}

/**
enum Result<T,E> {
    Ok(T),
    Err(T)
}
*/

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    //Using the Result enum
    if b == 0.0 {
        Err(Error::other("cant divide by 0")) //Err is imported from the standard library. No general catch all exists
    } else {
        Ok(a / b)
    }
}

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

