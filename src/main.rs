use decoder::Decoder;

mod args;
mod decoder;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Command::Decode(decode_args) => run_decode(decode_args.value),
    }
}

fn run_decode(value: String) {
    let mut decoder = Decoder::new(&value);
    let decoded_value = decoder.decode().expect("Failed to decode");
    println!("{}", decoded_value);
}
