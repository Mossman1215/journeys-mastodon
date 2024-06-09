FROM alpine:latest

COPY ./target/release/journeys-mastodon /bin/

ENTRYPOINT [ "/bin/journeys-mastodon" ]

LABEL org.opencontainers.image.source="https://github.com/Mossman1215/journeys-mastodon"
LABEL org.opencontainers.image.ref.name="ghcr.io/mossman1215/journeys-mastodon:latest"
LABEL org.opencontainers.image.title="Journeys Mastodon"
LABEL org.opencontainers.image.description="A fedi bot for NZ wide traffic updates"