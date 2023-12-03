use log::debug;
use std::fs;

pub struct Args {
    pub input_path: std::path::PathBuf,
    pub part_two: bool,
}

impl Args {
    pub fn parse() -> Result<Self, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();

        let args = Self {
            part_two: pargs.contains("-2"),
            input_path: pargs.free_from_str()?,
        };

        Ok(args)
    }

    pub fn read_input_file(&self) -> Vec<String> {
        let contents = fs::read_to_string(&self.input_path)
            .unwrap_or_else(|_| panic!("Unable to open input file: {}", self.input_path.display()));

        debug!("Input contents:\n{}", contents);

        return contents.split('\n').map(|s| s.to_string()).collect();
    }
}
