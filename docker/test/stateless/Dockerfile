# rebuild in #33610
# docker build -t clickhouse/stateless-test .
ARG FROM_TAG=latest
FROM clickhouse/test-base:$FROM_TAG

ARG odbc_driver_url="https://github.com/ClickHouse/clickhouse-odbc/releases/download/v1.1.6.20200320/clickhouse-odbc-1.1.6-Linux.tar.gz"

# golang version 1.13 on Ubuntu 20 is enough for tests
RUN apt-get update -y \
    && env DEBIAN_FRONTEND=noninteractive \
        apt-get install --yes --no-install-recommends \
            awscli \
            brotli \
            lz4 \
            expect \
            golang \
            lsof \
            mysql-client=8.0* \
            ncdu \
            netcat-openbsd \
            nodejs \
            npm \
            odbcinst \
            openjdk-11-jre-headless \
            openssl \
            postgresql-client \
            python3 \
            python3-pip \
            qemu-user-static \
            sqlite3 \
            sudo \
            tree \
            unixodbc \
            rustc \
            cargo \
            zstd \
            file \
            jq \
            pv \
            zip \
            unzip \
            p7zip-full \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /var/cache/debconf /tmp/*

ARG PROTOC_VERSION=25.1
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v${PROTOC_VERSION}/protoc-${PROTOC_VERSION}-linux-x86_64.zip \
    && unzip protoc-${PROTOC_VERSION}-linux-x86_64.zip -d /usr/local \
    && rm protoc-${PROTOC_VERSION}-linux-x86_64.zip

COPY requirements.txt /
RUN pip3 install --no-cache-dir -r /requirements.txt

RUN mkdir -p /tmp/clickhouse-odbc-tmp \
  && cd /tmp/clickhouse-odbc-tmp \
  && curl -L ${odbc_driver_url} | tar --strip-components=1 -xz clickhouse-odbc-1.1.6-Linux \
  && mkdir /usr/local/lib64 -p \
  && cp /tmp/clickhouse-odbc-tmp/lib64/*.so /usr/local/lib64/ \
  && odbcinst -i -d -f /tmp/clickhouse-odbc-tmp/share/doc/clickhouse-odbc/config/odbcinst.ini.sample \
  && odbcinst -i -s -l -f /tmp/clickhouse-odbc-tmp/share/doc/clickhouse-odbc/config/odbc.ini.sample \
  && sed -i 's"=libclickhouseodbc"=/usr/local/lib64/libclickhouseodbc"' /etc/odbcinst.ini \
  && rm -rf /tmp/clickhouse-odbc-tmp

ENV TZ=Europe/Amsterdam
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

ENV NUM_TRIES=1

# Unrelated to vars in setup_minio.sh, but should be the same there
# to have the same binaries for local running scenario
ARG MINIO_SERVER_VERSION=2024-08-03T04-33-23Z
ARG MINIO_CLIENT_VERSION=2024-07-31T15-58-33Z
ARG TARGETARCH

# Download Minio-related binaries
RUN arch=${TARGETARCH:-amd64} \
    && curl -L "https://dl.min.io/server/minio/release/linux-${arch}/archive/minio.RELEASE.${MINIO_SERVER_VERSION}" -o ./minio \
    && curl -L "https://dl.min.io/client/mc/release/linux-${arch}/archive/mc.RELEASE.${MINIO_CLIENT_VERSION}" -o ./mc \
    && chmod +x ./mc ./minio

ENV MINIO_ROOT_USER="clickhouse"
ENV MINIO_ROOT_PASSWORD="clickhouse"
ENV EXPORT_S3_STORAGE_POLICIES=1

RUN npm install -g azurite@3.33.0 \
    && npm install -g tslib && npm install -g node

ENV PREFIX=/opt/openssl-fips
ENV OPENSSL_VERSION=3.1.2

RUN apt-get update && apt-get install -y \
    build-essential \
    wget \
    perl \
    python3 \
    rng-tools \
    make \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp
RUN wget https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz && \
    tar xzf openssl-${OPENSSL_VERSION}.tar.gz && \
    cd openssl-${OPENSSL_VERSION} && \
    ./Configure enable-fips --prefix=${PREFIX} && \
    make -j$(nproc) && \
    make install_sw && \
    make install_ssldirs && \
    make install_fips && \
    mv "${PREFIX}/ssl/openssl.cnf.dist" "${PREFIX}/ssl/openssl.cnf"

RUN echo ".include ${PREFIX}/ssl/fipsmodule.cnf" >> ${PREFIX}/ssl/openssl.cnf
RUN cat >> ${PREFIX}/ssl/openssl.cnf <<EOF
[openssl_init]
providers = provider_sect
alg_section = algorithm_sect
ssl_conf = ssl_sect

[ssl_sect]
system_default = system_default_sect

[system_default_sect]
CipherString = DEFAULT:@SECLEVEL=2

[provider_sect]
default = default_sect
fips = fips_sect
base = base_sect

[base_sect]
activate = 1

[default_sect]
activate = 1

[algorithm_sect]
default_properties = fips=yes
EOF

ENV PATH="${PREFIX}/bin:$PATH"
ENV OPENSSL_CONF=${PREFIX}/ssl/openssl.cnf
ENV OPENSSL_MODULES="${PREFIX}/lib64/ossl-modules"
WORKDIR /
