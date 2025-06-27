# Juno: Workshop

![A screenshot of the example developed during the workshop](https://raw.githubusercontent.com/junobuild/create-juno/main/screenshots/screenshot-example.png)

This repository provides code samples and instructions to guide attendees in discovering Juno during a workshop.

## Getting Started

Clone the repository and install the dependencies:

```bash
git clone https://github.com/junobuild/workshop
cd workshop
npm ci
```

## Workshop

We are developing a note-taking app, and the core functionality is already in place. However, we still need to integrate Juno, which we plan to implement during the workshop.

By following the steps below and replacing the provided snippet, we will be able to implement the app and learn about building on Juno.

---

### Frontend Integration Steps

1. [Prerequisites](#1-prerequisites)
2. [Initialization](#2-initialization)
3. [Authentication](#3-authentication)
4. [Storing Document](#4-storing-documents)
5. [Listing Document](#5-listing-documents)
6. [Uploading Files](#6-uploading-files)

---

### 1. Prerequisites

Before initializing the project and integrating Juno, make sure you have the following installed:

- **[Docker](https://www.docker.com/)** – Required to run the Juno Emulator, which mimics the production environment locally.
- **Juno CLI** – Install it globally with:

```bash
npm install -g @junobuild/cli
```

### 2. Initialization

Before we can integrate Juno into the app, we’ll need to create a satellite and configure our project.

#### a. Start the local development emulator

This will spin up the Juno Emulator:

```bash
juno dev start
```

#### b. Create a Satellite

Your project needs a Satellite. Create one to connect your app for development.

👉 [Open the Juno Console](http://localhost:5866)

#### c. Configure your project

Set the Satellite ID in your `juno.config.mjs` file:

```ts
import { defineConfig } from "@junobuild/config";

export default defineConfig({
  satellite: {
    ids: {
      development: "<DEV_SATELLITE_ID>",
    },
    source: "dist",
    predeploy: ["npm run build"],
  },
});
```

#### d. Start the frontend dev server

In another terminal, start your app's dev server:

```bash
npm run dev
```

#### e. Create a Datastore collection

This template is a note-taking app, so it needs a `notes` collection. Create it in the Datastore.

👉 [Go to Datastore](http://localhost:5866/datastore)

#### f. Create a Storage collection

Likewise, it needs a collection named `images` to save assets. Create it in the Storage.

👉 [Go to Storage](http://localhost:5866/storage)

#### g. Start using the JavaScript library

Initialize the Satellite for your app. The configuration variables are automatically injected via the plugins.

> TODO: find and replace STEP_INITIALIZATION

```javascript
await initSatellite();
```

---

### 3. Authentication

To get to know the user’s state, Juno provides an observable function called `authSubscribe()`. We can use it as many times as required, but I find it convenient to subscribe to it at the top of an app.

> TODO: find and replace STEP_AUTH_SUBSCRIBE

```typescript
import { authSubscribe, type User } from "@junobuild/core";

const sub = authSubscribe((user: User | null) => console.log(user));
```

To securely identify users anonymously, they will need to sign in.

> TODO: find and replace STEP_AUTH_SIGN_IN

```javascript
import { signIn } from "@junobuild/core";

await signIn();
```

> [!NOTE]  
> Signing out works the same way.

---

### 4. Storing Documents

Storing data on the blockchain with Juno is done through a feature called “Datastore”. Follow the instructions in the documentation to create a collection, which can be named accordingly (“notes”).

Once our collection is created, we can persist data on the blockchain using the `setDoc` function.

> TODO: find and replace STEP_SET_DOC

```javascript
await setDoc({
  collection: "notes",
  doc: {
    key,
    data: {
      text: inputText,
    },
  },
});
```

---

### 5. Listing Documents

To fetch the list of documents saved on the blockchain, we can use the `listDocs` function.

> TODO: find and replace STEP_LIST_DOCS

```javascript
const { items } = await listDocs({
  collection: "notes",
});
```

---

### 6. Uploading Files

As for the documents, to upload assets we will need first to create a collection in the “Storage”. We can be name it “images”.

Once our collection is set, we can upload a file on chain using the `uploadFile` function.

> TODO: find and replace STEP_UPLOAD_FILE

```javascript
const { downloadUrl } = await uploadFile({
  collection: "images",
  data: file,
  filename,
});
```

In this particular workshop, we also want to save a reference within the document to its related asset.

> TODO: find and replace STEP_ADD_REFERENCE

```javascript
await setDoc({
  collection: "notes",
  doc: {
    key,
    data: {
      text: inputText,
      ...(url !== undefined && { url }), // <--- We add this reference
    },
  },
});
```

---

### Serverless Functions Steps

1. [Initialization](#1-init)
2. [Build](#2-build)
3. [Assertion](#3-assertion)
4. [Hook](#4-hook)

---

#### 1. Init

In a new terminal, run the command to scaffold the functions and select `JavaScript`.

```bash
juno functions eject
```

#### 2. Build

To ensure everything works out, let's add some log and build the functions.

Search for `assertSetDoc` and replace the default snippet with following:

```javascript
export const assertSetDoc = defineAssert({
  collections: ["notes"],
  assert: (context) => {
    console.log("Hello");
  }
});
```

Then build the functions:

```bash
juno functions build
```

The local emulator should detect the change and automatically upgrade (re-deploy) the WASM container.

Once applied, every time you record a note, a console log should appear in the emulator output.

#### 3. Assertion

Instead of a log, we can implement a custom assertion that should reject every document that contains the text "Hello".

```javascript
export const assertSetDoc = defineAssert({
  collections: ["notes"],
  assert: (context) => {
    const data = decodeDocData(context.data.data.proposed.data);

    if (data.text.toLowerCase().includes("hello")) {
      throw new Error("The text must not include the word 'hello'");
    }
  }
});
```

#### 4. Hook

Hooks allow you to implement custom backend logic — a kind of post-processing that runs when data is created, updated, or deleted.

Search for `onSetDoc` and replace the default snippet with this function that updates the content of the notes that is saved.

```javascript
export const onSetDoc = defineHook({
  collections: ["notes"],
  run: async (context) => {
    // Decode the document's data (stored as a blob)
    const data = decodeDocData(context.data.data.after.data);

    // Update the document's data by enhancing the "hello" field
    const updated = {
      text: `${data.text} checked ✅`
    };

    // Encode the data back to blob format
    const encoded = encodeDocData(updated);

    // Save the updated document using the same caller, collection, and key
    await setDocStore({
      caller: context.caller,
      collection: context.data.collection,
      key: context.data.key,
      doc: {
        data: encoded,
        description: context.data.data.after.description,
        version: context.data.data.after.version
      }
    });
  }
});
```

---

## Production

Ready to go live?

Just like for local development, you'll need to create a Satellite — but this time on the mainnet [Console](https://console.juno.build). Then, update your `juno.config.mjs` with the new Satellite ID:

```ts
import { defineConfig } from "@junobuild/config";

export default defineConfig({
  satellite: {
    ids: {
      development: "<DEV_SATELLITE_ID>",
      production: "<PROD_SATELLITE_ID>",
    },
    source: "dist",
    predeploy: ["npm run build"],
  },
});
```

Check out the full guides in the [docs](https://juno.build/docs/category/deployment).

## ✨ Links & Resources

- Looking to get started with Juno? Check out the [documentation](https://juno.build).
- Have a look at [React](https://react.dev) for question regarding the templates.
- Got questions, comments or feedback? [Join our discord](https://discord.gg/wHZ57Z2RAG) or [OpenChat](https://oc.app/community/vxgpi-nqaaa-aaaar-ar4lq-cai/?ref=xanzv-uaaaa-aaaaf-aneba-cai).

## 🧞 Commands

All commands are run from the root of the project, from a terminal:

| Command          | Action                                                      |
| :--------------- | :---------------------------------------------------------- |
| `npm install`    | Installs dependencies                                       |
| `npm run dev`    | Starts frontend dev server at `localhost:5173`              |
| `juno dev start` | Quickstart the local development emulator (requires Docker) |
| `npm run build`  | Build your production site to `./dist/`                     |
| `juno deploy`    | Deploy your project to a Satellite                          |
