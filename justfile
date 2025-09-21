upload:
    cross build --release --target=aarch64-unknown-linux-gnu
    scp ./target/aarch64-unknown-linux-gnu/release/aquarium-light-controller tobinio@192.168.0.130:/home/tobinio/Desktop/light-controller
