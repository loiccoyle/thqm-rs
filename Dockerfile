# Build Stage
FROM rust AS build-stage

ADD . /usr/src/thqm
WORKDIR /usr/src/myapp

RUN cargo build --release

# Final Stage
FROM scratch

ARG GIT_COMMIT
ARG VERSION
LABEL REPO="https://github.com/loiccoyle/thqm-rs"
LABEL GIT_COMMIT=$GIT_COMMIT
LABEL VERSION=$VERSION

WORKDIR /usr/local/bin

COPY --from=build-stage /usr/src/thqm/bin/thqm /opt/thqm/bin/
RUN chmod +x /usr/local/bin/thqm

CMD /usr/local/bin/thqm
