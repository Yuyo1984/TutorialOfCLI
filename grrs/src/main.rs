use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

#[allow(unused_variables)]
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
fn main() {
    let args = Cli::parse();

    // ファイルの内容を取得
    let content = std::fs::read_to_string(&args.path).expect("ファイルが読み込めませんでした");
    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())
}
