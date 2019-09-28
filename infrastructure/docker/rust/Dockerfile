FROM rust:1.38-stretch
 
ENV USER=bokotomo
WORKDIR /rust-actix-web
RUN apt-get update
RUN apt install -y vim mysql-client
RUN cargo install diesel_cli --no-default-features --features "mysql"