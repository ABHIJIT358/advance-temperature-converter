

---

# 🌡️ Advanced Temperature Converter (Rust)

A **command-line based Temperature Converter** built using **Rust**, demonstrating core Rust concepts such as functions, loops, pattern matching, input handling, and error-safe parsing.
This project allows users to convert temperatures between **Celsius, Fahrenheit, and Kelvin** interactively.

---

## 📌 Features

* ✅ Celsius → Fahrenheit
* ✅ Fahrenheit → Celsius
* ✅ Celsius → Kelvin
* ✅ Kelvin → Celsius
* ✅ Interactive menu-driven CLI
* ✅ Continuous execution using loop until user exits
* ✅ Safe numeric input parsing
* ✅ Clean and modular Rust functions

---

## 🛠️ Technologies Used

* **Language:** Rust
* **Standard Library:** `std::io`
* **Platform:** Command Line Interface (CLI)
* **Rust Edition:** 2021 / 2024 compatible

---

## 📂 Project Structure

```
advanced_temperature_converter/
│
├── src/
│   └── main.rs        # Main application logic
│
├── Cargo.toml         # Project configuration
├── Cargo.lock        # Dependency lock file
└── README.md         # Project documentation
```

---

## 🧠 How the Program Works

1. The program runs inside an **infinite loop**.
2. A menu is displayed asking the user to choose a conversion type.
3. Based on user input, the appropriate conversion function is executed.
4. The result is printed to the console.
5. The program continues until the user selects **Exit (Option 5)**.

---

## 🔢 Conversion Logic Explained

### Celsius to Fahrenheit

```
F = (C × 9/5) + 32
```

### Fahrenheit to Celsius

```
C = (F − 32) × 5/9
```

### Celsius to Kelvin

```
K = C + 273.15
```

### Kelvin to Celsius

```
C = K − 273.15
```

Each formula is implemented as a **separate Rust function** to keep the code clean and reusable.

---

## 🧩 Code Highlights

### Reading Safe Numeric Input

```rust
fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse::<f64>().expect("Invalid number")
}
```

* Uses `expect()` for error handling
* Ensures correct floating-point input

---

### Menu Handling Using `match`

```rust
match choice {
    1 => { ... }
    2 => { ... }
    3 => { ... }
    4 => { ... }
    5 => break,
    _ => println!("Invalid choice, try again"),
}
```

* Demonstrates **pattern matching**
* Handles invalid inputs gracefully

---

## ▶️ How to Run the Project

### Step 1: Clone the Repository

```bash
git clone https://github.com/ABHIJIT358/advanced_temperature_converter.git
```

### Step 2: Navigate to Project Folder

```bash
cd advanced_temperature_converter
```

### Step 3: Run the Program

```bash
cargo run
```

---

## 🧪 Sample Output

```
Temperature Converter
1. Celsius to Fahrenheit
2. Fahrenheit to Celsius
3. Celsius to Kelvin
4. Kelvin to Celsius
5. Exit

Enter your choice: 1
Enter Celsius: 25
Result: 77 °F
```

---

## 🎯 Learning Outcomes

By building this project, you learn:

* Rust functions and return types
* `std::io` for user input
* Loops and program flow control
* Pattern matching with `match`
* Floating-point arithmetic
* Clean CLI application structure
* Real-world Rust project workflow

---

## 🚀 Future Enhancements

* 🔹 Add unit tests using `cargo test`
* 🔹 Support Rankine temperature scale
* 🔹 Add input validation without panic
* 🔹 Convert project into a Rust library
* 🔹 Add logging and error enums

---

## 👨‍💻 Author

**Abhijit Wasnik**
Rust Developer | Backend Enthusiast

🔗 GitHub: [https://github.com/ABHIJIT358](https://github.com/ABHIJIT358)

---

## ⭐ Support

If you like this project:

* ⭐ Star the repository
* 🍴 Fork it
* 📌 Use it in your Rust learning journey

---


