# DMARC Report Viewer

[![Build Status](https://github.com/cry-inc/dmarc-report-viewer/workflows/CI/badge.svg)](https://github.com/cry-inc/dmarc-report-viewer/actions)
[![No Unsafe](https://img.shields.io/badge/unsafe-forbidden-brightgreen.svg)](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A lightweight selfhosted standalone DMARC report viewer that automatically fetches input data periodically from an IMAP mailbox.

Ideal for smaller selfhosted mailservers.
The application is a single executable written in Rust.
It combines the DMARC report parser with an IMAP client and an HTTP server for easy access of the reports.
You can run the executable directly on any Linux, Windows or MacOS system.
Alternatively, you can use the tiny Linux Docker image to deploy the application.

## Run with Docker
The latest version is always published automatically as Docker image in the GitHub container registry.
You can download the image using the command `sudo docker pull ghcr.io/cry-inc/dmarc-report-viewer`.

List all available configuration parameters with the corresponding environment variables by running this command:
`sudo docker run --rm ghcr.io/cry-inc/dmarc-report-viewer ./dmarc-report-viewer --help`.

You can configure the application with command line arguments or environment variables.
For the Docker use case environment variables are recommended.
Do not forget to forward the port for the HTTP server!
Since the application has no persistent data, no volumes are required.
Here is an complete example: 

    sudo docker run --rm \
      -e IMAP_HOST=mymailserver.com \
      -e IMAP_USER=dmarc@mymailserver.com \
      -e IMAP_PASSWORD=mysecurepassword \
      -e HTTP_SERVER_PORT=8123 \
      -e HTTP_SERVER_USER=webui-user \
      -e HTTP_SERVER_PASSWORD=webui-password \
      -p 8123:8123 \
      ghcr.io/cry-inc/dmarc-report-viewer

## Build from Source
1. Install Rust (see https://rustup.rs/)
2. Check out this repository or download and extract the ZIP
3. Run the command `cargo build --release` in the folder with this README file
4. Find the compiled executable in the folder `target/release`
5. Use the help argument to list all possible configuration parameters: `dmarc-report-viewer --help`

## Acknowledgements
- https://github.com/bbustin/dmarc_aggregate_parser was used as foundation for the slightly modified DMARC report parser
- https://github.com/chartjs/Chart.js is embedded as JavaScript library for generating nice charts