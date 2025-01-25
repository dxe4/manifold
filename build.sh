rm -rf target/
rm -rf *.so
rm -rf *.pyd

# Rebuild the project
cargo clean
uv run maturin develop --uv
