# A Fedora 24 BDCS API Container
FROM weld/fedora:24
MAINTAINER Brian C. Lane <bcl@redhat.com>

RUN curl https://sh.rustup.rs -sSf \
  | sh -s -- -y --default-toolchain nightly-2017-02-04
ENV PATH="/root/.cargo/bin:${PATH}"

COPY entrypoint.sh /usr/local/bin/entrypoint.sh
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
EXPOSE 4000

# Volumes for database and recipe storage.
VOLUME /mddb /bdcs-recipes /mockfiles

## Do the things more likely to change below here. ##
## Run rustup update to pick up the latest nightly ##
COPY . /bdcs-api-rs/
RUN cd /bdcs-api-rs/ && rustup update && cargo build && cargo doc
