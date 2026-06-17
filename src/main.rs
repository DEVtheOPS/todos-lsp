fn main() -> std::process::ExitCode {
    todos_lsp::app::run(std::env::args_os())
}
