use crate::scraper::core::JobListing;
use crate::config::Config;
use crate::scraper::storage::Storage;
use eframe::{egui, NativeOptions};
use std::path::PathBuf;

/// Launches the native UI application.
pub fn run_ui(config: &Config) -> Result<(), eframe::Error> {
    let db_path = config.db_path.clone();
    let storage = Storage::new(&db_path).expect("Failed to initialize storage for UI");
    let jobs = storage.get_all().unwrap_or_default();
    let app = MyApp::new(jobs, db_path);
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Job Organizer",
        native_options,
        Box::new(move |_cc| Box::new(app)),
    )
}

/// Application state for the UI.
pub struct MyApp {
    jobs: Vec<JobListing>,
    db_path: PathBuf,
    selected_job_id: Option<String>,
    show_about_window: bool,
}

impl MyApp {
    fn new(jobs: Vec<JobListing>, db_path: PathBuf) -> Self {
        Self {
            jobs,
            db_path,
            selected_job_id: None,
            show_about_window: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top panel for menu
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about_window = true;
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
            });
        });

        // Left panel: job list
        egui::SidePanel::left("job_list").show(ctx, |ui| {
            ui.heading("Jobs");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for job in &self.jobs {
                    let selected = self
                        .selected_job_id
                        .as_ref()
                        .map(|id| id == &job.id)
                        .unwrap_or(false);
                    if ui.selectable_label(selected, &job.title).clicked() {
                        self.selected_job_id = Some(job.id.clone());
                    }
                }
            });
        });

        // Main panel: job details
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref sel_id) = self.selected_job_id.clone() {
                if let Some(job) = self.jobs.iter_mut().find(|j| j.id == *sel_id) {
                    ui.heading(&job.title);
                    ui.label(format!("Company: {}", job.company));
                    ui.separator();
                    let mut applied = job.is_applied;
                    if ui.checkbox(&mut applied, "Applied").changed() {
                        job.is_applied = applied;
                        // Persist change
                        if let Ok(storage) = Storage::new(&self.db_path) {
                            let _ = storage.set_job_applied(&job.id, applied);
                        }
                    }
                    ui.separator();
                    ui.label(&job.description);
                }
            } else {
                ui.label("Select a job to view details");
            }
        });

        // About window (dialog)
        if self.show_about_window {
            egui::Window::new("About Job Organizer")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.heading("Job Organizer");
                    ui.label("Version 0.1.0");
                    ui.add_space(10.0);
                    ui.label(
                        "This application helps you organize job listings scraped from various platforms.",
                    );
                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        self.show_about_window = false;
                    }
                });
        }
    }
}
