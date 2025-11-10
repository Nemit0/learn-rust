use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::{env, io::Read, path::PathBuf};

use eframe::egui::{self, RichText, ScrollArea};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize, Clone)]
struct Package {
    name: String,
    manifest_path: String,
}

#[derive(Default)]
struct UiPkg {
    name: String,
    manifest_path: String,
    selected: bool,
}

struct AppState {
    repo_root: PathBuf,
    packages: Vec<UiPkg>,
    output: String,
    running: bool,
    tx: Option<mpsc::Sender<String>>, // to append output from worker
    git_ref: String,
}

impl AppState {
    fn new(repo_root: PathBuf) -> Self {
        Self { repo_root, packages: Vec::new(), output: String::new(), running: false, tx: None, git_ref: String::new() }
    }

    fn log(&mut self, line: impl AsRef<str>) {
        self.output.push_str(line.as_ref());
        if !self.output.ends_with('\n') { self.output.push('\n'); }
    }
}

fn discover_packages(repo_root: &PathBuf) -> anyhow::Result<Vec<UiPkg>> {
    let mut cmd = Command::new("cargo");
    cmd.arg("metadata").arg("--format-version").arg("1").arg("--no-deps").current_dir(repo_root);
    let out = cmd.output()?;
    if !out.status.success() {
        anyhow::bail!("cargo metadata failed: {}", String::from_utf8_lossy(&out.stderr));
    }
    let md: Metadata = serde_json::from_slice(&out.stdout)?;
    let mut pkgs: Vec<UiPkg> = md
        .packages
        .into_iter()
        .filter(|p| p.manifest_path.contains("/lectures/") || p.manifest_path.contains("\\lectures\\"))
        .map(|p| UiPkg { name: p.name, manifest_path: p.manifest_path, selected: false })
        .collect();
    pkgs.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(pkgs)
}

fn spawn_and_stream(mut cmd: Command, tx: mpsc::Sender<String>) {
    thread::spawn(move || {
        let mut child = match cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn() {
            Ok(c) => c,
            Err(e) => {
                let _ = tx.send(format!("Failed to spawn: {}", e));
                return;
            }
        };
        let mut stdout = String::new();
        let mut stderr = String::new();
        if let Some(mut out) = child.stdout.take() { let _ = out.read_to_string(&mut stdout); }
        if let Some(mut err) = child.stderr.take() { let _ = err.read_to_string(&mut stderr); }
        let status = child.wait();
        let _ = tx.send(stdout);
        if !stderr.is_empty() { let _ = tx.send(stderr); }
        match status {
            Ok(st) => { let _ = tx.send(format!("\n[exit status: {}]", st)); }
            Err(e) => { let _ = tx.send(format!("\n[failed to wait: {}]", e)); }
        }
    });
}

fn main() -> eframe::Result<()> {
    let repo_root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Learn Rust — Grader UI",
        options,
        Box::new(move |_cc| Box::new(GraderApp::new(repo_root))),
    )
}

struct GraderApp { state: AppState, rx: mpsc::Receiver<String> }

impl GraderApp {
    fn new(repo_root: PathBuf) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut state = AppState::new(repo_root);
        state.tx = Some(tx);
        Self { state, rx }
    }

    fn start_job(&mut self, mut cmd: Command) {
        if self.state.running { return; }
        self.state.running = true;
        self.state.output.clear();
        self.state.log(format!("Running: {:?}", cmd));
        if let Some(tx) = self.state.tx.clone() { spawn_and_stream(cmd, tx); }
    }
}

impl eframe::App for GraderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Drain output channel
        while let Ok(line) = self.rx.try_recv() { self.state.log(line); }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Discover Packages").clicked() && !self.state.running {
                    match discover_packages(&self.state.repo_root) {
                        Ok(pkgs) => { self.state.packages = pkgs; self.state.output.clear(); self.state.log("Discovered packages under lectures/"); }
                        Err(e) => { self.state.log(format!("Discovery failed: {}", e)); }
                    }
                }

                if ui.button("Select All").clicked() { for p in &mut self.state.packages { p.selected = true; } }
                if ui.button("Clear").clicked() { for p in &mut self.state.packages { p.selected = false; } }

                if ui.button("Grade Selected").clicked() && !self.state.running {
                    let selected: Vec<String> = self.state.packages.iter().filter(|p| p.selected).map(|p| p.name.clone()).collect();
                    if selected.is_empty() {
                        self.state.log("No packages selected.");
                    } else {
                        let mut cmd = Command::new("cargo");
                        cmd.arg("test");
                        for p in selected { cmd.arg("-p").arg(p); }
                        cmd.arg("--quiet").current_dir(&self.state.repo_root);
                        self.start_job(cmd);
                    }
                }

                if ui.button("Grade All (workspace)").clicked() && !self.state.running {
                    let mut cmd = Command::new("cargo");
                    cmd.arg("test").arg("--workspace").current_dir(&self.state.repo_root);
                    self.start_job(cmd);
                }
            });
        });

        egui::SidePanel::left("left").resizable(true).show(ctx, |ui| {
            ui.heading("Packages");
            for pkg in &mut self.state.packages {
                ui.checkbox(&mut pkg.selected, &pkg.name);
            }

            ui.separator();
            ui.heading("Reset");
            ui.horizontal(|ui| {
                if ui.button("Snapshot Baseline").clicked() && !self.state.running {
                    // Call platform-specific script with Init
                    #[cfg(target_os = "windows")]
                    let mut cmd = { let mut c = Command::new("powershell.exe"); c.arg("-NoProfile").arg("-ExecutionPolicy").arg("Bypass").arg("-File").arg("./scripts/reset_libs.ps1").arg("-Init").current_dir(&self.state.repo_root); c };
                    #[cfg(not(target_os = "windows"))]
                    let mut cmd = { let mut c = Command::new("bash"); c.arg("./scripts/reset_libs.sh").arg("--init").current_dir(&self.state.repo_root); c };
                    self.start_job(cmd);
                }
            });
            ui.horizontal(|ui| {
                if ui.button("Restore Baseline").clicked() && !self.state.running {
                    #[cfg(target_os = "windows")]
                    let mut cmd = { let mut c = Command::new("powershell.exe"); c.arg("-NoProfile").arg("-ExecutionPolicy").arg("Bypass").arg("-File").arg("./scripts/reset_libs.ps1").current_dir(&self.state.repo_root); c };
                    #[cfg(not(target_os = "windows"))]
                    let mut cmd = { let mut c = Command::new("bash"); c.arg("./scripts/reset_libs.sh").current_dir(&self.state.repo_root); c };
                    self.start_job(cmd);
                }
            });
            ui.horizontal(|ui| {
                ui.label("Git ref:");
                ui.text_edit_singleline(&mut self.state.git_ref);
            });
            ui.horizontal(|ui| {
                if ui.button("Restore from Git Ref").clicked() && !self.state.running {
                    let git_ref = self.state.git_ref.trim().to_string();
                    if git_ref.is_empty() { self.state.log("Enter a git ref first (e.g., origin/main)"); } else {
                        #[cfg(target_os = "windows")]
                        let mut cmd = { let mut c = Command::new("powershell.exe"); c.arg("-NoProfile").arg("-ExecutionPolicy").arg("Bypass").arg("-File").arg("./scripts/reset_libs.ps1").arg("-GitRef").arg(&git_ref).current_dir(&self.state.repo_root); c };
                        #[cfg(not(target_os = "windows"))]
                        let mut cmd = { let mut c = Command::new("bash"); c.arg("./scripts/reset_libs.sh").arg("--git-ref").arg(&git_ref).current_dir(&self.state.repo_root); c };
                        self.start_job(cmd);
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let status = if self.state.running { RichText::new("Running...").strong() } else { RichText::new("Idle") };
                ui.label(status);
                if !self.state.running && ui.button("Clear Output").clicked() { self.state.output.clear(); }
            });
            ui.separator();
            ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
                ui.monospace(&self.state.output);
            });
        });

        // If we just finished a job (check for trailing exit status marker), flip running off.
        if self.state.running && self.state.output.contains("[exit status:") {
            // Heuristic: the spawned thread appends exit status at the end once.
            // We don't strictly know it's done streaming yet, but this is fine for a minimal tool.
            self.state.running = false;
        }
    }
}

