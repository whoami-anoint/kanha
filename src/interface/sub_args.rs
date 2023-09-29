use clap::Args;

#[derive(Args, Clone)]
pub struct StatusArgs {
    /// A file containing multiple urls
    #[arg(short, long)]
    #[arg(required = false, conflicts_with = "stdin")]
    pub filename: Option<String>,

    /// Define the maximum concurrent tasks
    #[arg(short, long, default_value = "10")]
    pub tasks: usize,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,

    /// Define your status code for selective exclusion.
    #[arg(long)]
    pub exclude: Option<String>,
}

#[derive(Args)]
pub struct FuzzerArgs {
    /// A file containing a list of possible wordlists.
    #[arg(required = true, short, long)]
    pub wordlist: String,

    /// Provide a url to fuzz.
    #[arg(required = true, short, long)]
    pub url: String,

    /// Define the maximum concurrent tasks.
    #[arg(short, long, default_value = "10")]
    pub tasks: usize,

    /// Define your status code for selective exclusion.
    #[arg(long)]
    pub exclude: Option<String>,
}

#[derive(Args, Clone)]
pub struct TakeoverArgs {
    /// A json file containing signature values of different services
    #[arg(required = false, short, long)]
    pub json_file: String,

    /// A file containing a list of urls
    #[arg(required = true, conflicts_with = "stdin", short, long)]
    pub filename: Option<String>,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,
}

#[derive(Args)]
pub struct RdnsArgs {
    /// a file containing a list of possible wordlists
    #[arg(required = true, conflicts_with = "stdin", short, long)]
    pub filename: Option<String>,

    /// Reads input from the standard in
    #[arg(long)]
    pub stdin: bool,
}

#[derive(Args)]
pub struct DencodeArgs {
    /// Provide a url to encode
    #[arg(required = false, conflicts_with = "decode", long)]
    pub encode: Option<String>,

    /// Provide a url to dencode
    #[arg(required = false, long)]
    pub decode: Option<String>,
}
