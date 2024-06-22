fn main() {
    // this is our first program in this course
    // this is the second line of the comment

    /* This is a
     * multiple line
     * comment
     */

    println!("Hello from Rust!");

    //print!(10);

    println!("The value is {}", 10);

    println!("My first name is {} and my last is {}", "Joseph", "Noblett");

    print!("This is a print command");

    print!("This is going to be on the same line, ");
    print!("This is going to be on the same line, ");
    print!("This is going to be on the same line.");

    println!("");

    print!(
        "this is going to be 
    on multiple 
    lines."
    );

    println!("\n\n this is going to be outputted after one line");

    println!("\t this will have some a tab inserted at the beginning");

    println!(
        "this is some text which will be overwritten \r this text will only appear on the screen"
    );

    println!("This will print single quote \' and this double quotes\"");

    println!("This will print one backslash \\");

    println!(
        "\n doing {2} from {1} years and i {0} it",
        "like", 20, "programming"
    );

    println!(
        "{language} is a system programming language which is cool to {activity} in.",
        activity = "Code",
        language = "Rust"
    );

    println!("The sum of 25 + 10 = {}", 25 + 10);
}
