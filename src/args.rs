use std::path::Path;
use std::process::exit;
use std::str::Chars;
use std::vec;

use crate::argparser::{ArgError, ArgParser, argv};

#[derive(Debug)]
pub struct Args {
    pub infiles: Vec<String>,
    pub debug: bool,
    argv: vec::IntoIter<String>,
}

impl ArgParser for Args {
    type Argv = vec::IntoIter<String>;

    fn positional(&mut self, arg: String) { self.infiles.push(arg); }

    fn argv(&mut self) -> &mut Self::Argv { &mut self.argv }

    #[allow(unused)]
    fn flag(&mut self, c: char, it: &mut Chars) -> Result<(), ArgError> {
        match c {
            'g' => self.debug = true,
            'h' => Self::help(),
            c => return Err(ArgError::UnknownOption(c)),
        }
        Ok(())
    }
}

impl Default for Args {
    fn default() -> Self { Self { infiles: Vec::default(), debug: false, argv: argv() } }
}

impl Args {
    pub fn parse() -> Result<Self, ArgError> {
        let mut parsed = Self::default();
        parsed.walk()?;
        parsed.check()
    }

    fn check(self) -> Result<Self, ArgError> {
        match self.infiles.len() {
            1 if Path::new(&self.infiles[0]).is_file() => Ok(self),
            1 => Err(ArgError::NotAfile(self.infiles[0].clone())),
            n => Err(ArgError::BadArgumentCount(n, 1)),
        }
    }

    pub fn help() {
        println!("usage: retro_emu rom.gb [-g]");
        exit(0);
    }
}
