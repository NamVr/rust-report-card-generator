# 📝 Rust Report Card Generator

A clean and minimal **Rust-based console application** that:
- Collects student details (name, marks, subjects)
- Calculates the average and grade
- Generates a professional **PDF report card** using the `genpdf` crate

> 📄 Built for internship task: Simple, useful, and entirely offline.

---

## 🚀 Features

✅ CLI interface for user input  
✅ Custom average calculation & grading system  
✅ Generates clean, stylized PDF report cards  
✅ Uses open-source `LiberationSans` font  
✅ PDF saved locally to `output/report_card.pdf`

---

## 📊 Grading Logic

| Average Score | Grade |
|---------------|-------|
| 90+           | A     |
| 75–89         | B     |
| 60–74         | C     |
| Below 60      | D     |

---

## 🛠️ Setup & Run

### 1. Clone the repo

```bash
git clone https://github.com/your-username/rust-report-card-generator.git
cd rust-report-card-generator
```

### 2. Add required fonts

Create a folder called `fonts/` in the root and add:

* `LiberationSans-Regular.ttf`
* `LiberationSans-Bold.ttf`

You can get both from:
[https://github.com/liberationfonts/liberation-fonts/files/1221329](https://github.com/liberationfonts/liberation-fonts/files/1221329)
[https://github.com/liberationfonts/liberation-fonts/files/1221331](https://github.com/liberationfonts/liberation-fonts/files/1221331)

### 3. Build and run

```bash
cargo build
cargo run
```

---

## 📁 Folder Structure

```
rust-report-card-generator/
├── fonts/                # Required TTF fonts
├── output/               # Contains generated report_card.pdf
├── src/
│   └── main.rs           # App logic
├── Cargo.toml
└── README.md
```

---

## ✨ Preview

> Output PDF (`output/report_card.pdf`) includes:

```
Student Report Card
-------------------

Name           : Naman Vrati
Total Marks    : 480
Subjects       : 5
Average Marks  : 96.00
Grade          : A
```

---

## 🤝 License

This project is open-source and free to use for educational or personal purposes.

---