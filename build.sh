rm -rf target/
rm -rf *.so
rm -rf *.pyd

# Rebuild the project
uv run maturin develop --uv
