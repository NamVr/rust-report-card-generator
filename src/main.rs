use std::io;

// Define a struct to store student details
struct Student {
    name: String,
    total_marks: f32,
    subjects: u32,
}

// Function to calculate average
fn calculate_average(total_marks: f32, subjects: u32) -> f32 {
    total_marks / subjects as f32
}

// Function to assign grade based on average
fn assign_grade(avg: f32) -> char {
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


fn main() {
    // Input: name
    println!("Enter student's name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read name");
    let name = name.trim().to_string();

    // Input: total marks
    println!("Enter total marks:");
    let mut total_marks_input = String::new();
    io::stdin().read_line(&mut total_marks_input).expect("Failed to read marks");
    let total_marks: f32 = total_marks_input.trim().parse().expect("Invalid number");

    // Input: number of subjects
    println!("Enter number of subjects:");
    let mut subjects_input = String::new();
    io::stdin().read_line(&mut subjects_input).expect("Failed to read subjects");
    let subjects: u32 = subjects_input.trim().parse().expect("Invalid number");

    // Create student
    let student = Student {
        name,
        total_marks,
        subjects,
    };

    // Calculate average and grade
    let average = calculate_average(student.total_marks, student.subjects);
    let grade = assign_grade(average);

    // Display results
    println!("\n🎓 Report Card:");
    println!("Name           : {}", student.name);
    println!("Total Marks    : {}", student.total_marks);
    println!("Subjects       : {}", student.subjects);
    println!("Average Marks  : {:.2}", average);
    println!("Grade          : {}", grade);

    // Generate PDF Report
    generate_pdf(&student.name, student.total_marks, student.subjects, average, grade);
}

use genpdf::{elements, style, Alignment, Document};
use genpdf::Element;
use genpdf::fonts;

fn generate_pdf(name: &str, total_marks: f32, subjects: u32, average: f32, grade: char) {
    // Load built-in font (LiberationSans)
    let font_family = genpdf::fonts::from_files("./fonts", "LiberationSans", None)
        .expect("Failed to load font");

    let mut doc = Document::new(font_family);
    doc.set_title("Report Card");
    doc.set_minimal_conformance();

    // Styles
    let title_style = style::Style::new().bold().with_font_size(24);
    let header_style = style::Style::new().bold().with_font_size(16);
    let label_style = style::Style::new().bold();
    let value_style = style::Style::new();

    // Title
    doc.push(
        elements::Paragraph::new("Student Report Card")
            .aligned(Alignment::Center)
            .styled(title_style),
    );
    doc.push(elements::Break::new(2));

    // Student Info Table
    let mut table = elements::TableLayout::new(vec![1, 2]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));
    table.row()
        .element(elements::Paragraph::new("Name").styled(label_style.clone()))
        .element(elements::Paragraph::new(name).styled(value_style.clone()))
        .push().expect("Failed to add row");
    table.row()
        .element(elements::Paragraph::new("Total Marks").styled(label_style.clone()))
        .element(elements::Paragraph::new(format!("{}", total_marks)).styled(value_style.clone()))
        .push().expect("Failed to add row");
    table.row()
        .element(elements::Paragraph::new("Subjects").styled(label_style.clone()))
        .element(elements::Paragraph::new(format!("{}", subjects)).styled(value_style.clone()))
        .push().expect("Failed to add row");
    table.row()
        .element(elements::Paragraph::new("Average Marks").styled(label_style.clone()))
        .element(elements::Paragraph::new(format!("{:.2}", average)).styled(value_style.clone()))
        .push().expect("Failed to add row");
    table.row()
        .element(elements::Paragraph::new("Grade").styled(label_style.clone()))
        .element(elements::Paragraph::new(format!("{}", grade)).styled(value_style.clone()))
        .push().expect("Failed to add row");

    doc.push(table);
    doc.push(elements::Break::new(2));

    // Footer
    doc.push(
        elements::Paragraph::new("Signature: ________________________")
            .aligned(Alignment::Right)
            .styled(header_style),
    );

    // Write to file
    std::fs::create_dir_all("output").expect("Failed to create output directory");
    let output = std::fs::File::create("output/report_card.pdf").expect("Failed to create PDF");
    doc.render(output).expect("Failed to render PDF");

    println!("✅ PDF generated successfully → ./output/report_card.pdf");
}