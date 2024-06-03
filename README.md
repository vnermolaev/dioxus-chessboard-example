## Overview

This binary demonstrates functionality of a Dioxus component
`dioxus-chessboard`([crates.io](https://crates.io/crates/dioxus-chessboard)).

## Setup

- Install Dioxus CLI

  ```bash
  cargo install dioxus-cli
  ```

- Clone the test project
  ```bash
  git clone git@github.com:vnermolaev/dioxus-chessboard-example.git
  ```   

- Switch to the test project and execute
  ``` bash
  cd ./dioxus-chessboard-test
  npx tailwindcss -i ./input.css -o ./public/tailwind.css
  dx serve --hot-reload
  ```
  This will compile the styles and open a browser tab with the example page.