# 2022-opentelemetry

- OTLP endpoints via HTTP `4318` and gRPC `4317`

## Spin up infra

```sh
cd .infra
docker compose up -d
```

## Notes

- You can use the `OTEL_EXPORTER_OTLP_ENDPOINT="http://localhost:4317"` env var to configure the collector endpoint (see [here for details](https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/protocol/exporter.md)).

## TODO

- Make Grafana/Tempo search work
- Add effect example
