use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;

use clap::Parser;

use grand_slam::AnisetteConfiguration;
use grand_slam::auth::Account;
use ldid2::certificate::Certificate;
use ldid2::signing::signer::Signer;
use ldid2::signing::signer_settings::SignerSettings;
use rustls::crypto::CryptoProvider;
use types::Bundle;

#[derive(Debug, Parser)]
#[command(author, version, about, disable_help_subcommand = true)]
pub struct Cli {
    // #[arg(short = 'w', help = "Shallow (sign only the top-level bundle)")]
    // shallow: bool,
    // #[arg(long = "pem", value_name = "PEM", num_args = 1.., help = "Paths to PEM files")]
    // pem_files: Vec<PathBuf>,
    // #[arg(value_name = "BUNDLE", required = true, value_parser = clap::value_parser!(PathBuf), help = "Path to bundle to sign")]
    // bundle: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");

    // if cli.pem_files.len() < 2 {
    //     eprintln!("Please provide at least two PEM files (certificate and key) using --pem.");
    //     exit(1);
    // }

    // let signing_key = match Certificate::new(cli.pem_files.clone().into()) {
    //     Ok(cert) => cert,
    //     Err(e) => {
    //         eprintln!("Failed to create Certificate: {}", e);
    //         exit(1);
    //     }
    // };
    
    // let mut signer_settings = SignerSettings::default();
    // signer_settings.sign_shallow = cli.shallow;

    // let signer = Signer::new(Some(signing_key), signer_settings);
    // if let Err(e) = signer.sign(vec![cli.bundle.clone()]) {
    //     eprintln!("Failed to sign bundle {:?}: {}", cli.bundle, e);
    //     exit(1);
    // }
    
    // println!("{:?}", cli.bundle);
    
    // let bundle = Bundle::new(cli.bundle.clone());
    // match bundle {
    //     Ok(b) => {
    //         match b.get_embedded_bundles() {
    //             Ok(embedded_bundles) if !embedded_bundles.is_empty() => {
    //                 for embedded_bundle in embedded_bundles {
    //                     println!("{:?}", embedded_bundle.get_dir());
    //                 }
    //             }
    //             _ => {
    //                 println!("No embedded bundles found.");
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         eprintln!("Failed to open bundle {:?}: {}", cli.bundle, e);
    //         exit(1);
    //     }
    // }

    
}
