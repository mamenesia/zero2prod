FROM lukemathwalker/cargo-chef:latest-rust-1-slim-bookworm as chef
# Create a new stage for our application
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

# Copy the recipe from the planner stage    
COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies, not our application
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
ENV SQLX_OFFLINE true

# Build our application
RUN cargo build --release --bin zero2prod \
    && strip target/release/zero2prod


# Runtime stage
FROM debian:bookworm-slim AS runtime
# FROM gcr.io/distroless/cc-debian12 AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/* \
    # Create nonroot user and group
    && groupadd -r nonroot \
    && useradd -r -g nonroot nonroot \
    && chown -R nonroot:nonroot /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/zero2prod zero2prod

# Copy the configuration file from the builder stage
COPY configuration configuration

ENV APP_ENVIRONMENT production

# Use non-root user from distroless image
USER nonroot:nonroot


ENTRYPOINT [ "./zero2prod"]
