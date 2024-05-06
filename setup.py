from setuptools import find_packages, setup

with open("README.md", "r") as f:
    long_description = f.read()

setup(
    name="shakmaty_python_binding",
    version="0.0.10",
    description="a binding to python from shakmaty",
    package_dir={"": "src"},
    packages=find_packages(where="app"),
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/victorgabillon/shakmaty_python_binding",
    author="victorgabillon",
    author_email="victorgabillon@gmail.com",
    license="GPL3",
    extras_require={
        "dev": ["pytest>=7.0", "twine>=4.0.2"],
    },
    python_requires=">=3.11",
)