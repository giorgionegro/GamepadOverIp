<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->

[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]





<!-- ABOUT THE PROJECT -->
## Gamepad Over Ip

Simple program to send gamepad input over UDP to a remote host. (for now only in lan)
<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* [![][Rust]][Rust-url] Rust

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

No requirements for the server, just run the executable.
For the client you need to install  the latest ViGEmBus driver at [releases](https://github.com/ViGEm/ViGEmBus/releases)

### Installation

1. Download the latest release from [releases](https://github.com/giorgionegro/gamepadOverIp/releases)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage
1. execute the server executable
2. execute the client executable on the client machine (you can set the receiving port by passing it as an argument e.g. `./path/name.exe 1234`)



<!-- ROADMAP -->
## Roadmap

- [x] Dinput support
- [ ] Xinput support 
- [ ] Multiple gamepad support
    - [ ] Different client side gamepad types
- [ ] Different client for different platforms 

See the [open issues](https://github.com/giorgionegro/gamepadOverIp/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- Build -->
## Build
1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone the repo
   ```sh
   git clone https://github.com/giorgionegro/gamepadOverIp.git
    ```
3. make your changes
4. Build the project with cargo
   ```sh
   cargo build --release
   ```
5. enjoy the executables in `target/release/`


<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".


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

Giorgio Negro - giorgionegro+github@proton.me

Project Link: [https://github.com/giorgionegro/gamepadOverIp](https://github.com/giorgionegro/gamepadOverIp)

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [gilrs](https://github.com/Arvamer/gilrs)
* [vigem](https://github.com/TheRadioGuy/vigem)
* [ViGEmBus](https://github.com/ViGEm/ViGEmBus/releases)
* [vigem-client](https://github.com/CasualX/vigem-client)
<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/giorgionegro/gamepadOverIp.svg?style=for-the-badge
[contributors-url]: https://github.com/giorgionegro/gamepadOverIp/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/giorgionegro/gamepadOverIp.svg?style=for-the-badge
[forks-url]: https://github.com/giorgionegro/gamepadOverIp/network/members
[issues-shield]: https://img.shields.io/github/issues/giorgionegro/gamepadOverIp.svg?style=for-the-badge
[issues-url]: https://github.com/giorgionegro/gamepadOverIp/issues
[license-shield]: https://img.shields.io/github/license/giorgionegro/gamepadOverIp.svg?style=for-the-badge
[license-url]: https://github.com/giorgionegro/gamepadOverIp/blob/master/LICENSE

[Rust]: https://www.rust-lang.org/logos/rust-logo-16x16.png
[Rust-url]: https://www.rust-lang.org/
[Bootstrap.com]: https://img.shields.io/badge/Bootstrap-563D7C?style=for-the-badge&logo=bootstrap&logoColor=white
[Bootstrap-url]: https://getbootstrap.com
[JQuery.com]: https://img.shields.io/badge/jQuery-0769AD?style=for-the-badge&logo=jquery&logoColor=white
[JQuery-url]: https://jquery.com
