#------------------------------------------------------------------------------------------------------------
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
#-------------------------------------------------------------------------------------------------------------


FROM ubuntu:20.04
ARG DEBIAN_FRONTEND=noninteractive
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH
RUN apt update &&  apt -y upgrade
RUN apt install -y tzdata 
RUN apt install -y build-essential curl wget git vim libboost-all-dev
RUN curl -sL https://deb.nodesource.com/setup_14.x |  bash
RUN apt install -y nodejs \
RUN npm install -y -g ssvmup --unsafe-perm && npm install -y ssvm
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Switch back to dialog for any ad-hoc use of apt
ENV DEBIAN_FRONTEND=dialog
