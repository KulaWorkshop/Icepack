use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Arguments {
    /// Extract archive
    #[clap(short = 'x', long, action)]
    pub extract: bool,

    /// Create archive
    #[clap(short = 'c', long, action)]
    pub create: bool,

    /// Set creation type to PAK
    #[clap(short = 'p', long, action)]
    pub pak: bool,

    /// Set creation type to KUB
    #[clap(short = 'k', long, action)]
    pub kub: bool,

    #[arg(num_args(0..))]
    pub files: Vec<String>,
}
