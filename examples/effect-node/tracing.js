import { pipe } from "@effect/data/Function"
import * as Effect from "@effect/io/Effect"
import * as Layer from "@effect/io/Layer"
import * as NodeSdk from "@effect/opentelemetry/NodeSdk"
import * as Resource from "@effect/opentelemetry/Resource"
import * as Tracer from "@effect/opentelemetry/Tracer"
// import { ConsoleSpanExporter } from "@opentelemetry/sdk-trace-base"
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http'

const ResourceLive = Resource.layer({ serviceName: "effect-node" })

const NodeSdkLive = NodeSdk.layer(Effect.sync(() =>
  NodeSdk.config({
    // traceExporter: new ConsoleSpanExporter()
    traceExporter: new OTLPTraceExporter()
  })
))

const TracingLive = Layer.provide(
  ResourceLive,
  Layer.merge(NodeSdkLive, Tracer.layer)
)

const program = pipe(
  Effect.log("Hello"),
  Effect.withSpan("c"),
  Effect.withSpan("b"),
  Effect.withSpan("a")
)

pipe(
  program,
  Effect.provideLayer(TracingLive),
  Effect.catchAllCause(Effect.logErrorCause),
  Effect.runFork
)