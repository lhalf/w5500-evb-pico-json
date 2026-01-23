# w5500-evb-pico-json

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/lhalf/w5500-evb-pico-json/on_commit.yml)](https://github.com/lhalf/w5500-evb-pico-json/actions/workflows/on_commit.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue)](./LICENSE)

Protocol break relay for valid JSON on the W5500-EVB-Pico.

[![W5500-EVB-Pico](https://wiznet.io/files/attach/product/W5500-EVB-Pico.png)](https://wiznet.io/products/evaluation-boards/w5500-evb-pico)

## Overview

Uses the [WIZnet W5500](https://wiznet.io/products/ethernet-chips/w5500) on the [W5500-EVB-Pico](https://wiznet.io/products/evaluation-boards/w5500-evb-pico)
in [MACRAW](https://docs.wiznet.io/pdf-viewer?file=%2Fassets%2Ffiles%2FW5500_ds_v110e-226ffec190c588b69f88d629789585e1.pdf) mode to pass raw packets to the [RP2040](https://en.wikipedia.org/wiki/RP2040). A 
[protocol break](https://www.ncsc.gov.uk/collection/cross-domain-solutions/using-the-principles/network-protocol-attack-protection) is implemented by
throwing away the protocol information of received packets, then the payload is validated as JSON, and finally a new 
UDP packet is constructed to forward the contents on.

## Dependencies

You will need a debug probe that supports the [Serial Wire Debug](https://en.wikipedia.org/wiki/JTAG#Similar_interface_standards) (SWD) protocol.




