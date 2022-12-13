use anomaly_analysis::model::classifier::TextClassifier;

fn read() -> String {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf
}

fn main() {
    println!("Preparing model...");
    let classifier = TextClassifier::from_string(include_str!("classifier.json").to_owned());
    println!("Model ready! Enter text to classify");
    loop {
        let line = read();
        println!(": {}", classifier.classify(&line));
    }
}