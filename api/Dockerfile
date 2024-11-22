FROM rust:latest as builder

# build app
WORKDIR /app
COPY . /app
RUN cargo build --release

# Use a minimal base image for the final container
FROM gcr.io/distroless/cc-debian12

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/api /

# Expose the port that the Actix-web server will listen on
EXPOSE 8080

# Set the command to run the Actix-web server
CMD ["./api"]