![Zink Banner](zink-banner.png)

# zink! Smart Contract Macros

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Built with ink!](https://raw.githubusercontent.com/paritytech/ink/master/.images/badge.svg)](https://github.com/paritytech/ink)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)

zink! is a library for developing ink! smart contracts. It contains useful Rust macros that extend functionality and reduce boilerplate code.

This library still has a very limited scope, please submit an issue to suggest new modules or get help.

Join the discussion in our [Telegram Group](https://t.me/inkathon).

## Macros

| Macros         | Description                                                                                                                                                                                                                                                                                                                     | Audited |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------- |
| `Ownable2Step` | This module provides a two-step ownership transfer mechanism. It includes methods for getting the current and pending admin, transferring ownership, and accepting ownership. The ownership transfer is not immediate but requires the new owner to accept the ownership. This adds an extra layer of security to the contract. | ✅      |
| `Upgradable`   | This module provides a mechanism for upgrading the contract. It includes a method for setting a new code hash, effectively upgrading the contract to a new version. This is useful for contracts that may need to be updated or fixed after they have been deployed.                                                            | ✅      |
