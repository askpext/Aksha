# Aksha

Aksha is a high-performance system-wide file search utility for Windows, designed to replicate the efficiency and aesthetics of macOS Spotlight. Built on the Tauri framework, it leverages Rust for the backend indexing engine and React for a modern, responsive frontend interface.

## Overview

Aksha provides an instantaneous, keyboard-centric workflow for locating files and applications. It bypasses the traditional Windows Search indexing mechanism in favor of a custom, highly optimized file crawler, delivering result retrieval in milliseconds. The application runs silently in the background and is invoked via a global hotkey.

## Key Features

*   **High-Performance Indexing:** A custom Rust-based indexing engine traverses the file system with minimal resource overhead, caching file metadata for rapid access.
*   **Instant Search Retrieval:** Search results are populated in real-time as the query is typed, utilizing fuzzy matching algorithms for error tolerance.
*   **Global Hotkey Activation:** Activated instantly from any context using `ctrl + Space`, allowing for seamless integration into existing workflows.
*   **File Preview & Execution:** Supports direct file opening and location navigation. Integrated preview capabilities for common file types.
*   **Minimalist Interface:** Features a clean, distraction-free UI with glassmorphism effects, designed to blend with the modern Windows 11 aesthetic.
*   **Resource Efficiency:** Engineered to maintain a low memory footprint (typically <50MB RAM) when idle.

## Technology Stack

*   **Backend:** Rust (Tauri Core) - Handles file system operations, indexing, and OS integration.
*   **Frontend:** React, TypeScript, Vite - Powers the user interface and interaction logic.
*   **Build System:** Tauri CLI - Manages cross-platform compilation and bundling.

## Installation

### Prerequisites

*   Windows 10 or Windows 11 (64-bit)
*   Microsoft Visual C++ Redistributable 2019 or later

### Download

The latest stable release works out-of-the-box. Download the installer (`.exe`) or MSI package from the [Releases](https://github.com/askpext/Aksha/releases) page.

1.  Download `Aksha_x.x.x_x64_en-US.msi`.
2.  Run the installer and follow the on-screen prompts.
3.  Launch Aksha. Use `ctrl + Space` to toggle the search bar.

## Development Setup

To build Aksha from source, ensure the following dependencies are installed:

*   **Node.js** (LTS version recommended)
*   **Rust** (Stable toolchain via `rustup`)
*   **Visual Studio Build Tools** (C++ workload)

### Build Instructions

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/askpext/Aksha.git
    cd Aksha
    ```

2.  **Install frontend dependencies:**
    ```bash
    npm install
    ```

3.  **Run in development mode:**
    Starts the Tauri application with hot-module replacement (HMR).
    ```bash
    npm run tauri dev
    ```

4.  **Build for production:**
    Compiles the optimized release binary and creates the installer.
    ```bash
    npm run tauri build
    ```

## License

This project is open-source and available under the MIT License.
