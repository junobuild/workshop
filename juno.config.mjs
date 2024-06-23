import { defineConfig } from "@junobuild/config";

/** @type {import('@junobuild/config').JunoConfig} */
export default defineConfig({
  satellite: {
    // TODO: STEP_1_CONFIGURATION
    id: "replace-satellite-id",
    source: "dist",
  },
});
