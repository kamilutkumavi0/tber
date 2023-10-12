#[derive(Clone, Debug)]
pub struct App {
    pub start: (u16, u16),
    pub size: (u16, u16),
    curser_loc: (u16, u16, u16, u16), // x y x end y end
    data: Vec<String>,
    pub visual_data: Vec<String>,
    app_motions: fn((u16, u16, u16, u16), Vec<String>) -> (u16, u16, u16, u16, Vec<String>), // every thing will be here for the app
    app_userinterface: fn((u16, u16), (u16, u16), &Vec<String>) -> Vec<String>,
}

impl App {
    pub fn new(
        start: (u16, u16),
        size: (u16, u16),
        data: Vec<String>,
        app_motions: fn((u16, u16, u16, u16), Vec<String>) -> (u16, u16, u16, u16, Vec<String>),
        app_userinterface: fn((u16, u16), (u16, u16), &Vec<String>) -> Vec<String>,
    ) -> Self {
        let visual_data = app_userinterface((0,0), size, &data);
        Self {
            start,
            size,
            curser_loc: (0, 0, 0, 0),
            data,
            visual_data,
            app_motions,
            app_userinterface,
        }
    }

    //pub fn change_data(self, data: Vec<String>){}

    pub fn update(){}
}
