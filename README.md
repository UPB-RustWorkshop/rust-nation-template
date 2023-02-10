# tock-project
This directory contains libraries and example applications for developing 
Tock apps that sit above the kernel.

## Getting Started
In order to be able to use tock and flash it on the board using **Windows**, you will need the following VM:
https://tock-book.s3.us-west-1.amazonaws.com/VM/TockDev.ova

If you use **MacOS** or **Linux** as working environment, you will need to follow the steps bellow.

## Requirements

In order to use the repository on your local machine, you will need to install the following dependencies:

1. Update all the submodules present in this repository
    ```
    git submodule update --init
    ```

2. Install rust
    
    ```
    curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
    ```

3. The main requirement to build the C applications in this repository is having
   cross compilers for embedded targets. You will need an `arm-none-eabi`
   toolchain for Cortex-M targets.

   **MacOS**:
   ```
   $ brew tap ARMmbed/homebrew-formulae && brew update && brew install arm-none-eabi-gcc
   ```

   **Ubuntu (18.04LTS or later)**:
   ```
   $ sudo apt install gcc-arm-none-eabi
   ```

   **Arch**:
   ```
   $ sudo pacman -Syu arm-none-eabi-gcc arm-none-eabi-newlib
   ```

   **Fedora**:
   ```
   $ sudo dnf install arm-none-eabi-newlib arm-none-eabi-gcc-cs
   ```
