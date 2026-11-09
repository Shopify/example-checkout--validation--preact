# Test the migrated checkout validation example

End-to-end testing for this example, following the [checkout validation tutorial](https://shopify.dev/docs/apps/build/checkout/cart-checkout-validation/create-admin-ui-validation).

Use only an app and development store you own or are authorized to modify. These steps install a dev preview and create a checkout rule; they aren't read-only. No production deployment is needed.

## 1. Prerequisites and checkout

Use Node.js 22+, current Shopify CLI, Rust with rustup, and a dev store with a published product and at least 10 available units of a variant. Use a normal online-store checkout without unrelated rules interfering.

If you haven't already checked out the PR and installed dependencies:

```bash
cd example-checkout--validation--preact
pnpm install --frozen-lockfile
pnpm test
```

Don't overwrite existing local changes or switch branches if doing so would discard work. This walkthrough doesn't require scaffolding additional extensions.

## 2. Select the correct Rust toolchain

If your PATH selects Homebrew Rust while rustup installs targets under `~/.rustup`, `rustup target add` can report success while the default compiler still can't find Wasm `core` or `std`.

Use rustup's stable compiler for this terminal session:

```bash
rustup target add --toolchain stable wasm32-unknown-unknown
export PATH="$(dirname "$(rustup which --toolchain stable rustc)"):$PATH"
hash -r

which cargo rustc
rustc --print sysroot
```

Both tools and the sysroot should point into `~/.rustup/toolchains/stable-…`, not Homebrew. Keep this PATH for the later Shopify CLI build too. It doesn't modify your shell configuration.

## 3. Build with the committed release settings

```bash
cd extensions/product-limits-function
cargo build --target=wasm32-unknown-unknown --release
test -s target/wasm32-unknown-unknown/release/cart-checkout-validation.wasm
cd ../..
```

Expected: successful compilation and a nonempty artifact at the path above, matching `shopify.extension.toml`.

If `core` or `std` is missing, recheck step 2. If macOS kills host build scripts, record the error separately. A diagnostic build disabling strip/LTO isn't a pass for the unchanged release settings. Don't change the Wasm target back to `wasm32-wasip1` for this migrated example.

## 4. Link only your disposable test app

If `shopify.app.validation-test.toml` already points to your intended test app, reuse it and skip linking. Don't use the sample's checked-in app identity or overwrite another app's config.

Otherwise, obtain your test app's Client ID from the Developer Dashboard and run:

```bash
shopify app config link \
  --client-id YOUR_TEST_APP_CLIENT_ID \
  --file-name shopify.app.validation-test.toml
```

Don't pass `--force` to overwrite an existing configuration. If using the interactive picker and none of its organizations is appropriate, cancel rather than selecting a shared organization/app just to continue.

Ensure the test configuration includes:

```toml
[access_scopes]
scopes = "read_products"
```

If it already has other required scopes, retain them and add `read_products` to the comma-separated list. Don't copy credentials into this walkthrough or commit personal app configuration.

## 5. Start the dev preview

From the app root, in the same terminal with the Rust PATH from step 2:

```bash
shopify app dev \
  --config validation-test \
  --store YOUR-DEV-STORE.myshopify.com
```

Follow the prompts to install/update the test app and approve its access. Keep the process running. Verify that the Function and settings UI build successfully. Don't run `shopify app deploy` for this test.

## 6. Configure the rule

In your dev store's Shopify admin:

1. Go to **Settings → Checkout → Checkout rules → Add rule**.
2. Select the sample product-limits validation. Identify it by the app/extension shown; its displayed name can differ from the tutorial's scaffold name.
3. Enter a recognizable test title.
4. In its settings UI, set one displayed variant's limit to **5**.
5. Leave **Status** set to **Inactive**, and then **Save**.
6. Reopen the rule and confirm the limit persisted. For a rule created with the older flat settings format, reopen and save it once with the updated UI before testing; this migrates the saved configuration to the Function's wrapped integer format.

The sample UI fetches the first eight products and up to four variants per product. Choose a variant it displays, and use that exact variant in the cart. If the UI fails to save/reload or the Function logs show a configuration parsing error, record that as a failed test rather than manually bypassing the UI.

## 7. Test checkout

Use a fresh cart containing the selected variant. Make sure inventory or other checkout requirements aren't what blocks progress.

| Rule/configuration | Cart quantity | Expected |
| --- | --- | --- |
| Inactive; limit 5 | 6 | No product-limit error from this rule. |
| Activate and save; limit 5 | 6 | Error naming limit 5 and the product; checkout progression blocked. |
| Active; limit 5 | 5 | No product-limit error. |
| Active; limit 5 | 4 | No product-limit error. |
| Change limit to 3 and save | 4 | Error now names limit 3, proving updated settings reach the Function. |

For an over-limit cart, expect `Orders are limited to a maximum of 5 of [product]` (or 3 after the configuration update). Confirm that checkout can't progress, not merely that an error is visible. For passing carts, confirm the validation error clears and progress is possible once ordinary checkout prerequisites are satisfied. You don't need to place an order or make a real payment.

Leave **Block checkout if app experiences a problem** unchanged. These scenarios test returned validation errors, not runtime failures; returned errors should block regardless of that setting.

## 8. Inspect logs, record results, and clean up

In the Developer Dashboard, open your test app's **Logs** and filter to **Function**. Confirm that the tested runs complete without traps, input-deserialization errors, or invalid output.

Record:

```bash
git rev-parse HEAD
cargo --version
rustc --version
shopify version
```

Save the normal release-build outcome, the five checkout outcomes, evidence that settings persist, and relevant screenshots/logs. Redact credentials and customer information.

Deactivate the test rule and stop `shopify app dev`. If you created temporary products or other test data, remove them only when they're no longer needed.

## Pass criteria

- The committed release configuration builds for `wasm32-unknown-unknown`.
- The dev preview loads both extensions.
- Settings save and reload through the UI.
- The active rule blocks above the configured limit, but not at or below it.
- Updating the saved limit changes the observed behavior.
- Function execution logs show no runtime/input/output errors.

A successful Cargo build alone doesn't establish the tutorial's end-to-end behavior. This file describes checks to perform; it isn't a claim that they have all passed.
