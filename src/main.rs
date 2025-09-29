use iced::{
    alignment::Horizontal,
    widget::{Column, Row, Text, button, column, row, text, text_input},
};

mod puzzle_creator;
use puzzle_creator::{create_puzzle, generate_complete_grid};

const CELLS_SIZE: f32 = 40.0;

fn main() -> iced::Result {
    iced::run("Skycrapers", MyApp::update, MyApp::view)
}

#[derive(Debug, Clone)]
enum Message {
    Update(u8, u8, String),
    NewGame,
}

struct MyApp {
    size: u8,
    values: Vec<Vec<String>>,
    answer: Vec<Vec<u8>>,
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

    fn build_grid_comp(&'_ self) -> Column<'_, Message> {
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

        col_comp
    }

    fn build_toolbar_comp(&self) -> Row<'_, Message> {
        let new_game_button = button("New game").on_press(Message::NewGame);
        row![new_game_button]
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

        let mut dyn_answer = Vec::new();
        for _row in 0..default_size {
            let mut col_answer = Vec::new();
            for _col in 0..default_size {
                col_answer.push(0u8);
            }
            dyn_answer.push(col_answer);
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
            answer: dyn_answer,
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
            Message::NewGame => {
                let grid: Vec<Vec<u8>>;
                loop {
                    if let Some(inner_grid) = generate_complete_grid(self.size as usize) {
                        grid = inner_grid;
                        break;
                    }
                }
                let orig_clues = create_puzzle(&grid, self.size as usize);
                let mut clues = Vec::new();

                let mut top_clues = Vec::new();
                for col in 0..self.size {
                    top_clues.push(orig_clues[col as usize]);
                }
                clues.push(top_clues);

                let mut bottom_clues = Vec::new();
                for col in 0..self.size {
                    bottom_clues.push(orig_clues[(col + 2 * self.size) as usize]);
                }
                clues.push(bottom_clues);

                let mut left_clues = Vec::new();
                for col in 0..self.size {
                    left_clues.push(orig_clues[(col + 3 * self.size) as usize]);
                }
                clues.push(left_clues);

                let mut right_clues = Vec::new();
                for col in 0..self.size {
                    right_clues.push(orig_clues[(col + self.size) as usize]);
                }
                clues.push(right_clues);

                self.clues = clues;
                self.answer = grid;
            }
        }
    }

    fn view(&'_ self) -> iced::Element<'_, Message> {
        let grid = self.build_grid_comp();
        let tool_bar = self.build_toolbar_comp();

        column![tool_bar, grid].into()
    }
}
