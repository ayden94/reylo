// Vendored from @standard-schema/spec (MIT). The spec is designed to be copied so
// that implementers carry no dependency; only the members lando implements are kept.
export interface StandardTypedV1<Input = unknown, Output = Input> {
  readonly "~standard": StandardTypedV1.Props<Input, Output>;
}
export declare namespace StandardTypedV1 {
  interface Props<Input = unknown, Output = Input> {
    readonly version: 1;
    readonly vendor: string;
    readonly types?: Types<Input, Output> | undefined;
  }
  interface Types<Input = unknown, Output = Input> {
    readonly input: Input;
    readonly output: Output;
  }
}
export interface StandardJSONSchemaV1<Input = unknown, Output = Input> {
  readonly "~standard": StandardJSONSchemaV1.Props<Input, Output>;
}
export declare namespace StandardJSONSchemaV1 {
  interface Props<Input = unknown, Output = Input> extends StandardTypedV1.Props<Input, Output> {
    readonly jsonSchema: Converter;
  }
  interface Converter {
    readonly input: (options: Options) => Record<string, unknown>;
    readonly output: (options: Options) => Record<string, unknown>;
  }
  type Target = "draft-2020-12" | "draft-07" | "openapi-3.0" | ({} & string);
  interface Options {
    readonly target: Target;
    readonly libraryOptions?: Record<string, unknown> | undefined;
  }
}
