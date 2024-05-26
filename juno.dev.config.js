import { defineDevConfig } from "@junobuild/config";

/** @type {import('@junobuild/config').JunoDevConfig} */
export default defineDevConfig(() => ({
  satellite: {
    collections: {
      db: [
        {
          collection: "notes",
          read: "managed",
          write: "managed",
          memory: "storage",
          mutablePermissions: false,
        },
        {
          collection: "profiles",
          read: "managed",
          write: "managed",
          memory: "storage",
          mutablePermissions: false,
        },
      ],
      storage: [
        {
          collection: "images",
          read: "managed",
          write: "managed",
          memory: "storage",
          mutablePermissions: false,
        },
      ],
    },
    controllers: [],
  },
}));
