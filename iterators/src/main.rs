fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements.iter().for_each(|el| println!("{}", el)); //Iterators are lazy "for_each" is an iterator consumer
                                                       //Iterator adapters
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

/*
- Need to store argument? - Take ownership i.e receive a value
- Need to do a calculation - Use a read only ref
- Need to change the value - Use a mutable ref
*/
fn shorten_strings(elements: &mut [String]) {
    //Mutable slice of a vector of strings
    //iter() -> The iterator will only give you a read only reference to each element
    //iter_mut() -> The iterator will give you a mutable reference to each element
    //into_iter() -> The iterator will give you ownership of each element,*unless called on a mutable ref to a vector
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    // let _ = elements.iter().map(|el| el.to_uppercase()).collect();
    //collect is an iterator consumer and will return the new vector of string or any other data structure based on the type annotation

    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
    //Note the type annotation you add to your code in Rust can modify how the code runs.
}
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    /*An iterator which will give you ownership of each element
    - How it works
    --- Iterator called on a reference will produce references to each value
    --- Iterator created out of a mutable reference will produce mutable refs to each value
    --- Iterator create out of a value will produce each value and also moves ownership of these values
     */
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}
fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    let mut colors_iter = colors.iter();

    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());

    print_elements(&colors);
    print_elements(&colors[1..3]);

    shorten_strings(&mut colors[1..3]);
    println!("{:#?}", colors);

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);

    // let mut destination = vec![]; -- Could tweak this to use reference
    // move_elements(colors, &mut destination);
    // println!("Destination: {:#?}", destination);

    let exploded = explode(&colors);
    println!("{:#?}", exploded);

    let found_color = find_color_or(&colors, "asdfasdf", "Orange");
    println!("{}", found_color);
}
