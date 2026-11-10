// Read legacy flat settings as well as the Function's wrapped configuration.
export function readLimits(configuration) {
  return configuration.limits ?? configuration;
}

export function createConfiguration(limits) {
  const normalized = {};
  for (const [variantId, value] of Object.entries(limits)) {
    // An empty field removes a limit; zero remains a valid limit.
    if (value === "" || value === null || value === undefined) continue;
    if (
      (typeof value !== "string" && typeof value !== "number") ||
      (typeof value === "string" && !/^\d+$/.test(value))
    ) {
      throw new Error("Limits must be whole numbers between 0 and 2147483647.");
    }
    const limit = Number(value);
    if (!Number.isInteger(limit) || limit < 0 || limit > 2147483647) {
      throw new Error("Limits must be whole numbers between 0 and 2147483647.");
    }
    normalized[variantId] = limit;
  }
  return { limits: normalized };
}
