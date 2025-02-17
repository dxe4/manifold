import toml


def get_version_from_cargo():
    try:
        cargo_data = toml.load("Cargo.toml")
        return cargo_data["package"]["version"]
    except Exception as e:
        print(f"Error reading Cargo.toml: {e}")
        return None

def increment_patch_version(version):
    major, minor, patch = map(int, version.split("."))
    patch += 1
    return f"{major}.{minor}.{patch}"


def update_cargo_version(new_version):
    try:
        cargo_data = toml.load("Cargo.toml")
        cargo_data["package"]["version"] = new_version
        with open("Cargo.toml", "w") as file:
            toml.dump(cargo_data, file)
        print(f"Updated version in Cargo.toml to {new_version}")
    except Exception as e:
        print(f"Error updating Cargo.toml: {e}")


def update_pyproject_version(new_version):
    try:
        pyproject_data = toml.load("pyproject.toml")
        pyproject_data["project"]["version"] = new_version
        with open("pyproject.toml", "w") as file:
            toml.dump(pyproject_data, file)
        print(f"Updated version in pyproject.toml to {new_version}")
    except Exception as e:
        print(f"Error updating pyproject.toml: {e}")


def main():
    cargo_version = get_version_from_cargo()
    if cargo_version:
        incremented_version = increment_patch_version(cargo_version)
        update_cargo_version(incremented_version)
        update_pyproject_version(incremented_version)


if __name__ == "__main__":
    main()
