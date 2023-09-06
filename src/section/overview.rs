use crate::imports::*;

pub struct Overview {
    sender : Sender<Events>,

}

impl Overview {
    pub fn new(sender : Sender<Events>) -> Self {
        Self {
            sender,
        }
    }
}


impl SectionT for Overview {
    fn render(&mut self, _wallet : &mut Wallet, _ctx: &egui::Context, _frame: &mut eframe::Frame, ui : &mut egui::Ui) {
        ui.heading("Overview");
        ui.separator();
        ui.label("This is the overview page");
    }
}