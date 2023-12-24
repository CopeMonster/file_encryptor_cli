mod arguments;
mod encryptor;
mod io;

use arguments::Arguments;

fn main() {
    let args = Arguments::parse();

    let data = io::read_file(&args.filename);
    let encrypted = encryptor::encrypt(&data, &args.encryption);

    io::write(&args.filename, &args.output, &encrypted);
}