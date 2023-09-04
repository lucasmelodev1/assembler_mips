use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub line: usize,
}

impl Label {
    pub fn reference_to_relative_line(
        labels: &Vec<Label>,
        label_name: &str,
        current_line_number: usize,
    ) -> i32 {
        let label_line = Label::find_label_line(labels, &label_name);
        let current_line = current_line_number;
        label_line as i32 - current_line as i32
    }

    pub fn is_label(word: &str) -> bool {
        word.chars().last() == Some(':')
    }

    pub fn is_label_reference(word: &str) -> bool {
        if let Ok(_) = word.parse::<i32>() {
            return false;
        }
        true
    }

    fn find_label_line(labels: &Vec<Label>, label_name: &str) -> usize {        
        labels
            .iter()
            .find(|label| label.name == label_name)
            .expect("Label nao encontrado")
            .line
    }

    pub fn find_label_line_address(labels: &Vec<Label>, label_name: &str, address: bool) -> usize {
        let mul = if address{
            4
        } else {
            1
        };
        (labels
            .iter()
            .find(|label| label.name == label_name)
            .expect("Label nao encontrado")
            .line * mul)
            + 1048576
    }

    pub fn find_labels(file_to_read: &str) -> Vec<Label> {
        let file = File::open(file_to_read).expect("Erro ao abrir arquivo de entrada");
        let reader = BufReader::new(file);
        let mut labels: Vec<Label> = Vec::new();

        for (index, line) in reader.lines().enumerate() {
            let line = line.expect("Erro ao ler linha do arquivo de entrada");

            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() < 1 {
                continue;
            }

            if Label::is_label(words[0]) {
                let mut word = words[0].chars();
                word.next_back();
                labels.push(Label {
                    name: word.as_str().to_string(),
                    line: index,
                })
            }
        }

        labels
    }
}
