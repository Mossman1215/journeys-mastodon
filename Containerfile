FROM alpine:latest

COPY ./Data/NZ_Regions_LocalGovt.geojson /Data/NZ_Regions_LocalGovt.geojson
COPY ./target/x86_64-unknown-linux-musl/release/journeys-mastodon /bin/journeys-mastodon

ENTRYPOINT [ "/bin/journeys-mastodon" ]

LABEL org.opencontainers.image.source="https://github.com/Mossman1215/journeys-mastodon"
LABEL org.opencontainers.image.ref.name="ghcr.io/mossman1215/journeys-mastodon:latest"
LABEL org.opencontainers.image.title="Journeys Mastodon"
LABEL org.opencontainers.image.description="A fedi bot for NZ wide traffic updates"