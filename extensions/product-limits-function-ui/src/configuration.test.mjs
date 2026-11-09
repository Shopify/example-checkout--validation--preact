import assert from "node:assert/strict";
import test from "node:test";
import { createConfiguration, readLimits } from "./configuration.mjs";

test("legacy settings migrate to wrapped integer limits", () => {
  const legacy = { "variant-a": "0", "variant-b": "2" };
  assert.deepEqual(createConfiguration(readLimits(legacy)), {
    limits: { "variant-a": 0, "variant-b": 2 },
  });
});

test("wrapped settings reload and serialize without changing values", () => {
  const configuration = { limits: { "variant-a": 0, "variant-b": 2147483647 } };
  assert.deepEqual(createConfiguration(readLimits(configuration)), configuration);
});

test("empty fields remove limits", () => {
  assert.deepEqual(createConfiguration({ a: "", b: null, c: undefined }), { limits: {} });
});

test("invalid limits cannot be saved", () => {
  for (const value of ["-1", "1.5", "2abc", " ", "1e2", -1, 1.5, 2147483648, NaN, Infinity, true, {}]) {
    assert.throws(() => createConfiguration({ variant: value }));
  }
});
