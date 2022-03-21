use std::fs::File;
use std::io::Read;

use syn::__private::ToTokens;
use walkdir::WalkDir;

pub fn main() {
    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().map(|s| s == "rs").unwrap_or(false) {
            let mut file = File::open(&entry.path()).expect("Unable to open file");
            let mut src = String::new();
            file.read_to_string(&mut src).expect("Unable to read file");
            let syntax = syn::parse_file(&src).expect("Unable to parse file");
            let tokens = near_macros::metadata(syntax.to_token_stream());
            println!("\n{}:\n{}", entry.path().display(), tokens);
        }
    }
}
