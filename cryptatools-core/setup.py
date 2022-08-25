from setuptools import setup, Distribution, find_packages
from setuptools.command.install import install

from distutils.command.build import build as _build
import os
import re
import shutil
import subprocess
import sys

import wheel.bdist_wheel

sys.dont_write_bytecode = True


if sys.version_info < (3, 6):
    print("glean requires at least Python 3.6", file=sys.stderr)
    sys.exit(1)

from pathlib import Path  # noqa

# Path to the directory containing this file
PYTHON_ROOT = Path(__file__).parent.absolute()

# Relative path to this directory from cwd.
FROM_TOP = PYTHON_ROOT.relative_to(Path.cwd())

# Path to the root of the git checkout
SRC_ROOT = PYTHON_ROOT.parents[1]

# glean version. Automatically updated by the bin/prepare_release.sh script
version = "51.1.0"

requirements = [
    "semver>=2.13.0",
    "glean_parser~=6.1",
    "wheel",
    "semantic-version==2.9.0",
    "setuptools",
    "typing_extensions==4.0.1",
    "pytest==7.1.2",
    "tox==3.25.1",
]

# The environment variable `GLEAN_BUILD_VARIANT` can be set to `debug` or `release`
buildvariant = os.environ.get("GLEAN_BUILD_VARIANT", "debug")


class BinaryDistribution(Distribution):
    def is_pure(self):
        return False

    def has_ext_modules(self):
        return True


def macos_compat(target):
    if target.startswith("aarch64-"):
        return "11.0"
    return "10.7"


# The logic for specifying wheel tags in setuptools/wheel is very complex, hard
# to override, and is really meant for extensions that are compiled against
# libpython.so, not this case where we have a fairly portable Rust-compiled
# binary that should work across a number of Python versions. Therefore, we
# just skip all of its logic be overriding the `get_tag` method with something
# simple that only handles the cases we need.
class bdist_wheel(wheel.bdist_wheel.bdist_wheel):
    def get_tag(self):
        cpu, _, __ = target.partition("-")
        impl, abi_tag = "cp36", "abi3"
        if "-linux" in target:
            plat_name = f"linux_{cpu}"
        elif "-darwin" in target:
            compat = macos_compat(target).replace(".", "_")
            if cpu == "aarch64":
                cpu = "arm64"
            plat_name = f"macosx_{compat}_{cpu}"
        elif "-windows" in target:
            impl, abi_tag = "py3", "none"
            if cpu == "i686":
                plat_name = "win32"
            elif cpu == "x86_64":
                plat_name = "win_amd64"
            else:
                raise ValueError("Unsupported Windows platform")
        else:
            # Keep local wheel build on BSD/etc. working
            _, __, plat_name = super().get_tag()

        return (impl, abi_tag, plat_name)


class InstallPlatlib(install):
    def finalize_options(self):
        install.finalize_options(self)
        if self.distribution.has_ext_modules():
            self.install_lib = self.install_platlib


def get_rustc_info():
    """
    Get the rustc info from `rustc --version --verbose`, parsed into a
    dictionary.
    """
    regex = re.compile(r"(?P<key>[^:]+)(: *(?P<value>\S+))")

    output = subprocess.check_output(["rustc", "--version", "--verbose"])

    data = {}
    for line in output.decode("utf-8").splitlines():
        match = regex.match(line)
        if match:
            d = match.groupdict()
            data[d["key"]] = d["value"]

    return data


target = os.environ.get("GLEAN_BUILD_TARGET")
if not target:
    target = get_rustc_info()["host"]


if "-darwin" in target:
    shared_object = "libglean_ffi.dylib"
elif "-windows" in target:
    shared_object = "cryptatools_core.dll"
else:
    # Anything else must be an ELF platform - Linux, *BSD, Solaris/illumos
    shared_object = "libglean_ffi.so"


class build(_build):
    def run(self):
        try:
            # Use `check_output` to suppress output
            subprocess.check_output(["cargo"])
        except subprocess.CalledProcessError:
            print("Install Rust and Cargo through Rustup: https://rustup.rs/.")
            print(
                "Need help installing the glean_sdk? https://github.com/mozilla/glean/#contact"
            )
            sys.exit(1)

        env = os.environ.copy()

        # For `musl`-based targets (e.g. Alpine Linux), we need to set a flag
        # to produce a shared object Python extension.
        if "-musl" in target:
            env["RUSTFLAGS"] = (
                env.get("RUSTFLAGS", "") + " -C target-feature=-crt-static"
            )
        if target == "i686-pc-windows-gnu":
            env["RUSTFLAGS"] = env.get("RUSTFLAGS", "") + " -C panic=abort"

        command = [
            "cargo",
            "build",
            "--package",
            "cryptatools-core",
            "--target",
            target,
        ]

        if buildvariant != "debug":
            command.append(f"--{buildvariant}")

        if "-darwin" in target:
            env["MACOSX_DEPLOYMENT_TARGET"] = macos_compat(target)

        subprocess.check_call(command, cwd=SRC_ROOT / "cryptatools-rs", env=env)

        shutil.copyfile(
            SRC_ROOT / "cryptatools-rs" / "target" / target / buildvariant / "deps" / shared_object,
            SRC_ROOT / "cryptatools-rs" / "cryptatools-core" / "uniffi_caesar_number.dll",
        )

        command = [
            "uniffi-bindgen",
            "generate",
            "cryptatools-rs/cryptatools-core/src/caesar_number.udl",
            "--language",
            "python",
            "--out-dir",
            SRC_ROOT / "target",
        ]
        subprocess.check_call(command, cwd=SRC_ROOT, env=env)

        shutil.copyfile(
            SRC_ROOT / "target" / "caesar_number.py", SRC_ROOT / "cryptatools-rs" / "cryptatools-core" / "caesar_number.py"
        )

        return _build.run(self)


setup(
    author="gogo2464",
    author_email="gogo246475@gmail.com",
    classifiers=[
        "Intended Audience :: Developers",
        "Natural Language :: English",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
    ],
    description="Python Binding of the library and cryptanalysis tool 'cryptatools'.",
    install_requires=requirements,
    long_description_content_type="text/markdown",
    include_package_data=True,
    keywords="cryptatools",
    name="cryptatools-python3",
    version=version,
    packages=[
         "cryptatools_core",
    ],
    package_dir={
         "cryptatools_core": "cryptatools-core"
    },
    setup_requires=requirements,
    url="https://github.com/gogo2464/cryptatools",
    zip_safe=False,
    package_data={"cryptatools_core": [shared_object, "uniffi_caesar_number.dll"]},
    distclass=BinaryDistribution,
    cmdclass={"install": InstallPlatlib, "bdist_wheel": bdist_wheel, "build": build},
)