# Breez-SDK Liquid

## Configuration
### Running tests
Currently, running tests requires to have a working docker environment. To run them, you can execute the following commands:
```bash
# Build and run the container
docker build . -t breez-sdk-liquid
docker run --name breez-sdk-liquid --tty -d breez-sdk-liquid:latest

# Run the tests
docker exec -it breez-sdk-liquid sh -c "RUST_LOG=debug cargo test" -s -- --nocapture
```

### Environment Variables
Take a look at [.env.example](.env.example) for all the available environment variables, and place them in your `.env` file, they will be automatically sourced.
**Note:** You can override any environment variables specified in the .env file temporarily, by prefixing the `cargo test` command with the desired key-value pair. This is especially useful to avoid entering the container directly.
