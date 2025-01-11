use mecab_types::char::CharCategory;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn parse_char_map(line: &str) -> Result<(u16, u16, Vec<&str>)> {
    let fields: Vec<_> = line.split_whitespace().collect();
    let bounds: Vec<_> = fields[0].split("..").collect();

    let (lower, upper) = match bounds.len() {
        1 => {
            let bound = parse_hex(bounds[0])?;
            (bound, bound)
        }
        _ => {
            let lower = parse_hex(bounds[0])?;
            let upper = parse_hex(bounds[1])?;
            (lower, upper)
        }
    };

    let mut categories = Vec::new();

    for &category in &fields[1..] {
        if category == "#" {
            break;
        }

        categories.push(category);
    }

    Ok((lower, upper, categories))
}

pub fn parse_category(line: &str) -> Result<(String, CharCategory)> {
    let fields: Vec<_> = line.split_whitespace().collect();
    let name = fields[0].to_owned();
    let invoke: u8 = fields[1].parse()?;
    let invoke = matches!(invoke, 1);
    let group: u8 = fields[2].parse()?;
    let group = matches!(group, 1);
    let length = fields[3].parse()?;
    let category = CharCategory::new(name.clone(), invoke, group, length);

    Ok((name, category))
}

fn parse_hex(hex: &str) -> Result<u16> {
    let radix = hex.trim_start_matches("0x");
    let parsed = u16::from_str_radix(radix, 16)?;

    Ok(parsed)
}
