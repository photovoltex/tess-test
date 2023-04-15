FROM rust:slim

ARG USER_ID=1000
ARG GROUP_ID=1000

# update image and install required packages
RUN apt update &&\
    apt install -y pkg-config libleptonica-dev libtesseract-dev clang

# good explaination why we have to do this and how it works
#  https://jtreminio.com/blog/running-docker-containers-as-current-host-user/
# create and change user
# so that no mismatch between the current user and the container user are created
RUN groupadd -g ${GROUP_ID} rustacean &&\
    useradd -l -u ${USER_ID} -g rustacean rustacean
USER rustacean

# install additional rust sugar
RUN rustup component add clippy &&\
    cargo install bacon
