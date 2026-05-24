# Worker Server

Simple backend worker servers for the load-balancer project.

The project allows running multiple lightweight worker instances that can be used by the load balancer for request distribution, health checks, and load balancing experiments.

## Features

- Lightweight HTTP worker servers
- Supports running multiple workers simultaneously
- Health check endpoints
- Designed for integration with the load-balancer project
- Useful for testing load balancing algorithms and adaptive routing

## Related Project

This worker server is designed to work together with:

- https://github.com/cj-zhukov/load-balancer

## How to Run

Run multiple worker servers by providing:
- Worker name
- Bind address

Example:

```bash
cargo run -- \
  "super1,0.0.0.0:3001" \
  "super2,0.0.0.0:3002" \
  "super3,0.0.0.0:3003"
```

## Example

After startup:

- `super1` → `127.0.0.0:3001`
- `super2` → `127.0.0.0:3002`
- `super3` → `127.0.0.0:3003`

These workers can then be registered in the load balancer and used for request routing.
