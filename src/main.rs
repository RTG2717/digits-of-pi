mod output_file_handler;
mod remainder_file_handler;

use output_file_handler::OutputFileHandler;
use remainder_file_handler::RemainderFileHandler;

struct NextDigitOutput {quotient: u8, remainder:u8}

fn get_next_digit(cur_remainder: u8) -> NextDigitOutput {
    NextDigitOutput {
        quotient: (cur_remainder * 10) / 7,
        remainder: (cur_remainder * 10) % 7
    }
}

fn digits_of_pi() {
    let mut output_handler = OutputFileHandler::new();
    let mut remainder_handler = RemainderFileHandler::new();

    // Check if we already have a remainder, if we have then continue
    let cur_remainder_str = remainder_handler.read();
    let mut cur_remainder: u8 = 0;

    if !cur_remainder_str.is_empty() {
        cur_remainder = cur_remainder_str.parse().expect("Couldn't convert remainder to u8");
    }
    else {
        // Hardcode first (pre-decimal) digit
        output_handler.append("3.");
        remainder_handler.write("1");
        cur_remainder = 1;
    }

    loop {
        let NextDigitOutput {quotient, remainder} = get_next_digit(cur_remainder);
        println!("Cur Quotient: {}, Cur Remainder: {}", quotient, remainder);

        cur_remainder = remainder;
        output_handler.append(&format!("{}", quotient));
        remainder_handler.write(&format!("{}", cur_remainder));
    }
}

fn main() {
    digits_of_pi();
}
