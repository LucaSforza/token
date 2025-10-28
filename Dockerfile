FROM rust 
WORKDIR /app
COPY . .
RUN apt-get update && apt-get upgrade -y 
RUN curl -L https://foundry.paradigm.xyz | bash
RUN cargo build