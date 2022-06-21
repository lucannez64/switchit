#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{
    egui::{
        style::Margin, Align::Center, Button, CentralPanel, Color32, Context, FontData,
        FontDefinitions, FontFamily, Layout, RichText, ScrollArea, TextEdit, Vec2, Visuals,
    },
    run_native, App, Frame, IconData, NativeOptions,
};

// Get the code for every platform
#[cfg(target_os = "windows")]
static CODE: &str = "code.cmd";
#[cfg(target_os = "linux")]
static CODE: &str = "code";
#[cfg(target_os = "macos")]
static CODE: &str = "code";

fn main() {
    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(270.0, 540.0));
    native_options.resizable = false;
    native_options.always_on_top = true;
    native_options.icon_data = Some(load_icon(
        "C:\\Users\\mazav\\OneDrive\\Documents\\things\\switchit\\src\\_icon.png",
    ));
    run_native(
        "Switchit",
        native_options,
        Box::new(|cc| Box::new(Switchit::new(cc))),
    );
}

#[derive(Default, Clone, Debug)]
struct Switchit {
    projects: Vec<Project>,
    fonts: FontDefinitions,
    path: String,
    name: String,
    checked: bool,
    language: String,
}

#[derive(Default, Clone, Debug)]
struct Project {
    name: String,
    path: String,
}

//Load the icon file
// TODO: This should be compiled into the executable
fn load_icon(path: &str) -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

// parse every line of the file and convert it into Option<Project>
fn parse_line(line: &str) -> Option<Project> {
    let mut iter = line.split("{}");
    let key = iter.next().unwrap().to_string();
    let value = iter.next().unwrap().to_string();
    Some(Project {
        name: key,
        path: value,
    })
}

fn parse_file(path: &str) -> Result<Vec<Project>, std::io::Error> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let mut projects: Vec<Project> = Vec::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let p = match parse_line(&line?) {
            Some(p) => p,
            None => continue,
        };
        projects.push(p);
    }
    Ok(projects)
}

fn save(projects: Vec<Project>) {
    use std::env::var;
    use std::fs::OpenOptions;
    use std::io::Write;
    let mut file = OpenOptions::new()
        .write(true)
        .open(var("PROJECTS").unwrap())
        .unwrap();
    let mut s: String = String::new();
    for project in projects {
        s.push_str(&format!("{}{{}}{}\n", project.name, project.path));
    }
    file.set_len(0).unwrap();
    file.write_all(s.as_bytes()).unwrap();
}

impl App for Switchit {
    fn on_exit(&mut self, _gl: &eframe::glow::Context) {
        save(self.projects.clone());
    }

    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.set_fonts(self.fonts.clone());
        let mut visuals = Visuals::dark();
        visuals.extreme_bg_color = Color32::from_rgb(29, 32, 33);
        ctx.set_visuals(visuals);
        CentralPanel::default()
            .frame(
                eframe::egui::Frame::none()
                    .inner_margin(Margin::symmetric(5.0, 10.0))
                    .fill(Color32::from_rgb(40, 40, 40)),
            )
            .show(ctx, |ui| {
                ui.with_layout(Layout::top_down_justified(Center), |ui| {
                    ui.heading(RichText::new("Switchit").color(Color32::from_rgb(104, 157, 106)));
                    ui.add_space(14.0);
                    ui.separator();
                    ScrollArea::vertical()
                        .max_width(f32::INFINITY)
                        .show(ui, |ui| {
                            for (y, project) in self.projects.clone().iter().enumerate() {
                                ui.with_layout(Layout::top_down_justified(Center), |ui| {
                                    let style = RichText::new(project.name.clone())
                                        .size(14.0)
                                        .color(Color32::from_rgb(235, 219, 178));
                                    let button = ui.add(
                                        Button::new(style.to_owned())
                                            .fill(Color32::from_rgb(60, 56, 54)),
                                    );
                                    if button.secondary_clicked() {
                                        self.projects.remove(y);
                                    } else if button.clicked() {
                                        #[cfg(target_os = "windows")]
                                        use std::os::windows::process::CommandExt;
                                        #[cfg(target_os = "windows")]
                                        std::process::Command::new(CODE)
                                            .arg(&project.path)
                                            .arg("-r")
                                            .creation_flags(0x08000000)
                                            .spawn()
                                            .unwrap();
                                        #[cfg(target_os = "macos")]
                                        std::process::Command::new(CODE)
                                            .arg(&project.path)
                                            .arg("-r")
                                            .spawn()
                                            .unwrap();
                                        #[cfg(target_os = "linux")]
                                        std::process::Command::new(CODE)
                                            .arg(&project.path)
                                            .arg("-r")
                                            .arg("&")
                                            .spawn()
                                            .unwrap();
                                    };
                                });
                            }
                        });
                });
                ui.with_layout(Layout::top_down(Center), |ui| {
                    ui.separator();
                    ui.add_space(5.0);
                    let style = RichText::new("Add Project")
                        .size(16.0)
                        .color(Color32::from_rgb(250, 189, 47));
                    ui.strong(style);
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Name : ");
                        let _response = ui.add(TextEdit::singleline(&mut self.name));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Path : ");
                        let _pathr = ui.add(TextEdit::singleline(&mut self.path));
                    });

                    let _response = ui.checkbox(&mut self.checked, "Create new project");
                    if self.checked {
                        ui.horizontal(|ui| {
                            ui.label("Language : ");
                            let _response = ui.add(TextEdit::singleline(&mut self.language));
                        });
                    }
                    let button = ui.button("+");
                    if button.clicked() && !self.path.is_empty() && !self.name.is_empty() {
                        if self.checked && !self.language.is_empty() {
                            #[cfg(target_os = "windows")]
                            use std::os::windows::process::CommandExt;
                            #[cfg(target_os = "windows")]
                            std::process::Command::new("project")
                                .arg("-l")
                                .arg(&self.language.to_owned())
                                .arg("-n")
                                .arg(&self.name.to_owned())
                                .arg("-p")
                                .arg(&self.path.to_owned())
                                .creation_flags(0x08000000)
                                .spawn()
                                .unwrap();
                            #[cfg(target_os = "macos")]
                            std::process::Command::new("project")
                                .arg("-l")
                                .arg(&self.language.to_owned())
                                .arg("-n")
                                .arg(&self.name.to_owned())
                                .arg("-p")
                                .arg(&self.path.to_owned())
                                .spawn()
                                .unwrap();
                            #[cfg(target_os = "linux")]
                            std::process::Command::new("project")
                                .arg("-l")
                                .arg(&self.language.to_owned())
                                .arg("-n")
                                .arg(&self.name.to_owned())
                                .arg("-p")
                                .arg(&self.path.to_owned())
                                .spawn()
                                .unwrap();
                            self.projects.push(Project {
                                name: self.name.to_owned(),
                                path: self.path.to_owned() + &self.name.to_owned(),
                            });

                            self.name = String::new();
                            self.path = String::new();
                            self.language = String::new();
                        } else {
                            self.projects.push(Project {
                                name: self.name.to_owned(),
                                path: self.path.to_owned(),
                            });
                            self.name = String::new();
                            self.path = String::new();
                        }
                    }
                });
            });
    }
}

impl Switchit {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let projects =
            parse_file(&std::env::var("PROJECTS").expect("Invalid environment variable"))
                .expect("Can't parse file");
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Fantasque Sans Mono Bold".to_owned(),
            FontData::from_static(include_bytes!("fonts/FantasqueSansMono-Bold.ttf")),
        );
        fonts.font_data.insert(
            "Fantasque Sans Mono".to_owned(),
            FontData::from_static(include_bytes!("fonts/FantasqueSansMono-Regular.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Fantasque Sans Mono".to_owned());
        Self {
            projects,
            fonts,
            name: String::new(),
            path: String::new(),
            language: String::new(),
            checked: false,
        }
    }
}
