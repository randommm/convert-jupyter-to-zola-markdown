This program will convert Jupyter `ipynb` to Zola markdown `.md` files.

Moreover, it will create a link at the end of the Markdown file to "Download this page as a Jupyter Notebook".

It does that by scaning the `contents` folder and creating an `.md` file for every `.ipynb` it finds there.

E.g.: if it finds `/contents/getting-start/first-step.ipynb`, it will create a file at `/contents/getting-start/first-step.md`, note that if the file `/contents/getting-start/first-step.md` alredy exists, IT WILL BE OVERWRITTEN.

ALWAYS MAKE A BACKUP OF YOUR FILES BEFORE RUNNING THIS PROGRAM!!!

## Installation

* Install Jupyter if you don't have it yet.

* Install Rust if you don't have it yet, check here for instructions: `https://rustup.rs/`. On Ubuntu `sudo apt-get install cargo`. Another option is using Conda: `conda install -c conda-forge rust`.

* Run `cargo install --git https://github.com/randommm/convert-jupyter-to-zola-markdown`

## Usage

* Go to the root of your Zola project.

* Run `convert-jupyter-to-zola-markdown`
