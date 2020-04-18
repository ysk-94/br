mod req;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
   req::get(); 
}

