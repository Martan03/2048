use std::str::FromStr;

use termint::{
    enums::Color,
    geometry::Coords,
    help,
    widgets::{Grad, StrSpanExtension},
};

use crate::error::Error;

/// Parses given arguments and checks for arguments conditions
#[derive(Debug)]
pub struct Args {
    pub size: Coords,
    pub win: u16,
    pub help: bool,
}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, Error> {
        let mut parsed = Self::default();

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-s" | "--size" => parsed.parse_size(&mut args_iter)?,
                "-w" | "--win" => parsed.parse_win(&mut args_iter)?,
                "-h" | "--help" => parsed.help = true,
                arg => Err(format!("unexpected argument: '{arg}'"))?,
            }
        }

        Ok(parsed)
    }

    /// Displays help
    pub fn help() {
        println!(
            "Welcome to help for {} by {}\n",
            "tui2048".fg(Color::Green),
            Grad::new("Martan03", (0, 220, 255), (175, 80, 255))
        );
        help!(
            "Usage":
            "tui2048" => "Opens 4x4 game with win number 2048\n"
            "tictactoe" ["options"] => "Behaves according to options\n"
            "Options":
            "-s  --size" => "Sets size of the game\n"
            "-w  --win" => "Sets winning number\n"
            "-h  --help" => "Prints this help"
        );
    }

    /// Parses size from the given arguments
    fn parse_size<T>(&mut self, args: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        self.size = Coords::new(Args::get_num(args)?, Args::get_num(args)?);
        if self.size.x < 3 || self.size.y < 3 {
            return Err(Error::Msg("minimum supported size is 3".into()));
        }
        Ok(())
    }

    /// Parses win length from the given arguments
    fn parse_win<T>(&mut self, args: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        self.win = Args::get_num(args)?;

        let mut num = 2;
        while num < self.win {
            num *= 2;
        }

        if num != self.win {
            return Err(Error::Msg("win must be nth power of 2".into()));
        }
        Ok(())
    }

    /// Gets number (usize) from args
    fn get_num<N, T>(args: &mut T) -> Result<N, Error>
    where
        N: FromStr,
        T: Iterator<Item = String>,
    {
        let Some(val) = args.next() else {
            return Err(Error::Msg("missing argument parameter".into()));
        };

        val.parse::<N>()
            .map_err(|_| Error::Msg(format!("number expected, got '{val}'")))
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            size: Coords::new(4, 4),
            win: 2048,
            help: false,
        }
    }
}
