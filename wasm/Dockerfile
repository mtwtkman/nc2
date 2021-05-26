FROM debian:stable-slim

RUN apt update
RUN apt install -y python3 python3-dev curl
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt install -y nodejs

# workaround
RUN chmod 777 /usr/local/lib
RUN chmod 777 /usr/local/bin
RUN chmod 777 /usr/lib/node_modules
RUN chmod 777 /usr/bin

RUN adduser app
USER app
WORKDIR /home/app
RUN mkdir nc2
WORKDIR nc2
RUN npm install -g parcel@next parcel-plugin-wasm-pack