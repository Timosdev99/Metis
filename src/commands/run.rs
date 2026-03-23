use anyhow::Result;
use std::path::Path;

use crate::adapters::{get_adapter, known_adapters, InjectionMethod};
use crate::session::context::ContextBuilder;
use crate::session::models::Session;
use crate::session::store::SessionStore;

pub fn handle(project_root: &Path, cli_name: &str, extra_args: &[String]) -> Result<()> {
    let store = SessionStore::new(project_root);
    guard_initialised(&store)?;

    let adapter = require_adapter(cli_name)?;

    let mut session = store
        .load()?
        .unwrap_or_else(|| Session::new(project_root.display().to_string(), cli_name));
    session.active_cli = cli_name.to_string();
    store.save(&session)?;

    let handoff = if let Some(summary) = store.read_summary()? {
        println!(
            "Metis: resuming session ({} turns) — injecting context into {}",
            session.turn_count(),
            cli_name
        );
        ContextBuilder::build_handoff_prompt(&summary, cli_name)
    } else {
        println!("Metis: starting fresh session with {}", cli_name);
        String::new()
    };

    launch_cli(&adapter, &handoff, extra_args, project_root)?;
    Ok(())
}

fn guard_initialised(store: &SessionStore) -> Result<()> {
    if !store.is_initialised() {
        anyhow::bail!("This directory has not been initialised with Metis.\nRun `metis init` first.");
    }
    Ok(())
}

fn require_adapter(name: &str) -> Result<crate::adapters::CliAdapter> {
    get_adapter(name).ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown CLI: '{}'\nSupported: {}",
            name,
            known_adapters().join(", ")
        )
    })
}

pub(crate) fn launch_cli(
    adapter: &crate::adapters::CliAdapter,
    handoff: &str,
    extra_args: &[String],
    project_root: &Path,
) -> Result<()> {
    use std::io::{self, Read, Write};
    use std::sync::{Arc, Mutex};

    let (binary, args, stdin_content) = adapter.build_launch(handoff, extra_args);

    if matches!(adapter.injection, InjectionMethod::PrintOnly) && !handoff.is_empty() {
        println!("\n── Metis context (paste this into {}) ──\n", adapter.name);
        println!("{}", handoff);
        println!("────────────────────────────────────────\n");
    }

    if adapter.launch == crate::adapters::LaunchMode::Inherit {
        use std::process::Command;
        let mut cmd = Command::new(&binary);
        cmd.args(&args);
        cmd.current_dir(project_root);
        let status = cmd.status()?;
        if !status.success() {
            anyhow::bail!("{} exited with status {}", binary, status);
        }
        return Ok(());
    }

    use portable_pty::{CommandBuilder, PtySize, native_pty_system};

    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let mut cmd = CommandBuilder::new(binary);
    cmd.args(&args);
    cmd.cwd(project_root);

    let mut child = pair.slave.spawn_command(cmd)?;
    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader()?;
    let writer = Arc::new(Mutex::new(pair.master.take_writer()?));
    let responder_enabled = true;

    let output_writer = Arc::clone(&writer);
    let _raw_guard = RawModeGuard::enter()?;

    let output_handle = std::thread::spawn(move || {
        let mut stdout = io::stdout();
        let mut buf = [0u8; 2048];
        let mut pending: Vec<u8> = Vec::new();
        let mut parser = vt100::Parser::new(24, 80, 1000);

        loop {
            let n = match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            pending.extend_from_slice(&buf[..n]);

            // Respond to common terminal queries so CLIs don't stall.
            if responder_enabled {
                let combined = pending.clone();
                if combined.windows(4).any(|w| w == b"\x1b[6n") {
                    if let Ok(mut w) = output_writer.lock() {
                        let _ = w.write_all(b"\x1b[24;1R");
                        let _ = w.flush();
                    }
                }
                if combined.windows(3).any(|w| w == b"\x1b[c")
                    || combined.windows(4).any(|w| w == b"\x1b[0c")
                    || combined.windows(4).any(|w| w == b"\x1b[?c")
                {
                    if let Ok(mut w) = output_writer.lock() {
                        let _ = w.write_all(b"\x1b[?62;4c");
                        let _ = w.flush();
                    }
                }
            }

            parser.process(&pending);
            pending.clear();

            let screen = parser.screen();
            let (row, col) = screen.cursor_position();
            let _ = crossterm::execute!(
                stdout,
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                crossterm::cursor::MoveTo(0, 0),
                crossterm::cursor::Hide
            );
            let _ = stdout.write_all(screen.contents().as_bytes());
            let _ = crossterm::execute!(stdout, crossterm::cursor::MoveTo(col as u16, row as u16));
            let _ = stdout.flush();
        }
    });

    if let Some(stdin_text) = stdin_content {
        let mut text = stdin_text;
        if !text.ends_with('\n') {
            text.push('\n');
        }
        if let Ok(mut w) = writer.lock() {
            w.write_all(text.as_bytes())?;
            w.flush()?;
        }
    }

    let input_writer = Arc::clone(&writer);
    std::thread::spawn(move || {
        let mut stdin = io::stdin().lock();
        let mut buf = [0u8; 1024];
        loop {
            let n = match stdin.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(_) => break,
            };
            if let Ok(mut w) = input_writer.lock() {
                let _ = w.write_all(&buf[..n]);
                let _ = w.flush();
            }
        }
    });

    let status = child.wait()?;
    let _ = output_handle.join();
    if !status.success() {
        anyhow::bail!("{} exited with status {}", adapter.binary, status);
    }

    Ok(())
}

struct RawModeGuard;

impl RawModeGuard {
    fn enter() -> anyhow::Result<Self> {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        let _ = crossterm::execute!(stdout, crossterm::cursor::Hide);
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let mut stdout = std::io::stdout();
        let _ = crossterm::execute!(stdout, crossterm::cursor::Show);
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
