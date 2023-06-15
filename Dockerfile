FROM rust:1.69-bullseye
COPY ./src src
COPY ./Cargo.toml Cargo.toml
RUN curl https://github.com/WiggidyW/eve_item_configurator_sqlite_accessor_rs/raw/master/db.sqlite -o db.sqlite -L
RUN mv db.sqlite /root/db.sqlite
RUN cargo build --release
RUN mv target/release/wetc_item_configurator_server /root/service

FROM envoyproxy/envoy:v1.26-latest
COPY ./envoy.yaml envoy.yaml
COPY ./run.sh run.sh
COPY --from=0 /root/service service
COPY --from=0 /root/db.sqlite db.sqlite
RUN chmod +x service
RUN chmod +x run.sh
ENV SERVICE_ADDRESS=0.0.0.0:8080
CMD ["./run.sh"]
