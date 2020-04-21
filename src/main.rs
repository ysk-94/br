mod req;

fn main() {
    let mode = std::env::args().nth(1);

   req::get(); 
}

