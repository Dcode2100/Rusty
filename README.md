# Rust Application

## Getting Started

This guide will help you set up, run, and debug your Rust application.

### Prerequisites

* **Rust** : Ensure you have Rust installed. You can install Rust using `rustup`. If you haven't installed it yet, follow the instructions [here]().
* **Visual Studio Code (VS Code)** : This guide assumes you are using VS Code for development. If you don't have it installed, download it from [here](https://code.visualstudio.com/).

### Setting Up the Project

1. **Clone the Repository** :

git clone https://github.com/GameDevGraphics/RustDebugging.git
cd Test-project_HowToDebug

1. **Install Dependencies** :
   Run the following command to install the required dependencies:

   cargo build

### Running the Application

1. **Build the Project**cargo build

   For a release build, use:

   cargo build --release
2. **Run the Application** :
   After building, run the application using:

   cargo run

   For the release build, run:

   cargo run --release

### Debugging the Application

1. **Open the Project in VS Code** :
   Open VS Code and navigate to your project directory.
2. **Configure Debugging** :
   Make sure you have the Rust extension installed in VS Code. You can find it in the Extensions Marketplace by searching for "rust-analyzer".
3. **Start Debugging** :

* Set breakpoints in your code as needed.
* Open the Debug view by clicking on the Debug icon in the Activity Bar on the side of the window or by pressing `Ctrl + Shift + D`.
* Select the appropriate configuration from the dropdown menu and start debugging by clicking the green play button or pressing `F5`.

### Common Issues

* **`link.exe` Missing Error** :
* Ensure that you have the Visual Studio Build Tools installed. This includes the C++ build tools that are necessary for linking Rust binaries on Windows.
* If you encounter issues, verify that the build tools are properly installed and that their paths are correctly set in your system environment variables.
