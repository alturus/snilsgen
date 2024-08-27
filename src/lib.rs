use rand::Rng;
use std::fmt;

const MIN_SNILS_NUMBER: u32 = 1002001;
const MAX_SNILS_NUMBER: u32 = 998998998;
const SNILS_LEN: usize = 11;
const SNILS_CHECKSUM_LEN: usize = 2;
const MAX_REPEATED_DIGITS: u8 = 2;

type SnilsNumber = [u8; SNILS_LEN];

fn init_snils_number() -> SnilsNumber {
    let mut raw_number = rand::thread_rng().gen_range(MIN_SNILS_NUMBER..=MAX_SNILS_NUMBER);
    let mut snils_number: SnilsNumber = [0u8; SNILS_LEN];
    let mut checksum: u16 = 0;
    let mut prev_digit: u8 = u8::MAX;
    let mut repeated_digit_count: u8 = 0;

    for (i, d) in snils_number
        .iter_mut()
        .rev()
        .skip(SNILS_CHECKSUM_LEN)
        .enumerate()
    {
        if raw_number == 0 {
            break;
        }

        *d = (raw_number % 10) as u8;
        raw_number /= 10;

        if *d == prev_digit {
            repeated_digit_count += 1;
            if repeated_digit_count == MAX_REPEATED_DIGITS {
                match *d {
                    0..=5 => *d += 1,
                    _ => *d -= 1,
                }
                repeated_digit_count = 0;
            }
        } else {
            repeated_digit_count = 0;
        }

        prev_digit = *d;

        checksum += *d as u16 * (i as u16 + 1);
    }

    checksum %= 101;
    if checksum == 100 {
        checksum = 0;
    }

    if checksum > 0 {
        for d in snils_number.iter_mut().rev().take(SNILS_CHECKSUM_LEN) {
            *d = (checksum % 10) as u8;
            checksum /= 10;
        }
    }

    snils_number
}

pub struct Snils {
    number: SnilsNumber,
}

impl Snils {
    pub fn new() -> Self {
        Self {
            number: init_snils_number(),
        }
    }
}

impl Default for Snils {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Snils {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_number = self.number.iter().enumerate().fold(
            String::with_capacity(SNILS_LEN),
            |mut out, (i, d)| {
                if i == 3 || i == 6 {
                    out.push('-');
                } else if i == 9 {
                    out.push(' ');
                }
                out.push_str(d.to_string().as_str());
                out
            },
        );
        write!(f, "{}", formatted_number)
    }
}
