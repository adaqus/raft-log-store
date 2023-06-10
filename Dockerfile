FROM debian:bullseye-slim

ARG USER_ID=1000
ARG GROUP_ID=1000
ARG APP_SOURCE="<PATH_TO_COMPILED_BINARY>"

ENV RUST_BACKTRACE=1

WORKDIR /var/app

RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y --no-install-recommends ca-certificates libssl-dev openssl libmariadb3 \
    && apt-get clean \
    && groupadd --gid $GROUP_ID app \
    && useradd --uid $USER_ID --gid $GROUP_ID --shell /bin/bash app

USER $USER_ID:$GROUP_ID

COPY --chown=$USER_ID:$GROUP_ID ${APP_SOURCE} /var/app/raft-log-store

CMD ["/var/app/raft-log-store"]

EXPOSE 8080
