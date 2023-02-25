mod args;
use args::Args;

fn main() {
    let args = Args::new(); 
    println!("{:?}", args);

}

fn find_image_from_path(path: String){
    let image_reader = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();
}
