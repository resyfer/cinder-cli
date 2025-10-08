use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short)]
    pub dataset_path: String,

    #[arg(short)]
    pub score_output_path: String,

    #[arg(short, long, default_value_t = 7)]
    pub threads: u8,
}
