import { defineConfig } from "@junobuild/config";

/** @type {import('@junobuild/config').JunoConfig} */
export default defineConfig({
  satellite: {
    ids: {
      development: "uxrrr-q7777-77774-qaaaq-cai",
      production: "<PROD_SATELLITE_ID>",
    },
    source: "dist",
    predeploy: ["npm run build"],
  },
  emulator: {
    runner: {
      type: "docker",
      name: "workshop-eth-address"
    },
    skylab: {}
  }
});
