use std::fmt;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

impl fmt::Display for SpreadsheetCell {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpreadsheetCell::Int(i) => write!(formatter, "{}", i),
            SpreadsheetCell::Float(f) => write!(formatter, "{}", f),
            SpreadsheetCell::Text(s) => write!(formatter, "{}", s),
        }
    }
}

struct VecSpreadsheetCell<'a>(&'a Vec<SpreadsheetCell>);

impl fmt::Display for VecSpreadsheetCell<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            write!(formatter, "{}", item)?;
            if i < self.0.len() - 1 {
                write!(formatter, ", ")?;
            }
        }
        write!(formatter, "]")
    }
}

fn main() {
    let mut row = Vec::new();

    let wrapper = VecSpreadsheetCell(&row);

    println!("{}", wrapper);

    row.push(SpreadsheetCell::Int(3));
    row.push(SpreadsheetCell::Text("blue".to_string()));
    row.push(SpreadsheetCell::Float(10.12));

    let wrapper = VecSpreadsheetCell(&row);
    println!("{}", wrapper);
}
