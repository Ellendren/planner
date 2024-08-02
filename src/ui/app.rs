use fltk::{
    prelude::*,
    app,
    window::Window,
};
use crate::ui::widgets::planner_tab;

pub fn run() -> Result<(), FltkError>{
    let app = app::App::default();
    let mut win = Window::new(
        0, 
        0, 
        1000, 
        700, 
        "Ellendren's Planner"
    )
    .center_screen();
    
    planner_tab::PlannerTab::new();

    win.make_resizable(true);
    win.maximize();

    win.end();
    win.show();

    app.run()
}