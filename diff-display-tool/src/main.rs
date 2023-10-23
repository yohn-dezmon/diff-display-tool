use std::fs;

fn main() {
    let file_1_data = fs::read_to_string("src/large_data/ex_file_1.json").expect("Unable to read file 1");
    let file_2_data = fs::read_to_string("src/large_data/ex_file_2.json").expect("Unable to read file 2");

    // Split the file contents into lines
    let lines_1: Vec<&str> = file_1_data.lines().collect();
    let lines_2: Vec<&str> = file_2_data.lines().collect();

    for (i, line) in lines_1.iter().enumerate() {
        if line != &lines_2[i] {
            println!("Line {} did not match!", i);
            println!("file 1: {} \nfile 2: {}\n", line, lines_2[i]);
        }
    }
}
