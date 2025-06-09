use iced::widget::{column, text, text_input};
use iced::{Alignment, Element, Settings};


struct Destination_Service {
    destination_service_name: String,
}

enum Message {
    add_destination_service,
}

impl Destination_Service {
    fn update(&mut self, message: Message) {
        match message {
            Message::add_destination_service => {
                self.destination_service_name = "New String?".to_string();
            }
        }
    }

}