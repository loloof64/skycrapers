use iced::{
    alignment::Horizontal,
    widget::{Column, Row, Text, text, text_input},
};

const CELLS_SIZE: f32 = 40.0;

fn main() -> iced::Result {
    iced::run("Skycrapers", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    Update(u8, u8, String),
}

struct MyApp {
    size: u8,
    values: Vec<Vec<String>>,
    /*
       index 0 is the top line
       index 1 is the bottom line
       index 2 is the left line
       index 3 is the right line
    */
    clues: Vec<Vec<u8>>,
}

impl MyApp {
    fn build_clue_component(&'_ self, clue: u8) -> Text<'_> {
        text(format!(
            "{}",
            if clue != 0 {
                clue.to_string()
            } else {
                String::new()
            }
        ))
        .width(CELLS_SIZE)
        .size(CELLS_SIZE)
        .align_x(Horizontal::Center)
    }

    fn build_empty_clue_component(&'_ self) -> Text<'_> {
        text("")
            .width(CELLS_SIZE)
            .size(CELLS_SIZE)
            .align_x(Horizontal::Center)
    }
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

        let mut dyn_clues = Vec::new();
        for _row in 0..4 {
            let mut col_array = Vec::new();
            for _col in 0..default_size {
                col_array.push(0);
            }
            dyn_clues.push(col_array);
        }

        Self {
            size: default_size,
            values: dyn_array,
            clues: dyn_clues,
        }
    }
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
        let mut col_comp = Column::new();

        let mut top_clues = Row::new();
        top_clues = top_clues.push(self.build_empty_clue_component());
        for col_index in 0..self.size {
            let clue_comp = self.build_clue_component(self.clues[0][col_index as usize]);
            top_clues = top_clues.push(clue_comp);
        }
        col_comp = col_comp.push(top_clues);

        for row_index in 0..self.size {
            let mut row_comp = Row::new();

            let left_clue_comp = self.build_clue_component(self.clues[2][row_index as usize]);
            let right_clue_comp = self.build_clue_component(self.clues[3][row_index as usize]);

            row_comp = row_comp.push(left_clue_comp);

            for col_index in 0..self.size {
                let cell = text_input(
                    "",
                    self.values[row_index as usize][col_index as usize].as_str(),
                )
                .width(CELLS_SIZE)
                .size(CELLS_SIZE * 0.8)
                .align_x(Horizontal::Center)
                .on_input(move |value| Message::Update(row_index, col_index, value));
                row_comp = row_comp.push(cell);
            }

            row_comp = row_comp.push(right_clue_comp);

            col_comp = col_comp.push(row_comp);
        }

        let mut bottom_clues = Row::new();
        bottom_clues = bottom_clues.push(self.build_empty_clue_component());
        for col_index in 0..self.size {
            let clue_comp = self.build_clue_component(self.clues[1][col_index as usize]);
            bottom_clues = bottom_clues.push(clue_comp);
        }
        col_comp = col_comp.push(bottom_clues);

        col_comp.into()
    }
}
