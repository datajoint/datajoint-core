FROM xd009642/tarpaulin:develop-nightly

COPY . .

ENV PATH="/root/.cargo/bin:${PATH}"

# we don't need to build here since 
# the code will be built once we run the tests
# RUN cargo build