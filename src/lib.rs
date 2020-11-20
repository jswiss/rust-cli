use indicatif::{ProgressBar, ProgressStyle};

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
    ));

    let pattern_length = String::from(content).chars().count();
    println!("num of chars: {}", pattern_length);

    if pattern_length == 0 {
        panic!("You need to include a string to match!");
    }

    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("something went wrong....");
            pb.println(format!("[+] finished #{}", line));
            pb.inc(25);
        }
    }
    pb.finish_with_message("donezo!")
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
