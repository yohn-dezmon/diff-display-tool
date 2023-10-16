use serde_json;
/*
Serde is the preferred JSON serialization provider

How can I compare these line by line?
Maybe converting to JSOn is the wrong appraoch.
Maybe I should just read them in as text.

Then split by line
and compare each line.
If there is a line that does not match, save it in a file.
Then print the lines that did not match from each file!

Otherwise if I stick with JSON, I could compare each JSON inner object,
and do the same where I save the values that don't match, and then print them out.
*/
fn main() {
    // import file_1
    // let file_1_path = Path::new("data/ex_file_1.json");
    // let file_1 = File::open(file_1_path);
    let file_1_data =
        std::fs::read_to_string("src/data/ex_file_1.json").expect("Unable to read file 1");
    let file_1_json: serde_json::Value =
        serde_json::from_str(&file_1_data).expect("JSON not well formatted");

    // import file_2
    // let file_2_path = Path::new("data/ex_file_2.json");
    // let file_2 = File::open(file_2_path);
    let file_2_data =
        std::fs::read_to_string("src/data/ex_file_2.json").expect("Unable to read file 2");
    let file_2_json: serde_json::Value =
        serde_json::from_str(&file_2_data).expect("JSON not well formatted");

    print!("{}", file_1_json);
    print!("{}", file_2_json);
}
