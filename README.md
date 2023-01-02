# Image and video file organization project
Project to organize photos and videos by separating them into folders by month and year.

### Used Crates
- chrono
- clap

## Building instructions
- `cargo build --release`

## Instructions for use
- `<binary_bath> --src <source_path> --dst <destination_path>`
- example: ./target/release/fior --src ./all --dst ./photos

**Obs.:** If you omit any arguments, it will be considered the default path '.'
