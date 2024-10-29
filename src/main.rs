use clap::{Parser, Subcommand};
mod models;
mod controllers;

#[derive(Parser)]
#[command(name = "cli-AutoCad-manager")]
#[command(about = "AutoCad manager cli app")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // Parts: 部品表の処理
    // Collection⇒Check⇒Aggregateの一括処理
    #[command(about = "Collect XLSX parts list")]
    Parts,
    #[command(about = "Collect XLSX parts list")]
    Collection,
    #[command(about = "Check duplicate error in collection csv ")]
    Check,
    #[command(about = "Aggregate from collection csv")]
    Aggregate,

}
#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Parts) => {
            controllers::parts::aggregate_parts_table_from_xlsx()?;
        },
        
        Some(Commands::Collection) => {
            controllers::collect::collect_xlsx_parts_list()?;
        },

        Some(Commands::Check) => {
            controllers::check::check_collection_csv()?;
        },
        Some(Commands::Aggregate) => {
            controllers::aggregate::aggregate_collection_csv()?;
        },

        None => {
            // 指定しない場合はPartsTableを実行
            controllers::parts::aggregate_parts_table_from_xlsx()?;
        }
    }

    Ok(())
}
