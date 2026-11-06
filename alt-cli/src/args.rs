use alt_ansi::colour::Colours;

#[derive(clap::Parser)]
pub struct Args {
    // -h is in use by --help
    #[arg(short, long)]
    pub ignore_lines: Option<usize>,
    
    #[arg(short, long)]
    pub colours: Vec<String>,
}

