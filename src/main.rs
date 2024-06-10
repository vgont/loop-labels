use std::io;

fn main() {
    println!("understanding labels...");

    let mut label = String::new();

    'first_label: loop {
        println!("Type 'first_label' to stop the main loop");

        'second_label: loop {
            println!("Type 'second_label' to stop the nested loop");

            match io::stdin().read_line(&mut label) {
                Ok(bytes_read) => println!("Bytes read: {}", bytes_read), //Ok(var) -> var represents the number of bytes read.
                Err(_) => {
                    println!("An error ocurred while trying to read the input");
                    continue;
                }
            };

            match &label.trim()[..] {
                "first_label" => {
                    println!("getting out first loop");
                    break 'first_label;
                }
                "second_label" => {
                    println!("getting out second loop");
                    break 'second_label;
                }
                _ => {
                    println!("Invalid label! Try 'first_label' or 'second_label'");
                    continue;
                }
            }
        }
    }
    println!("Have no more loops to stop!");
}
