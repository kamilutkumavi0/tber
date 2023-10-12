mod text_box;
use crate::window::Window;
use text_box::setup_app;

pub fn setup_window()->Window{
    let mut vector: Vec<String> = Vec::new();
    vector.push(String::from("Kamil U"));
    vector.push(String::from("Kamil Utku"));
    vector.push(String::from("Kamil Utku"));
    vector.push(String::from("Kamil Utku"));
    let app = setup_app(vector);
    let mut text_editor = Window::new("Text Editor".to_string(), Vec::new(), (80, 0), (50, 8));
    text_editor = text_editor.add(app);
    text_editor
}

fn update(){

}