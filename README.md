# Web3 Diagram

## **This is a cargo utility that allows developers to generate visualizations depicting data flows within their NEAR Rust smart contracts.**

<br>

### Which problem does it solve?
**Provides the ability to generate representations of methods to allow for anyone to quickly understand how the contract works.**

<br>

### Who can use this?
**Smart contract developers use it to generate the images, but the images can help anyone trying to understand what’s going on.**

<br>

### Key Features
**Method specific file hierarchy diagrams**<br>

<br>

## Install cli with [brew](https://brew.sh/)
___
`brew install mermaid-cli`

<br>

## Install [node](https://gist.github.com/tomysmile/da3cb8194ec8f0e4df86#install-node)
___
`brew install node`

<br>

## Web3 Utility Usage
___

### 1) Clone raffle smart contract from near examples

`git clone git@github.com:near-examples/rust-ft-raffle.git`

<br>

### 2) Go to the root directory
`cd rust-ft-raffle` 

### 3) Run utility
`web3d -O --input raffle.md`

<br>

| Key/Command | Description |
| ----------- | ----------- |
| -b, --backgroundColor | Background color. Example: transparent, red, '#F0F0F0'. Optional. Default: white |
| -h, --help | Print help information |
| -H, --height | Height of the page. Optional. Default: 600 |
| -i, --input  | Markdown file name |
| -o, --output | Output file name. It should be either md, svg, png or pdf. Optional. Default:'./res/input_file_name.svg' |
| -O, --openb | Should open output file in browser |
| -q, --quiet | Suppress log output |
| -w, --width | Width of the page. Optional. Default: 800 |