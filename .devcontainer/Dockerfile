# Setup the development container image

ARG VARIANT="bookworm"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

# https://github.com/docker-library/buildpack-deps/blob/65d69325ad741cea6dee20781c1faaab2e003d87/debian/bullseye/curl/Dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends \
		ca-certificates \
		curl \
		netbase \
		wget \
	&& rm -rf /var/lib/apt/lists/*

RUN set -ex; \
	if ! command -v gpg > /dev/null; then \
		apt-get update; \
		apt-get install -y --no-install-recommends \
			gnupg \
			dirmngr \
		; \
		rm -rf /var/lib/apt/lists/*; \
	fi

# https://github.com/docker-library/buildpack-deps/blob/65d69325ad741cea6dee20781c1faaab2e003d87/debian/bullseye/Dockerfile
# procps is very common in build systems, and is a reasonably small package
RUN apt-get update && apt-get install -y --no-install-recommends \
		git \
		\
		procps \
	&& rm -rf /var/lib/apt/lists/*

RUN set -ex; \
	apt-get update; \
	apt-get install -y --no-install-recommends \
		autoconf \
		automake \
		bzip2 \
		dpkg-dev \
		file \
		g++ \
		gcc \
		imagemagick \
		libbz2-dev \
		libc6-dev \
		libcurl4-openssl-dev \
		libdb-dev \
		libevent-dev \
		libffi-dev \
		libgdbm-dev \
		libglib2.0-dev \
		libgmp-dev \
		libjpeg-dev \
		libkrb5-dev \
		liblzma-dev \
		libmagickcore-dev \
		libmagickwand-dev \
		libmaxminddb-dev \
		libncurses5-dev \
		libncursesw5-dev \
		libpng-dev \
		libpq-dev \
		libreadline-dev \
		libsqlite3-dev \
		libssl-dev \
		libtool \
		libwebp-dev \
		libxml2-dev \
		libxslt-dev \
		libyaml-dev \
		make \
		patch \
		unzip \
		xz-utils \
		zlib1g-dev \
		\
# https://lists.debian.org/debian-devel-announce/2016/09/msg00000.html
		$( \
# if we use just "apt-cache show" here, it returns zero because "Can't select versions from package 'libmysqlclient-dev' as it is purely virtual", hence the pipe to grep
			if apt-cache show 'default-libmysqlclient-dev' 2>/dev/null | grep -q '^Version:'; then \
				echo 'default-libmysqlclient-dev'; \
			else \
				echo 'libmysqlclient-dev'; \
			fi \
		) \
	; \
	rm -rf /var/lib/apt/lists/*

# Install additional OS packages.
# Add Wasi-sdk build libs: https://github.com/WebAssembly/wasi-sdk
RUN apt-get update && export ACCEPT_EULA=y DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
	# unixodbc-dev \
	openssl \
	apt-transport-https \
	build-essential \
	libssl-dev \
	cmake \
	clang \
	ninja-build \
	gnupg2 \
	sudo

# Replace shell with bash so we can source files
RUN rm /bin/sh && ln -s /bin/bash /bin/sh

# Install NVM via vendored shell script
COPY .devcontainer/nvm_install.sh /bin/nvm_install.sh

# Enable vscode user to make changes to bin
RUN chown -R vscode /bin

# Begin using vscode instead of root
USER vscode

# Install and configure nvm / node

# Setup nvm environment variables
# ENV NODE_VERSION 19.3.0
ENV NODE_VERSION 22.6.0

ENV NVM_DIR="/bin/.nvm"
RUN sudo mkdir -p $NVM_DIR
RUN sudo chown -R vscode $NVM_DIR

# Install nvm from vendored script instead of:
# RUN curl --silent -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
RUN cat /bin/nvm_install.sh | bash

RUN source $NVM_DIR/nvm.sh \
	&& source $NVM_DIR/bash_completion \
    && nvm install $NODE_VERSION \
    && nvm alias default $NODE_VERSION \
    && nvm use default

# Add node and npm to path so the commands are available
ENV NODE_PATH $NVM_DIR/v$NODE_VERSION/lib/node_modules
ENV PATH $NVM_DIR/versions/node/v$NODE_VERSION/bin:$PATH

RUN npm install -g wrangler

# Setup nightly, wasm32-wasi and general components
RUN rustup update && \
	rustup default nightly && \
	rustup component add rustfmt rust-src clippy-preview && \
	rustup component add llvm-tools-preview --toolchain nightly-aarch64-unknown-linux-gnu

# Pre-fetch wrangler utilities
RUN cargo install worker-build
RUN cargo install cargo-llvm-cov
RUN cargo install just
