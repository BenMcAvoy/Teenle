use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "teenle")]
pub struct Config {
    #[structopt(short, long, default_value="localhost")]
    pub local: String,

    #[structopt(short, long)]
    pub remote: String,

    #[structopt(short, long, default_value="8080")]
    pub source_port: String,

    #[structopt(short, long, default_value="8080")]
    pub target_port: String,
}
