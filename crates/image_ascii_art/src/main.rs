use std::path::PathBuf;

use clap::Parser;

mod ascii_art {
    use std::path::Path;

    const ASCII_BRIGHTNESS: &str =
        "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

    pub fn ascii(file: impl AsRef<Path>) -> anyhow::Result<String> {
        let img = image::open(file.as_ref())?;
        let mut res = String::new();
        let mut height = 0;
        for (_, y, p) in img.to_luma8().enumerate_pixels() {
            if height != y {
                // height changed
                height = y;
                res.push('\n');
            }
            let [brightness] = p.0;
            res.push(scale(brightness));
        }
        Ok(res)
    }

    /// Scale a ['u8'] value representing brightness to a character in ['ASCII_BRIGHTNESS']
    ///
    /// [0 - 255] scaled to [0 - 65]
    /// [min max] scaled to [a - b]
    /// ((b - a)(x - min) / (max - min)) + a
    /// ((L - 0)(bright - 0)/(255 - 0)) + 0
    ///
    /// brightness / 255 to get percentage
    /// then multiple by ASCII length
    fn scale(bright: u8) -> char {
        const L: usize = ASCII_BRIGHTNESS.len() - 1; //Start at zero

        let index = (L * bright as usize) / 255;
        ASCII_BRIGHTNESS.chars().nth(index).unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::scale;
        use super::ASCII_BRIGHTNESS;

        #[test]
        fn test_scale() {
            assert_eq!(scale(u8::MIN), '`');
            assert_eq!(scale(u8::MAX), '$');
            for b in u8::MIN..=u8::MAX {
                let c = scale(b);
                assert!(ASCII_BRIGHTNESS.contains(c));
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_parser)]
    image: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let art = ascii_art::ascii(args.image)?;
    println!("{art}");

    Ok(())
}
