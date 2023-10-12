#[derive(Clone, Debug)]
pub struct App{
    pub start: (u16, u16),
    pub size: (u16, u16),
    curser_loc: (u16, u16, u16, u16), // x y x end y end
    pub visual_data: Vec<String>,
    app_motions: fn((u16, u16, u16, u16), Vec<String>) -> (u16, u16, u16, u16, Vec<String>), // every thing will be here for the app 
    app_userinterface: fn((u16,u16), (u16,u16), &Vec<String>) -> Vec<String>,
}

impl  App {
    pub fn new(start: (u16, u16), size: (u16, u16), app_motions: fn((u16, u16, u16, u16), Vec<String>) -> (u16, u16, u16, u16, Vec<String>), app_userinterface: fn((u16,u16), (u16,u16), &Vec<String>) -> Vec<String>) -> Self{
        Self {start:start, size:size, curser_loc: (0,0,0,0), visual_data: Vec::new(), app_motions:app_motions, app_userinterface: app_userinterface}
    }

    //pub fn update(){}
}