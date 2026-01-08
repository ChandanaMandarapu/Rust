pub struct LogEntry<'a> {
    pub timestamp: &'a str,
    pub level: &'a str,
    pub message: &'a str,
}

pub struct LogAnalyzer<'a> {
    pub source: &'a str,
    pub entries: Vec<LogEntry<'a>>,
}

impl<'a> LogAnalyzer<'a> {
    pub fn new(raw_data: &'a str) -> Self {
        Self {
            source: raw_data,
            entries: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        for line in self.source.lines() {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();

            match parts.as_slice() {
                [ts, lvl, msg] => {
                    self.entries.push(LogEntry {
                        timestamp: ts,
                        level: lvl,
                        message: msg,
                    });
                }
                _ => continue,
            }
        }
    }

    pub fn get_errors(&self) -> Vec<&LogEntry<'a>> {
        self.entries
            .iter()
            .filter(|e| e.level == "ERROR" || e.level == "CRITICAL")
            .collect()
    }
}
