FROM rustlang/rust:nightly-slim

RUN cargo install siredir 

ENTRYPOINT ["siredir"]
