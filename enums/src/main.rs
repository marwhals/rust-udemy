/**
- Enums, similar to traits in Scala
-- Three 'structs' which can be used where a Media type is required

Structs vs Enums
- Similar methods -> Enums, Different methods -> structs

Some notes on Options
- item.unwrap() - will panic if the item is None. Scala equiv: .get
- item.expect("123") - will panic and provide a debug message - used to cause a crash if there is no value
- item.unwrap_of(&placeholder) - if item is None, return the placeholder value. Scala equiv: getOrElse

*/
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book: {} {}", title, author)
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie: {} {}", title, director)
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else {
        //     String::from("Media description")
        // }

        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) { //self reference, changing data in catalgue, want function to take ownership of media
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> MightHaveAValue {
        if self.items.len() > index {
            // Good! We have something to return
            MightHaveAValue::ThereIsAValue(&self.items[index])
        } else {
            // Bad! We don't have anything to return!!!
            MightHaveAValue::NoValueAvailable
        }
    }

    fn get_by_index2(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good! We have somethign to return
            Some(&self.items[index])
        } else {
            // Bad! We don't have anything to return!!!
            None
        }
    }

}

/**
A custom implementation of an Option
*/
enum MightHaveAValue<'a> {
    ThereIsAValue(&'a Media),
    NoValueAvailable,
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn print_media_via_reference(media: &Media) {
    println!("{:#?}", media);
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("An Audiobook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    print_media_via_reference(&audiobook);
    print_media_via_reference(&good_movie);
    print_media_via_reference(&bad_book);
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;


    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    /** Options
    - Rust doesn't have null, nil or undefined. Has Options instead. Two variants, Some and None,
    - To extract the value need to use pattern matching or a match statement
    - Forces you to handle the case of Some() and a None
    */
    match catalog.items.get(100) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at that index")
        }
    }

    match catalog.get_by_index2(9999) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here!");
        }
    }

    if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(99990) {
        println!("Item in pattern match: {:#?}", value)
    } else {
        println!("No value!!!!!!!!!!");
    }
}
