<a name="readme-top"></a>

<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
<div align="center">
  <h3 align="center">fsren - CLI rename app</h3>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

This is a simple CLI file renaming app I wrote in Rust, mainly as a learning
project and for my own use.
I download a lot of files from the internet, and their file names are always a 
mess—some have spaces, others use capital letters. For backing them up, I needed 
a way to clean up and standardize the names. A bash script would have worked fine, 
but since I also use Windows sometimes, I decided to build this app instead.
I kept it as a CLI tool because it’s straightforward, works well in bash scripts, 
and makes automating backups much easier.
It uses several crates:
- [https://crates.io/crates/clap](clap)
- [https://crates.io/crates/walkdir](walkdir)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

* RUST 1.84

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### How to Build

I could add actions and build it automatically, but this time I decided that it
is not necessary.

1. install Rust
2. `git clone https://github.com/riccione/fsren`
3. run `cargo build --release` -> this will do a magic and under
   `target/release/` get `fsren`

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

### Prerequisites

To build install Rust compiler [https://www.rust-lang.org/](https://www.rust-lang.org/)

### Installation

No installation - portable, one executable file

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
## Usage

rename all files under a given dir and subdirs using rules:
- lowercase filenames
- spaces, () replaced by _

`fsren <path to dir>`

rename all files under a given dir and subdirs using rules above +:
- limit filename to n characters
`fsren <path to dir> -l <n>`

or

`fsren <path to dir> --limit <n>`

verbosity (by default is false)
`fsren <path to dir> -v`

dry-run (by default is false)
`fsren <path to dir> -d`

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->
## Roadmap

Not sure anymore that encryption is necessary... need to re-think
rename all files under a given dir and subdirs using rules above +:
- encrypt filenames with AES and base32
- key is randomly generated
`fsren <path to dir> -e`

rename all files under a given dir and subdirs using rules above +:
- encrypt files with AES and base32
- with a provided key
`fsren <path to dir> -e -k <path to the key>`

- generate random key in the given dir
`fsren -g <path to the dir>`

rename all files under a given dir and subdirs using rules above +:
- decrypt files with AES and base32
- with a provided key
`fsren <path to dir> -d -k <path to the key>`

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

No reason to contact with me ^_-.
Just create an issue if you need something.

Project Link:
[https://github.com/riccionee/hermes](https://github.com/riccione/hermes)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Choose an Open Source License](https://choosealicense.com)
* [Rust](https://www.rust-lang.org/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>
