use crate::app::App;
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

pub fn setup_app(data: Vec<String>) -> App{
    App::new((3, 1), (10, 5), data ,motions, visuals)
}

fn update(){
    
}