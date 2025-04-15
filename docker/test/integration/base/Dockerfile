# rebuild in #33610
# docker build -t clickhouse/integration-test .
ARG FROM_TAG=latest
FROM clickhouse/test-base:$FROM_TAG

SHELL ["/bin/bash", "-c"]

RUN apt-get update \
    && env DEBIAN_FRONTEND=noninteractive apt-get -y install \
        bsdutils \
        curl \
        default-jre \
        g++ \
        iproute2 \
        krb5-user \
        libicu-dev \
        libsqlite3-dev \
        libsqliteodbc \
        lsof \
        lz4 \
        odbc-postgresql \
        odbcinst \
        python3 \
        rpm2cpio \
        sqlite3 \
        tar \
        tzdata \
        unixodbc \
        python3-pip \
        libcurl4-openssl-dev \
        libssl-dev \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /var/cache/debconf /tmp/*

COPY requirements.txt /
RUN pip3 install --no-cache-dir -r requirements.txt && rm -rf /root/.cache/pip

# Architecture of the image when BuildKit/buildx is used
ARG TARGETARCH

# Install MySQL ODBC driver from RHEL rpm
# For reference https://downloads.mysql.com/archives/c-odbc/ RHEL
RUN arch=${TARGETARCH:-amd64} \
  && case $arch in \
      amd64) rarch=x86_64 ;; \
      arm64) rarch=aarch64 ;; \
    esac \
  && cd /tmp \
  && curl -o mysql-odbc.rpm "https://cdn.mysql.com/archives/mysql-connector-odbc-8.0/mysql-connector-odbc-8.0.32-1.el9.${rarch}.rpm" \
  && rpm2archive mysql-odbc.rpm \
  && tar xf mysql-odbc.rpm.tgz -C / ./usr/lib64/ \
  && rm mysql-odbc.rpm mysql-odbc.rpm.tgz \
  && ODBC_DIR=$(dpkg -L odbc-postgresql | rg '^/usr/lib/.*-linux-gnu/odbc$') \
  && ln -s /usr/lib64/libmyodbc8a.so "$ODBC_DIR" \
  && ln -s /usr/lib64/libmyodbc8a.so "$ODBC_DIR"/libmyodbc.so

# Unfortunately this is required for a single test for conversion data from zookeeper to clickhouse-keeper.
# ZooKeeper is not started by default, but consumes some space in containers.
# 777 perms used to allow anybody to start/stop ZooKeeper
ENV ZOOKEEPER_VERSION='3.6.3'
RUN curl "https://archive.apache.org/dist/zookeeper/zookeeper-${ZOOKEEPER_VERSION}/apache-zookeeper-${ZOOKEEPER_VERSION}-bin.tar.gz" | \
      tar -C opt -zxv && \
  mv /opt/apache-zookeeper-${ZOOKEEPER_VERSION}-bin /opt/zookeeper && \
  chmod -R 777 /opt/zookeeper && \
  echo $'tickTime=2500 \n\
tickTime=2500 \n\
dataDir=/zookeeper \n\
clientPort=2181 \n\
maxClientCnxns=80' > /opt/zookeeper/conf/zoo.cfg && \
  mkdir /zookeeper && \
  chmod -R 777 /zookeeper

ENV TZ=Etc/UTC
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

COPY --from=clickhouse/cctools:13cfebfd8f1bbc65ab49 /opt/gdb /opt/gdb
ENV PATH="/opt/gdb/bin:${PATH}"
ENV PREFIX=/opt/openssl-fips
ENV OPENSSL_VERSION=3.1.2

RUN apt-get update && apt-get install -y \
    build-essential \
    wget \
    perl \
    python3 \
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

[provider_sect]
fips = fips_sect
base = base_sect

[base_sect]
activate = 1

[default_sect]
activate = 0

[algorithm_sect]
default_properties = fips=yes
EOF

ENV PATH="${PREFIX}/bin:$PATH"
ENV OPENSSL_CONF=${PREFIX}/ssl/openssl.cnf
ENV OPENSSL_MODULES="${PREFIX}/lib64/ossl-modules"
WORKDIR /
