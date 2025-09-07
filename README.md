# Aerugo

[![CI/CD](https://github.com/your-org/aerugo/actions/workflows/main.yml/badge.svg)](https://github.com/your-org/aerugo/actions/workflows/main.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Aerugo** is a next-generation, distributed, and multi-tenant container registry built with Rust. It is designed for high performance, security, and scalability, leveraging an S3-compatible object storage backend for infinite scalability of container images.

## ✨ Core Features

* **Distributed & Highly Available:** Designed from the ground up to run in a clustered environment with no single point of failure.
* **Multi-tenancy:** First-class support for individual users and organizations, allowing for the creation and management of private registries with granular access control.
* **S3-Compatible Backend:** Uses any S3-compatible object storage (AWS S3, MinIO, Ceph, etc.) for storing container image layers, ensuring durability and scalability.
* **Written in Rust:** Provides memory safety, concurrency, and performance, making it a secure and efficient core for your registry infrastructure.
* **Docker Registry V2 API Compliant:** Fully compatible with the Docker client and other OCI-compliant tools.
* **Modern Management API:** A separate, clean RESTful API for programmatic management of users, organizations, repositories, and permissions.

---

## 🏛️ Architecture

Aerugo operates on a shared-nothing, stateless node architecture. This allows for effortless horizontal scaling by simply adding more nodes behind a load balancer. The state is managed externally in a dedicated metadata store and the S3 backend.

### High-Level Diagram

```ascii
        +---------------------------------+
        |   Docker Client / Admin Client  |
        +----------------+----------------+
                         |
           +-------------+-------------+
           | HTTPS (Registry & Mgmt API) |
           v                             v
+---------------------------------------------------+
|                  Load Balancer                    |
+---------------------------------------------------+
           |              |              |
           v              v              v
    +--------------+ +--------------+ +--------------+
    | Aerugo Node  | | Aerugo Node  | | Aerugo Node  |
    |   (Rust)     | |   (Rust)     | |   (Rust)     |  <-- Stateless, Scalable Service
    +------+-------+ +------+-------+ +------+-------+
           |              |              |
           |       +------+------+       |
           |       |             |       |
           v       v             v       v
+---------------------+     +---------------------+
|   Metadata Store    |<--->|    Cache Layer      |
| (e.g., PostgreSQL,  |     |   (e.g., Redis)     |
|     CockroachDB)    |     +---------------------+
+---------------------+
           ^
           | (Manages users, orgs, permissions, manifests, tags)
           |
           +---------------------------------------------+
                                                         |
                                                         v (Generates presigned URLs for blobs)
                                               +-------------------------+
                                               |      S3-Compatible      |
                                               |      Object Storage     |
                                               +-------------------------+
                                                         ^
                                                         |
                                                         | (Direct Blob Upload/Download)
                                                         +-------------------> Docker Client
```


Component Breakdown
Load Balancer: The entry point for all traffic. It distributes requests across the available Aerugo nodes. It should handle TLS termination.

Aerugo Nodes: These are the stateless, core application instances written in Rust.

They handle all API logic for both the Docker Registry V2 API and the Management API.

They authenticate and authorize requests by querying the Metadata Store.

For blob operations (pushes/pulls), they do not proxy the data. Instead, they generate pre-signed URLs for the client to interact directly with the S3-compatible backend. This drastically reduces the load on the registry nodes and improves performance.

Metadata Store: A transactional, persistent database (e.g., PostgreSQL, CockroachDB) that stores all non-blob data:

User and Organization accounts.

Repository information and permissions.

Image manifests and tags.

Authentication tokens and API keys.

S3-Compatible Object Storage: This is the storage layer for the actual content of the container images (the layers, or "blobs"). By offloading this to an S3-compatible service, Aerugo can scale its storage capacity independently and benefit from the durability features of these systems.

Cache Layer: A distributed cache (e.g., Redis) is used to cache frequently accessed metadata, such as manifest data and authorization decisions, to reduce latency and load on the Metadata Store.

⚙️ API Overview
Aerugo exposes two primary APIs on the same port, routed by the application based on the request path.

1. Registry API (/v2/)
Implements the Docker Registry V2 API specification.

Handles docker pull, docker push, and other OCI-related commands.

Authentication is typically done via Bearer tokens.

2. Management API (/api/v1/)
A RESTful API for administrative and user-level management tasks. All responses are in JSON.

Key Endpoints (Conceptual):

Authentication:

POST /api/v1/auth/token: Exchange credentials for a JWT.

Users:

POST /api/v1/users: Create a new user.

GET /api/v1/users/{username}: Get user details.

Organizations:

POST /api/v1/orgs: Create a new organization.

GET /api/v1/orgs/{org_name}: Get organization details.

POST /api/v1/orgs/{org_name}/members: Add a user to an organization.

Repositories:

GET /api/v1/repos/{namespace}/{repo_name}: Get repository details and tags.

DELETE /api/v1/repos/{namespace}/{repo_name}: Delete a repository.

PUT /api/v1/repos/{namespace}/{repo_name}/permissions: Set user/team permissions for a repository.

🚀 Getting Started
(This section will be updated once the initial codebase is available.)

Prerequisites
Rust toolchain (latest stable)

A running PostgreSQL instance

An S3-compatible object storage endpoint

Building
Bash

git clone [https://github.com/your-org/aerugo.git](https://github.com/your-org/aerugo.git)
cd aerugo
cargo build --release
Configuration
Configuration is managed via a config.toml file or environment variables.

Ini, TOML

# Example config.toml
[server]
listen_address = "0.0.0.0:8080"

[database]
url = "postgres://user:password@localhost/aerugo"

[storage]
type = "s3"
bucket = "aerugo-registry-bucket"
region = "us-east-1"
endpoint = "[https://s3.amazonaws.com](https://s3.amazonaws.com)"
# access_key and secret_key should be set via env vars
🗺️ Roadmap
[x] Core architecture design

[ ] Implementation of Docker V2 API for pull operations (blobs & manifests)

[ ] Implementation of Docker V2 API for push operations

[ ] Development of the core Management API for users and organizations

[ ] JWT-based authentication and authorization layer

[ ] Implement repository-level access controls

[ ] Online garbage collection for untagged images

[ ] Webhook notifications for image pushes

[ ] Helm chart for easy Kubernetes deployment

🤝 Contributing
Contributions are welcome! Please open an issue to discuss your ideas or submit a pull request.

📜 License
This project is licensed under the Apache 2.0 License. See the LICENSE file for details.
