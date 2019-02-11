import sys

from setuptools import setup
from setuptools.command.test import test as TestCommand

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension


class PyTest(TestCommand):
    user_options = []

    def run(self):
        self.run_command("test_rust")

        import subprocess

        subprocess.check_call(["pytest", "tests"])


setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []
tests_require = install_requires + ["pytest", "pytest-benchmark"]

setup(
    name="py_disc_model",
    version="0.1.0",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Intended Audience :: Science/Research",
        "License :: CC0 1.0 Universal (CC0 1.0) Public Domain Dedication",
        "Natural Language :: English",
        "Operating System :: OS Independent",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Topic :: Games/Entertainment :: Simulation",
        "Topic :: Scientific/Engineering :: Physics",
        "Topic :: Scientific/Engineering",
        "Topic :: Software Development :: Libraries",
    ],
    packages=["py_disc_model"],
    rust_extensions=[RustExtension("py_disc_model.py_disc_model", "Cargo.toml")],
    install_requires=install_requires,
    tests_require=tests_require,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
    cmdclass=dict(test=PyTest),
)
