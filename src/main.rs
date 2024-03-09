use clap::{Args, Parser};
use serde::Serialize;
use tabled::{
    builder::Builder,
    settings::{object::Rows, Alignment, Modify, Style},
};

struct StringDetail {
    characters: Vec<CharacterDetail>,
    length: usize,
}

struct CharacterDetail {
    character: Option<char>,
    byte_index: usize,
    byte: u8,
}

impl StringDetail {
    fn parse_utf8(query: &str) -> Self {
        let mut details: StringDetail = StringDetail::default();
        for i in query.chars() {
            let mut bytes = [0; 4];
            let encoded = i.encode_utf8(&mut bytes);

            let mut citer = vec![i].into_iter();
            for b in encoded.bytes() {
                details.push(citer.next(), b);
            }
        }

        details
    }

    fn parse_utf16(query: &String) -> Self {
        let mut details: StringDetail = StringDetail::default();
        for i in query.chars() {
            let mut bytes = [0; 2];
            let encoded = i.encode_utf16(&mut bytes);

            let mut citer = vec![i].into_iter();
            for b in encoded {
                details.push_utf16(citer.next(), *b);
            }
        }

        details
    }

    fn default() -> Self {
        Self {
            characters: Vec::new(),
            length: 0,
        }
    }

    fn push(&mut self, character: Option<char>, byte: u8) {
        self.characters.push(CharacterDetail {
            byte_index: self.length,
            character,
            byte,
        });
        self.length += 1;
    }

    fn push_utf16(&mut self, character: Option<char>, byte: u16) {
        let bytes = byte.to_be_bytes();
        self.push(character, bytes[0]);
        self.push(None, bytes[1]);
    }
}

#[derive(Serialize)]
struct StringTable {
    characters: Vec<StringTableRow>,
    length: usize,
}

#[derive(Serialize)]
struct StringTableRow {
    unicode: String,
    unicode_hex: String,
    character: String,
    byte: String,
    hex: String,
    dec: String,
    bin: String,
}

impl StringTableRow {
    fn from(char_detail: &CharacterDetail) -> Self {
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
            None => {}
        };
        let byte = format!("{}", char_detail.byte_index);
        let hex = format!("{:02x}", char_detail.byte);
        let dec = format!("{}", char_detail.byte);
        let bin = format!("{:08b}", char_detail.byte);

        StringTableRow {
            unicode,
            unicode_hex,
            character,
            byte,
            hex,
            dec,
            bin,
        }
    }

    fn header() -> Vec<String> {
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

    fn to_table_row(self) -> Vec<String> {
        vec![
            self.unicode,
            self.unicode_hex,
            self.character,
            self.byte,
            self.hex,
            self.dec,
            self.bin,
        ]
    }
}

impl StringTable {
    fn from(string_details: &StringDetail) -> Self {
        let characters = string_details
            .characters
            .iter()
            .map(StringTableRow::from)
            .collect::<Vec<StringTableRow>>();

        StringTable {
            characters,
            length: string_details.length,
        }
    }

    fn as_table(self) -> String {
        let mut table_builder = Builder::default();
        table_builder.set_header(StringTableRow::header());
        for i in self.characters {
            table_builder.push_record(i.to_table_row());
        }

        let table = table_builder
            .build()
            .with(Style::sharp())
            .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
            .to_string();

        format!("{}", table)
    }

    fn as_json(self) -> String {
        format!("{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Parser)]
#[command(next_line_help = true)]
struct CliArgs {
    /// The string to inspect
    name: String,

    #[command(flatten)]
    inspect: InspectArgs,

    /// Output as json. Useful as a command line tool
    #[arg(short = 'j', long = "json", action)]
    json: bool,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct InspectArgs {
    /// Inspect utf-8
    #[arg(short = '8', action)]
    utf8: bool,

    /// Inspect utf-16
    #[arg(short = '6', action)]
    utf16: bool,
}

fn main() {
    let cli = CliArgs::parse();
    let details = match cli.inspect.utf8 {
        true => StringDetail::parse_utf8(&cli.name),
        false => StringDetail::parse_utf16(&cli.name),
    };

    let char_table = StringTable::from(&details);

    match cli.json {
        false => println!("{}", char_table.as_table()),
        true => println!("{}", char_table.as_json()),
    }
}
