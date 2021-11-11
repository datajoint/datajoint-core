FROM xd009642/tarpaulin:develop-nightly

COPY . .

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build