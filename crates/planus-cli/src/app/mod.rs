mod check;
mod dot;
mod format;
mod gen_completions;
mod ocaml;
mod rust;
mod view;

use std::process::ExitCode;

use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use color_eyre::Result;

fn clap_v3_styling() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Green.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser)]
#[command(styles = clap_v3_styling())]
pub struct App {
    #[clap(flatten)]
    app_options: AppOptions,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser)]
pub enum Command {
    Dot(dot::Command),
    Rust(rust::Command),
    Ocaml(ocaml::Command),
    View(view::Command),
    Format(format::Command),
    Check(check::Command),
    GenerateCompletions(gen_completions::Command),
}

#[derive(Default, Parser)]
pub struct AppOptions {}

impl App {
    pub fn run(self) -> Result<ExitCode> {
        match self.command {
            Command::Dot(command) => command.run(self.app_options),
            Command::Rust(command) => command.run(self.app_options),
            Command::Ocaml(command) => command.run(self.app_options),
            Command::Format(command) => command.run(self.app_options),
            Command::Check(command) => command.run(self.app_options),
            Command::GenerateCompletions(command) => command.run(self.app_options),
            Command::View(command) => command.run(self.app_options),
        }
    }
}
