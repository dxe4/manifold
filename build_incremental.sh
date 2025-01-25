rm -rf *.so
rm -rf *.pyd

uv run maturin develop --uv
