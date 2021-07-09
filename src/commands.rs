#![allow(clippy::new_ret_no_self)]
use crate::command::Command;
use itertools::Itertools;

pub struct FeedPaper;

impl FeedPaper {
    const COMMAND: u8 = 0xa1;

    pub fn new(steps: u16) -> Command {
        Command::new(Self::COMMAND, &steps.to_le_bytes())
    }
}

pub struct PrintLine;

impl PrintLine {
    const COMMAND: u8 = 0xa2;

    pub fn new(line: &[u8]) -> Command {
        assert_eq!(line.len(), 48);
        Command::new(Self::COMMAND, line)
    }

    pub fn with_pixels(line: &[bool]) -> Command {
        assert_eq!(line.len(), 384);

        let mut payload = [0x00; 48];
        for (i, pixel) in line.iter().enumerate() {
            if *pixel {
                let idx = i / 8;
                let bit = i % 8;
                payload[idx] |= 1 << bit;
            }
        }

        Command::new(Self::COMMAND, &payload)
    }
}

pub struct PrintLineCompressed;

impl PrintLineCompressed {
    const COMMAND: u8 = 0xbf;

    pub fn new(line: &[bool]) -> Command {
        // A compressed line is a sequence of bytes where bit 7 represents the
        // color (0 for white, 1 for black) and the low 7 bits represent a
        // count for pixels of that color.
        assert_eq!(line.len(), 384);

        let mut payload = vec![];

        let mut pixel_iter = line.iter();
        while let Some(first_pixel) = pixel_iter.next() {
            // Add one to account for the first pixel we grabbed
            let mut count = 1 + pixel_iter
                .take_while_ref(|pixel| *pixel == first_pixel)
                .count();

            let color = if *first_pixel { 0x80 } else { 0x00 };

            while count != 0 {
                let inner_count = usize::min(count, 0x7f);

                let byte = color | (inner_count as u8);
                payload.push(byte);

                count -= inner_count;
            }
        }

        Command::new(Self::COMMAND, &payload)
    }
}

pub struct SetQuality;

impl SetQuality {
    const COMMAND: u8 = 0xaf;

    pub fn new(quality: u8) -> Command {
        Command::new(Self::COMMAND, &[quality])
    }
}

pub enum Mode {
    Image,
    Text,
}

pub struct SetMode;

impl SetMode {
    const COMMAND: u8 = 0xbe;

    pub fn new(mode: Mode) -> Command {
        Command::new(
            Self::COMMAND,
            &[match mode {
                Mode::Image => 0x00,
                Mode::Text => 0x01,
            }],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_line_compressed() {
        let line = [false; 384];
        let cmd = PrintLineCompressed::new(&line);
        assert_eq!(cmd, Command::new(0xbf, &[0x7f, 0x7f, 0x7f, 0x03]));

        let line = [true; 384];
        let cmd = PrintLineCompressed::new(&line);
        assert_eq!(cmd, Command::new(0xbf, &[0xff, 0xff, 0xff, 0x83]));

        // this is the top of a :nya_hearts:
        let line_str = "000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000111111111111111111111111111111000000000111000000001111000000000000000111100000000000000000111111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let mut line = [false; 384];
        for (i, c) in line_str.chars().enumerate() {
            if c == '1' {
                line[i] = true;
            }
        }

        let cmd = PrintLineCompressed::new(&line);
        assert_eq!(
            cmd,
            Command::new(
                0xbf,
                &[0x7f, 0x41, 0x9e, 0x09, 0x83, 0x08, 0x84, 0x0f, 0x84, 0x11, 0x87, 0x5f]
            )
        );
    }
}
