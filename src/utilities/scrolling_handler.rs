use ratatui::widgets::ListState;



pub fn scroll_state_incrementer(scroll_state: &mut ListState, vector_length: &usize){
        let i = match scroll_state.selected() {
            Some(i) => {
                if i >= vector_length - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        scroll_state.select(Some(i));
    }

    pub fn scroll_state_decrementer(scroll_state: &mut ListState, vector_length: &usize) {
        let i = match scroll_state.selected() {
            Some(i) => {
                if i == 0 {
                    vector_length - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        scroll_state.select(Some(i));
    }