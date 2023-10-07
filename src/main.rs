use clap::Parser;
use tabled::{
    builder::Builder,
    settings::{Modify, object::Rows, Alignment, Style}
};

#[derive(Parser)]
struct CliArgs {
    name: String,

    #[arg(short = 'j', long = "json", action)]
    json: bool,
}

struct StringDetail {
    characters: Vec<CharacterDetail>,
    len: usize,
}

struct CharacterDetail {
    character: Option<char>,
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

    fn push(&mut self, character:Option<char>, byte:u8){ self.characters
            .push(CharacterDetail {
                byte_index: self.len,
                character,
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
        let empty = "";
        let mut character = String::from(empty);
        let mut unicode = String::from(empty);
        let mut unicode_hex = String::from(empty);
        match char_detail.character {
            Some(x) => {
                character = String::from(format!("{}", x));
                unicode = String::from(format!("{}", x as u32));
                unicode_hex = String::from(format!("{:x}", x as u32));
            }
            None => {
            }
        };
        let byte = format!("{}", char_detail.byte_index);
        let hex = format!("{:02x}", char_detail.byte);
        let dec = format!("{}", char_detail.byte);
        let bin = format!("{:08b}", char_detail.byte);

        vec![
            unicode,
            unicode_hex,
            character,
            byte,
            hex,
            dec,
            bin]
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
    let cli = CliArgs::parse();
    let utf8 = StringDetail::parse_utf8(&cli.name);
    match cli.json {
        false => utf8.print_table(),
        _ => panic!("Not yet implemented!!"),
    }
}
