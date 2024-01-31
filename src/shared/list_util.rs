pub struct SpaceSeparatedList {
    pub str_value: String,
    pub list: Vec<usize>,
}

impl From<String> for SpaceSeparatedList {
    fn from(value: String) -> Self {
        Self {
            str_value: String::from(&value),
            list: value
                .split(" ")
                .map(|str| str.parse::<usize>().unwrap())
                .collect(),
        }
    }
}
