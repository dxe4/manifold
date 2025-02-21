set -e

read -p "Do you want to bump the version? (Y/n): " RESPONSE
RESPONSE=${RESPONSE:-Y}

if [[ "$RESPONSE" =~ ^[Yy]$ ]]; then
    echo "Bumping version..."
    uv run python bump_rust_version.py
fi

VERSION=$(grep '^version =' Cargo.toml | head -1 | cut -d '"' -f2)
TAG="v$VERSION"

echo "Releasing version: v$VERSION"

if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "Tag $TAG already exists, skipping tagging."
else
    echo "Creating new Git tag: $TAG"
    git tag -a "$TAG" -m "Release version $VERSION"
    git push origin "$TAG"
fi

rm target/wheels/*.whl
echo "Building the wheel..."
uv run maturin build --release

echo "Creating GitHub release..."
gh release create "v$VERSION" target/wheels/*.whl --title "Version $VERSION" --notes "Automated release of version $VERSION."

echo "Release v$VERSION completed successfully!"
