use std::env;

use tabled::{
    builder::Builder,
    settings::{Modify, object::Rows, Alignment, Style}
};

struct StringDetail {
    characters: Vec<CharacterDetail>,
    len: usize,
}

struct CharacterDetail {
    character: Option<char>,
    unicode: Option<u32>,
    byte_index: usize,
    byte: u8,
}

impl StringDetail{
    fn parse_utf8(query: &String) -> Self {
        let mut details:StringDetail = StringDetail::default();
        for i in query.chars() {
            let mut bytes = [0; 4];
            i.encode_utf8(&mut bytes);

            details.push(Some(i), bytes[0]);
            for b in 1..i.len_utf8() {
                details.push(None, bytes[b]);
            }
        }

        details
    }

    fn default() -> Self {
        Self { characters: Vec::new(), len: 0 }
    }

    fn push(&mut self, character:Option<char>, byte:u8){
        self.characters
            .push(CharacterDetail {
                byte_index: self.len,
                character,
                unicode: character.map(|c| c as u32),
                byte,
            });
        self.len += 1;
    }

    fn print_table(&self) {
        let mut table_builder = Builder::default();
        table_builder.set_header(StringDetail::table_header());
        for i in self.table_rows() {
            table_builder.push_record(i);
        }

        let table = table_builder.build()
            .with(Style::sharp())
            .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
            .to_string();
        print!("{}", table);
    }

    fn table_rows(&self) -> Vec<Vec<String>> {
        self.characters.iter()
                .map(StringDetail::to_table_row)
                .collect::<Vec<Vec<_>>>()
    }

    fn to_table_row(char_detail: &CharacterDetail) -> Vec<String> {
        let character: String = match char_detail.character {
            Some(x) => String::from(format!("{}", x)),
            None => String::from("<->"),
        };

        let unicode: String = match char_detail.unicode {
            Some(x) => String::from(format!("{}", x)),
            None => String::from("<->"),
        };

        let unicode_hex: String = match char_detail.unicode {
            Some(x) => String::from(format!("{:x}", x)),
            None => String::from("<->"),
        };

        vec![
            unicode,
            unicode_hex,
            character,
            format!("{}", char_detail.byte_index),
            format!("{:02x}", char_detail.byte),
            format!("{}", char_detail.byte),
            format!("{:08b}", char_detail.byte)]
    }

    fn table_header() -> Vec<String> {
        vec![
            String::from("U+dec"),
            String::from("U+hex"),
            String::from("character"),
            String::from("byte"),
            String::from("hex"),
            String::from("dec"),
            String::from("bin"),
        ]
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print!("Usage: enc <string>");
        return;
    }

    let utf8 = StringDetail::parse_utf8(&args[1]);
    utf8.print_table();
}
