# PortWatch

## Summary

This Rust application syncs the Gluetun forwarded port with a running qBittorrent instance and resets the qBittorrent listening port if it becomes inaccessible. It supports multiple authentication types for the Gluetun API.

## Description

Every 5 minutes, this application queries the Gluetun control server API for the forwarded port. If the forwarded port has changed since the last query, it updates the qBittorrent listening port through its API. Additionally, it attempts to establish a TLS connection to the forwarded port and resets the port in qBittorrent if a connection cannot be established.

## Docker Compose Example

```yaml
services:
  portwatch:
    image: tsousa28/portwatch
    container_name: portwatch
    environment:
      - QBITTORRENT_USERNAME=${QBITTORRENT_USERNAME}
      - QBITTORRENT_PASSWORD=${QBITTORRENT_PASSWORD}
    restart: unless-stopped
    depends_on:
      - qbittorrent
    networks:
      - proxy
```

## Docker Compose Environment Variables

| Variable                | Description                                                         | Default        |
|-------------------------|---------------------------------------------------------------------|----------------|
| `QBITTORRENT_USERNAME`   | qBittorrent WebUI username                                         | -              |
| `QBITTORRENT_PASSWORD`   | qBittorrent WebUI password                                         | -              |
| `QBITTORRENT_PORT`       | qBittorrent instance port                                          | `8080`         |
| `GLUETUN_HOST`           | Gluetun IP address                                                 | `gluetun`      |
| `GLUETUN_PORT`           | Gluetun control server port                                        | `8000`         |
| `GLUETUN_AUTH_TYPE`      | Gluetun instance authentication type (`noauth`, `basic`, `apikey`) | `noauth`       |
| `GLUETUN_API_USERNAME`   | Gluetun basic auth username                                        | -              |
| `GLUETUN_API_PASSWORD`   | Gluetun basic auth password                                        | -              |
| `GLUETUN_API_KEY`        | Gluetun API key                                                    | -              |

## License

This project is licensed under the GNU General Public License (GPL) - see the [LICENSE](LICENSE) file for details.
