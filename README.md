# Meowfetch

Meowfetch is a simple and lightweight system information tool written in Rust. It displays various system details such as the operating system, kernel version, hostname, username, cpu, memory and swap usage, ip address, and more.

## Installation

To install Meowfetch, simply clone the repository and build it using Cargo:
```bash
git clone https://github.com/PengweeWang/meowfetch.git
cd meowfetch
cargo install --path .
```

Alternatively, you can download a pre-compiled binary from the [releases page](https://github.com/PengweeWang/meowfetch/releases).

## Usage

To run Meowfetch, simply type `meowfetch` in your terminal.
Here is an example output of Meowfetch:

![meowfetch](./asset/display1.png)


You can also use the `-t` flag to display another type of cat logo (1-3, default: 3).

```bash
meowfetch -t 1
meowfetch --logo /path/to/custom_logo.txt
meowfetch --no-color
```

![meowfetch](./asset/display2.png)


![meowfetch](./asset/display3.png)

## Customization

You can customize the Logo of Meowfetch by modifying the `.meowrc` file at `$HOME/.config/.meowrc`, by [this tool](https://github.com/PengweeWang/terminal_art_creator).


Just draw the logo on the website and touch the export button to copy the string and then paste it to the `.meowrc` file. 

![Website](./asset/tool.png)

Then run `meowfetch` to see the changes.

![meowfetch](./asset/display5.png)


## Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on the GitHub repository. If you want to contribute code, please fork the repository and submit a pull request.

## License

Meowfetch is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more information.


