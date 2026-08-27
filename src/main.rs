use clap::{Parser, Subcommand};
use exo_planets::pipeline::{print_report, run_fetch, run_train_score};
use exo_planets::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "exo-planets",
    about = "Kipping TTV/TDV features and a linfa trainer on a cached NASA TAP slice. Not a moon confirmation engine."
)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Re-download the NASA Exoplanet Archive TAP slices into the cache.
    Fetch {
        #[arg(long, default_value = "data/cache")]
        cache: PathBuf,
    },
    /// Train linfa logistic regression on cached KOI/PS + synthetic TTV/TDV.
    Train {
        #[arg(long, default_value = "data/cache")]
        cache: PathBuf,
        #[arg(long, default_value = "data/out")]
        out: PathBuf,
    },
    /// Alias for train (writes holdout score cards too).
    Score {
        #[arg(long, default_value = "data/cache")]
        cache: PathBuf,
        #[arg(long, default_value = "data/out")]
        out: PathBuf,
    },
    /// Train + score using the in-repo cache (no network).
    All {
        #[arg(long, default_value = "data/cache")]
        cache: PathBuf,
        #[arg(long, default_value = "data/out")]
        out: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd.unwrap_or(Cmd::All {
        cache: PathBuf::from("data/cache"),
        out: PathBuf::from("data/out"),
    }) {
        Cmd::Fetch { cache } => {
            let paths = run_fetch(&cache)?;
            for p in paths {
                println!("wrote {}", p.display());
            }
        }
        Cmd::Train { cache, out } | Cmd::Score { cache, out } | Cmd::All { cache, out } => {
            let report = run_train_score(&cache, &out)?;
            print_report(&report);
            println!("wrote {}/report.json", out.display());
        }
    }
    Ok(())
}
