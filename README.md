# Proedufi

> **WORK IN PROGRESS:** This software is currently in active early development. The features listed below are planned for the future and are not yet implemented!

**Proedufi** (**Pro**jeto **Edu**cação **Fi**nanceira / Financial Education Project) is a command-line interface (CLI) application written in **Rust**. It aims to improve and encourage financial education, promoting healthier financial habits and offering accessible, easy-to-understand expense control directly from your terminal.

## Planned Features
- **User Management:** Create accounts, securely log in, or continue as a guest.
- **Family Account Control:** Manage and monitor multiple financial accounts for your family members in a centralized place.
- **Income & Expense Tracking:** Comprehensive control over your cash flow. Easily log and track your earnings (wins) and expenses (losses) to maintain a healthy budget.
- **Data Validation:** Robust system to ensure valid names, properly formatted emails, and strong passwords (requiring uppercase, lowercase, numbers, and special characters).

## Prerequisites

Before you begin, ensure you have the following dependencies installed on your Linux environment:

- `cargo`
- `rustc`

## Installation & Setup

1. **Navigate to your desired applications directory:**
e.g:
   ```bash
   cd ~/Applications
   ```

2. **Clone the repository and build from source:**
   ```bash
   git clone https://github.com/RyanHangai/proedufi.git
   cd proedufi
   make
   ```

## Usage

To run the application, execute the generated binary from the project folder:

```bash
./proedufi
```

> **Note:** The software will only run successfully if it is executed from the folder where it was built.
