<h1 align="center">
<image src="assets/flowlab_logo.svg" width="350">
</h1>

<p align="center">
An open-source application to control and monitor lab equipments.
</p>

## Table of Contents

- [Table of Contents](#table-of-contents)
- [About](#about)
- [Disclaimer](#disclaimer)
- [Roadmap](#roadmap)
  - [Application library](#application-library)
  - [MVP application](#mvp-application)
  - [User interface](#user-interface)
- [License](#license)

## About

Flowlab is an open-source application written in Rust to control and monitor lab equipments. It is designed to be modular and extensible, allowing users to easily add new devices and instructions depending on their needs.  

⚠️ Warning: This project is still in its early stages of development, so it is not ready for production use yet.

## Disclaimer

This application is being developed as part of my undergraduate engineering physics term paper at the Universidade Federal do Rio Grande do Sul (UFRGS). Therefore, although it is planned to be a general-purpose application, it is currently being developed with the needs of the Physics Institute of UFRGS in mind, more specifically to control and monitor the CMAG9 criostat of the Laboratório de Resistividade, Magnetismo e Supercondutividade (LabRMS).

## Roadmap

### Application library
- [x] Develop the building blocks of the application library:
  - [x] Device configuration abstraction
  - [x] Device commands abstraction
  - [x] Pipeline declaration abstraction
- [ ] Device protocol implementations:
  - [x] TCP/IP
  - [ ] GPIB
  - [ ] Serial

### MVP application
- [ ] Develop the MVP application:
  - [ ] Device configuration
  - [ ] Device commands
  - [ ] Pipeline declaration
  - [ ] Pipeline execution

### User interface
- [ ] CLI application
- [ ] TUI application
- [ ] GUI application

## License

This project is licensed under the GNU GPLv3 license. See the [LICENSE](LICENSE) file for details.