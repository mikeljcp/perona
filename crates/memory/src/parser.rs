use regex::Regex;

#[derive(Debug)]
pub struct Parser {
    message: String,
}

impl Parser {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    fn message(&self) -> Vec<&str> {
        self.message.split(" ").collect()
    }

    fn message_parts_count(&self) -> usize {
        self.message().len()
    }

    fn message_regex(&self) -> Regex {
        Regex::new(r"/[()</FONT><BR>]/g").unwrap()
    }

    pub fn is_valid(&self) -> bool {
        !(self.message.len() < 6 || self.message.len() > 7)
    }

    pub fn get_killer(&self) -> &str {
        self.message()[0].trim()
    }

    pub fn get_dead(&self) -> &str {
        self.message()[2].trim()
    }

    pub fn get_map(&self) -> String {
        let message = self.message();
        let count = self.message_parts_count();

        if count > 6 {
            return format!("{} {}", message[4], message[5]).trim().to_string();
        }

        message[4].trim().to_string()
    }

    fn get_parts(&self, index: usize) -> Vec<String> {
        let message = self.message();

        self
            .message_regex()
            .replace_all(message[index], "")
            .split(",")
            .map(|s| s.to_string())
            .collect()
    }

    pub fn get_coords(&self) -> (i32, i32) {
        let count = self.message_parts_count();
        let coords = self.get_parts(if count > 6 { 6 } else { 5 });
        let x = coords[0].parse().unwrap_or(0);
        let y = coords[1].parse().unwrap_or(0);

        (x, y)
    }
}
