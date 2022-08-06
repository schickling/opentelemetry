# 2022-opentelemetry [![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-ready--to--code-908a85?logo=gitpod)](https://gitpod.io/#https://github.com/schickling-test/2022-opentelemetry)

- OTLP endpoints via HTTP `4318` and gRPC `4317`

## Spin up infra

```sh
cd .infra
docker compose up -d
```

## Examples

- See [`opentelemetry-js` repo](https://github.com/open-telemetry/opentelemetry-js) for more JS/TS examples

## Notes

- You can use the `OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"` env var to configure the collector endpoint (see [here for details](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md)).

## TODO

- Make Grafana/Tempo search work
- Add effect example
