use std::io;
use genpdf::{Document, elements, fonts};

struct Student {
    name: String,
    total_marks: f32,
    subjects: u32,
}

impl Student {
    fn average(&self) -> f32 {
        self.total_marks / self.subjects as f32
    }

    fn grade(&self) -> char {
        let avg = self.average();
        if avg >= 90.0 {
            'A'
        } else if avg >= 75.0 {
            'B'
        } else if avg >= 60.0 {
            'C'
        } else {
            'D'
        }
    }
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut subjects = String::new();

    println!("Enter Student Name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter Total Marks:");
    io::stdin().read_line(&mut total_marks).unwrap();

    println!("Enter Number of Subjects:");
    io::stdin().read_line(&mut subjects).unwrap();

    let student = Student {
        name: name.trim().to_string(),
        total_marks: total_marks.trim().parse().unwrap(),
        subjects: subjects.trim().parse().unwrap(),
    };

    println!("\n--- Report Card ---");
    println!("Name: {}", student.name);
    println!("Average: {:.2}", student.average());
    println!("Grade: {}", student.grade());

    // Load font
    let font_family = fonts::from_files("./", "LiberationSerif", None)
        .expect("Failed to load font");

    // Create a PDF document
    let mut doc = Document::new(font_family.clone());
    doc.set_title("Student Report Card");

    // Add content
    doc.push(elements::Paragraph::new("Student Report Card"));
    doc.push(elements::Break::new(1));
    doc.push(elements::Paragraph::new(format!("Name: {}", student.name)));
    doc.push(elements::Paragraph::new(format!("Average: {:.2}", student.average())));
    doc.push(elements::Paragraph::new(format!("Grade: {}", student.grade())));

    // Render to file
    doc.render_to_file("report_card.pdf").expect("Failed to write PDF");

    println!("\nPDF generated successfully: report_card.pdf ✅");
}
