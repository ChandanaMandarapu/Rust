// Library Management System - Advanced Enum Practice
// for to learnnn nested enums, enum methods, pattern matching with enums

use std::fmt;

#[derive(Debug, Clone)]
enum BookStatus {
    Available,
    CheckedOut { borrower: String, due_date: String },
    Reserved { reserver: String },
    UnderMaintenance { reason: String },
    Lost,
}

#[derive(Debug, Clone)]
enum BookFormat {
    Physical { isbn: String, location: String },
    Digital { file_size_mb: u32, download_url: String },
    AudioBook { duration_minutes: u32, narrator: String },
}

#[derive(Debug)]
enum LibraryEvent {
    BookAdded { title: String, format: BookFormat },
    BookCheckedOut { book_id: u32, borrower: String },
    BookReturned { book_id: u32, condition: BookCondition },
    FineIssued { borrower: String, amount: f64, reason: String },
    MembershipCreated { member_id: u32, name: String, tier: MembershipTier },
}

#[derive(Debug, Clone)]
enum BookCondition {
    Excellent,
    Good,
    Fair,
    Poor,
    Damaged { description: String },
}

#[derive(Debug, Clone)]
enum MembershipTier {
    Basic { max_books: u32 },
    Premium { max_books: u32, discount_percent: f64 },
    VIP { max_books: u32, discount_percent: f64, priority_reservations: bool },
}

impl BookStatus {
    fn is_available(&self) -> bool {
        matches!(self, BookStatus::Available)
    }

    fn description(&self) -> String {
        match self {
            BookStatus::Available => "Available for checkout".to_string(),
            BookStatus::CheckedOut { borrower, due_date } => {
                format!("Checked out by {} (due: {})", borrower, due_date)
            }
            BookStatus::Reserved { reserver } => {
                format!("Reserved for {}", reserver)
            }
            BookStatus::UnderMaintenance { reason } => {
                format!("Under maintenance: {}", reason)
            }
            BookStatus::Lost => "Reported as lost".to_string(),
        }
    }
}

impl BookFormat {
    fn storage_cost(&self) -> f64 {
        match self {
            BookFormat::Physical { .. } => 2.50,
            BookFormat::Digital { file_size_mb, .. } => {
                (*file_size_mb as f64) * 0.01
            }
            BookFormat::AudioBook { duration_minutes, .. } => {
                (*duration_minutes as f64) * 0.05
            }
        }
    }

    fn access_method(&self) -> String {
        match self {
            BookFormat::Physical { location, .. } => {
                format!("Pick up from: {}", location)
            }
            BookFormat::Digital { download_url, .. } => {
                format!("Download from: {}", download_url)
            }
            BookFormat::AudioBook { narrator, .. } => {
                format!("Stream audio narrated by {}", narrator)
            }
        }
    }
}

impl MembershipTier {
    fn calculate_fine(&self, base_fine: f64) -> f64 {
        match self {
            MembershipTier::Basic { .. } => base_fine,
            MembershipTier::Premium { discount_percent, .. } => {
                base_fine * (1.0 - discount_percent / 100.0)
            }
            MembershipTier::VIP { discount_percent, .. } => {
                base_fine * (1.0 - discount_percent / 100.0)
            }
        }
    }

    fn max_borrowing_limit(&self) -> u32 {
        match self {
            MembershipTier::Basic { max_books } => *max_books,
            MembershipTier::Premium { max_books, .. } => *max_books,
            MembershipTier::VIP { max_books, .. } => *max_books,
        }
    }
}

impl fmt::Display for LibraryEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryEvent::BookAdded { title, format } => {
                write!(f, "New book added: '{}' ({:?})", title, format)
            }
            LibraryEvent::BookCheckedOut { book_id, borrower } => {
                write!(f, "Book #{} checked out by {}", book_id, borrower)
            }
            LibraryEvent::BookReturned { book_id, condition } => {
                write!(f, "Book #{} returned in {:?} condition", book_id, condition)
            }
            LibraryEvent::FineIssued { borrower, amount, reason } => {
                write!(f, "Fine issued to {}: ${:.2} ({})", borrower, amount, reason)
            }
            LibraryEvent::MembershipCreated { member_id, name, tier } => {
                write!(f, "New member #{}: {} ({:?})", member_id, name, tier)
            }
        }
    }
}

struct Library {
    events: Vec<LibraryEvent>,
}

impl Library {
    fn new() -> Self {
        Library { events: Vec::new() }
    }

    fn log_event(&mut self, event: LibraryEvent) {
        println!("EVENT: {}", event);
        self.events.push(event);
    }

    fn process_return(&self, condition: &BookCondition) -> Option<f64> {
        match condition {
            BookCondition::Excellent | BookCondition::Good => None,
            BookCondition::Fair => Some(5.0),
            BookCondition::Poor => Some(15.0),
            BookCondition::Damaged { description } => {
                println!("Damage report: {}", description);
                Some(50.0)
            }
        }
    }
}

fn main() {
    let mut library = Library::new();

    // creting various book formats
    let physical_book = BookFormat::Physical {
        isbn: "978-0-123456-78-9".to_string(),
        location: "Section A, Shelf 3".to_string(),
    };

    let digital_book = BookFormat::Digital {
        file_size_mb: 45,
        download_url: "https://lib.example.com/book123".to_string(),
    };

    let audio_book = BookFormat::AudioBook {
        duration_minutes: 420,
        narrator: "Jane Smith".to_string(),
    };

    println!("=== Book Format Storage Costs ===");
    println!("Physical: ${:.2}", physical_book.storage_cost());
    println!("Digital: ${:.2}", digital_book.storage_cost());
    println!("AudioBook: ${:.2}\n", audio_book.storage_cost());

    // Log library events
    library.log_event(LibraryEvent::BookAdded {
        title: "Rust Programming".to_string(),
        format: physical_book.clone(),
    });

    library.log_event(LibraryEvent::MembershipCreated {
        member_id: 1001,
        name: "Alice Johnson".to_string(),
        tier: MembershipTier::Premium {
            max_books: 10,
            discount_percent: 20.0,
        },
    });

    library.log_event(LibraryEvent::BookCheckedOut {
        book_id: 1,
        borrower: "Alice Johnson".to_string(),
    });

    // Process book returns with different conditions
    println!("\n=== Processing Returns ===");
    let conditions = vec![
        BookCondition::Excellent,
        BookCondition::Fair,
        BookCondition::Damaged {
            description: "Water damage on pages 50-75".to_string(),
        },
    ];

    for condition in &conditions {
        if let Some(fine) = library.process_return(condition) {
            println!("Fine for {:?}: ${:.2}", condition, fine);
        } else {
            println!("No fine for {:?}", condition);
        }
    }

    // Test membership tiers
    println!("\n=== Membership Tier Tests ===");
    let tiers = vec![
        MembershipTier::Basic { max_books: 3 },
        MembershipTier::Premium {
            max_books: 10,
            discount_percent: 20.0,
        },
        MembershipTier::VIP {
            max_books: 20,
            discount_percent: 50.0,
            priority_reservations: true,
        },
    ];

    let base_fine = 10.0;
    for tier in &tiers {
        println!(
            "{:?}: Limit={}, Fine=${:.2}",
            tier,
            tier.max_borrowing_limit(),
            tier.calculate_fine(base_fine)
        );
    }

    // Test book statuses
    println!("\n=== Book Status Examples ===");
    let statuses = vec![
        BookStatus::Available,
        BookStatus::CheckedOut {
            borrower: "Bob Smith".to_string(),
            due_date: "2026-01-15".to_string(),
        },
        BookStatus::Reserved {
            reserver: "Carol White".to_string(),
        },
        BookStatus::UnderMaintenance {
            reason: "Rebinding required".to_string(),
        },
    ];

    for status in &statuses {
        println!("{} - Available: {}", status.description(), status.is_available());
    }

    println!("\n=== Total Events Logged: {} ===", library.events.len());
}