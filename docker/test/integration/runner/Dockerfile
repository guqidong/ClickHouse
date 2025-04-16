# docker build -t clickhouse/integration-tests-runner .
ARG FROM_TAG=latest
FROM clickhouse/test-util:$FROM_TAG

# ARG for quick switch to a given ubuntu mirror
ARG apt_archive="http://archive.ubuntu.com"

RUN sed -i "s|http://archive.ubuntu.com|$apt_archive|g" /etc/apt/sources.list

RUN apt-get update \
    && env DEBIAN_FRONTEND=noninteractive apt-get install --yes \
    adduser \
    ca-certificates \
    bash \
    btrfs-progs \
    e2fsprogs \
    iptables \
    xfsprogs \
    tar \
    pigz \
    wget \
    git \
    iproute2 \
    cgroupfs-mount \
    python3-pip \
    tzdata \
    libicu-dev \
    bsdutils \
    curl \
    openssh-client \
    liblua5.1-dev \
    luajit \
    libssl-dev \
    libcurl4-openssl-dev \
    default-jdk \
    software-properties-common \
    libkrb5-dev \
    krb5-user \
    g++ \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* /var/cache/debconf /tmp/*

ENV TZ=Etc/UTC
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

ENV DOCKER_CHANNEL stable
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add - \
    && add-apt-repository "deb https://download.docker.com/linux/ubuntu $(lsb_release -c -s) ${DOCKER_CHANNEL}" \
    && apt-get update \
    && env DEBIAN_FRONTEND=noninteractive apt-get install --yes \
        docker-ce="5:27.0.3*" \
        docker-compose-plugin='2.29.*' \
    && rm -rf \
        /var/lib/apt/lists/* \
        /var/cache/debconf \
        /tmp/* \
    && apt-get clean \
    && dockerd --version; docker --version


# kazoo 2.10.0 is broken
# https://s3.amazonaws.com/clickhouse-test-reports/59337/524625a1d2f4cc608a3f1059e3df2c30f353a649/integration_tests__asan__analyzer__[5_6].html
COPY requirements.txt /
RUN python3 -m pip install --no-cache-dir -r requirements.txt

# Hudi supports only spark 3.3.*, not 3.4
RUN curl -fsSL -O https://archive.apache.org/dist/spark/spark-3.3.2/spark-3.3.2-bin-hadoop3.tgz \
    && tar xzvf spark-3.3.2-bin-hadoop3.tgz -C / \
    && rm spark-3.3.2-bin-hadoop3.tgz

# download spark and packages
# if you change packages, don't forget to update them in tests/integration/helpers/cluster.py
RUN packages="org.apache.hudi:hudi-spark3.3-bundle_2.12:0.13.0,\
io.delta:delta-core_2.12:2.3.0,\
org.apache.iceberg:iceberg-spark-runtime-3.3_2.12:1.1.0" \
    && /spark-3.3.2-bin-hadoop3/bin/spark-shell --packages "$packages" > /dev/null \
    && find /root/.ivy2/ -name '*.jar' -exec ln -sf {} /spark-3.3.2-bin-hadoop3/jars/ \;

RUN set -x \
  && addgroup --system dockremap \
  && adduser --system dockremap \
  && adduser dockremap dockremap \
  && echo 'dockremap:165536:65536' >> /etc/subuid \
  && echo 'dockremap:165536:65536' >> /etc/subgid

COPY modprobe.sh /usr/local/bin/modprobe
COPY dockerd-entrypoint.sh /usr/local/bin/
COPY misc/ /misc/

COPY --from=clickhouse/cctools:13cfebfd8f1bbc65ab49 /opt/gdb /opt/gdb
ENV PATH="/opt/gdb/bin:${PATH}"

# Same options as in test/base/Dockerfile
# (in case you need to override them in tests)
ENV TSAN_OPTIONS='halt_on_error=1 abort_on_error=1 history_size=7 memory_limit_mb=46080 second_deadlock_stack=1'
ENV UBSAN_OPTIONS='print_stacktrace=1'
ENV MSAN_OPTIONS='abort_on_error=1 poison_in_dtor=1'
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

EXPOSE 2375
ENTRYPOINT ["dockerd-entrypoint.sh"]
# To pass additional arguments (i.e. list of tests) use PYTEST_ADDOPTS
CMD ["sh", "-c", "pytest"]
