# DDNS Updater for Google Domains

A simple and ephemeral Rust program to update a Google Domains Dynamic DNS record.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Features](#features)
- [Contributing](#contributing)
- [License](#license)

## Installation

This project uses Docker and Devcontainer to provide a consistent development environment. To get started, install the following dependencies:

- [Docker](https://docs.docker.com/get-docker/)
- [Visual Studio Code](https://code.visualstudio.com/)
- [Visual Studio Code Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### Requirements

- Domain name registered with Google Domains
- Dynamic DNS record configured for the domain
- Google Domains Dynamic DNS username and password

## Usage

To get started, follow these steps:

1. Open the project in Visual Studio Code.
2. Select the "Reopen in Container" option to build the Docker image and start a container with the project's development environment.
3. Provide your Google Domains Dynamic DNS username, password, and host when running the Docker container:

```bash
docker run -e DDNS_USER=<your-ddns-user> -e DDNS_PASS=<your-ddns-password> -e DDNS_HOST=<your-ddns-host> ghcr.io/cjaviersaldana/ddns-agent-domainsgoogle
```

## Features

Update a Google Domains Dynamic DNS record. This can be done by running the Docker container or using the image in a Kubernetes CronJob.

### How to build

```bash
docker build -t ddns-agent-domainsgoogle .
```

## Contributing

Feel free to submit a pull request or an issue. Contributions are welcome.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
