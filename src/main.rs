use tber::{app::App, screen::Screen, window::Window, editor::text_editor};

fn visuals(s: (u16, u16), size: (u16, u16), data: &Vec<String>) -> Vec<String> {
    let mut data_virtual: Vec<String> = Vec::new();
    for i in s.1..s.1 + size.1 {
        if i < data.len() as u16 {
            if data[i as usize].len() >= s.0 as usize + size.0 as usize {
                data_virtual.push(
                    data[i as usize].clone()[s.0 as usize..s.0 as usize + size.0 as usize]
                        .to_string(),
                );
            } else {
                data_virtual.push(
                    data[i as usize].clone()[s.0 as usize..data[i as usize].len()].to_string(),
                );
            }
        }
    }
    data_virtual
}
fn motions(
    (_x, _y, _x_s, _y_s): (u16, u16, u16, u16),
    data: &Vec<String>,
) -> (u16, u16, u16, u16, Vec<String>) {
    (0, 0, 0, 0, data.to_vec())
}

fn main() {
    // let mut vector: Vec<String> = Vec::new();
    // vector.push(String::from("1"));
    // vector.push(String::from("2"));
    // vector.push(String::from("3"));
    // vector.push(String::from("4"));

    let mut screen = Screen::new();

    
    //let b = App::new((1,1), (1,5),dv2 ,(0,0,0,0), motions);
    //text_editor = text_editor.add(b);
    let window = text_editor::setup_window();
    screen = screen.add(window);

    // let mut vector: Vec<String> = Vec::new();
    // vector.push(String::from("1"));
    // vector.push(String::from("2"));
    // vector.push(String::from("3"));
    // vector.push(String::from("4"));
    // vector.push(String::from("5"));
    // let file_explorer = Window::new("File Explorer".to_string(), Vec::new(), (0, 0), (10, 10));
    //let b = App::new((1,1), (1,5),dv2 ,(0,0,0,0), motions);
    //file_explorer = file_explorer.add(b);
    //screen = screen.add(file_explorer);
    screen = screen.update();
    screen.render();
}
