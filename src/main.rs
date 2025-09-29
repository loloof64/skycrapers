use iced::{
    alignment::Horizontal,
    widget::{Column, Row, text_input},
};

fn main() -> iced::Result {
    iced::run("Skycrapers", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    Update(u8, u8, String),
}

impl Default for MyApp {
    fn default() -> Self {
        let default_size = 4;
        let mut dyn_array = Vec::new();
        for _row in 0..default_size {
            let mut col_array = Vec::new();
            for _col in 0..default_size {
                col_array.push("".to_string());
            }
            dyn_array.push(col_array);
        }

        Self {
            size: default_size,
            values: dyn_array,
        }
    }
}

struct MyApp {
    size: u8,
    values: Vec<Vec<String>>,
}

impl MyApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::Update(row, col, value) => {
                if let Ok(num_value) = value.parse::<u8>()
                    && num_value > 0
                    && num_value < 10
                {
                    self.values[row as usize][col as usize] = value;
                    return;
                }
                if value.is_empty() {
                    self.values[row as usize][col as usize] = value;
                }
            }
        }
    }

    fn view(&'_ self) -> iced::Element<'_, Message> {
        let cells_size = 100.0;
        let mut col_comp = Column::new();
        for row_index in 0..self.size {
            let mut row_comp = Row::new();
            for col_index in 0..self.size {
                let cell = text_input(
                    "",
                    self.values[row_index as usize][col_index as usize].as_str(),
                )
                .width(cells_size)
                .size(cells_size)
                .align_x(Horizontal::Center)
                .on_input(move |value| Message::Update(row_index, col_index, value));
                row_comp = row_comp.push(cell);
            }
            col_comp = col_comp.push(row_comp);
        }
        col_comp.into()
    }
}
