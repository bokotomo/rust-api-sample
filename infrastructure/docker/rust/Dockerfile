FROM rust:1.41-stretch

ENV USER=bokotomo
WORKDIR /rust-actix-web
RUN apt-get update
RUN apt install -y vim mysql-client python3-pip python3-dev default-libmysqlclient-dev
RUN cargo install diesel_cli --no-default-features --features "mysql"
RUN pip3 install pytest requests mysql-connector-python mysqlclient orator