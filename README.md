# opentelemetry [![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-ready--to--code-908a85?logo=gitpod)](https://gitpod.io/#https://github.com/schickling/opentelemetry)

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

### Spanmetrics on Prometheus

```
traces_spanmetrics_calls_total
traces_spanmetrics_latency_bucket
traces_spanmetrics_latency_count
traces_spanmetrics_latency_sum
```

### Snippet to clone `.infra` folder to a local project

```sh
mkdir .infra
cd .infra
curl https://codeload.github.com/schickling/opentelemetry/tar.gz/main | \
  tar -xz --strip=2 opentelemetry-main/.infra
```

## TODO

- [ ] Go from metrics to traces
- [ ] Go from traces to metrics
- [ ] Add grafana dashboard to project for some Otel metrics (see [docs](https://grafana.com/docs/grafana/latest/administration/provisioning/#dashboards))
- [ ] Add effect example
