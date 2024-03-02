use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// DMARC Report Analyzer
pub struct Configuration {
    /// Host name or domain of the IMAP server with the DMARC reports inbox
    #[arg(short = 's', long)]
    pub imap_host: String,

    /// User name of the IMAP inbox with the DMARC reports
    #[arg(short = 'u', long)]
    pub imap_user: String,

    /// Password of the IMAP inbox with the DMARC reports
    #[arg(short = 'p', long)]
    pub imap_password: String,

    /// TLS encrypted port of the IMAP server
    #[arg(long, default_value = "993")]
    pub imap_port: u16,
}
