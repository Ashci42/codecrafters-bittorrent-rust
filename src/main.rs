mod args;
mod decoder;

fn main() {
    let args = args::Args::new();
    match args.command {
        args::Command::Decode(decode_args) => run_decode(decode_args.value),
    }
}

fn run_decode(value: String) {
    let decoded_value = decoder::decode(&value).expect("Failed to decode");
    println!("{}", decoded_value);
}
