FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres

RUN apt update
RUN apt install postgresql-client -y


VOLUME ["/usr/local/cargo"]
