// The global `Temporal` namespace for the types a mapper with `"dates": "temporal"` emits
// (`Temporal.Instant`, `Temporal.PlainDateTime`, `Temporal.PlainDate`). TypeScript's libs do
// not declare Temporal yet; `temporal-spec` carries the proposal's declarations.
import "temporal-spec/global";
export {};
