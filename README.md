# Workshops Materials

This repository provides code samples and instructions that can be used to guide attendees in discovering Juno during workshops.

## Getting Started

Juno being framework-agnostic, the material is provided in various flavors. After cloning the repo, choose the framework that best suits your needs.

```bash
git clone https://github.com/junobuild/workshops
cd workshops
cd react|angular|vue
npm ci
```

## Workshop

We are developing a note-taking app and the core functionality is already functional. However, we still need to integrate Juno, which we plan to implement during the workshop.

By following the steps below and replacing the provided snippet, we will be able to implement the app and learn about building on Web3 simultaneously.

---

### Table of contents

1. [Initialization](#initialization)
2. [Authentication](#authentication)
3. [Storing Document](#storing-documents)
4. [Listing Document](#listing-documents)
5. [Uploading Files](#uploading-files)
6. [Deployment](#deployment)

---

### Initialization

Before we can integrate Juno into an app, we’ll need to create a satellite. This process is explained in detail in the [documentation](https://juno.build/docs/add-juno-to-an-app/create-a-satellite).

> New developers will also need to sign in to Juno's [console](https://console.juno.build) and may even need to create an Internet Identity.

Moreover, you also need to install the SDK.

```bash
npm i @junobuild/core
```

After completing both of these steps, we can initialize Juno with your satellite ID.

> TODO: find and replace STEP_1_INITIALIZATION

```javascript
await initJuno({
    satelliteId: 'replace-satellite-id'
})
```

---

### Authentication

To securely identify users anonymously, they will need to sign in.

> TODO: find and replace STEP_2_AUTH_SIGN_IN

```javascript
import { signIn } from "@junobuild/core";

await signIn();
```

Likewise, users should be able to sign out.

> TODO: find and replace STEP_3_AUTH_SIGN_OUT

```javascript
import { signOut } from "@junobuild/core";

await signOut();
```

To get to know the user’s state, Juno provides an observable function called `authSubscribe()`. We can use it as many times as required, but I find it convenient to subscribe to it at the top of an app.

> TODO: find and replace STEP_4_AUTH_SUBSCRIBE

```typescript
import { authSubscribe, type User } from '@junobuild/core';

const sub = authSubscribe((user: User | null) => console.log(user));
```

---

### Storing Documents

Storing data on the blockchain with Juno is done through a feature called “Datastore”. Follow the instructions in the documentation to create a collection, which can be named accordingly (“notes”).

Once our collection is created, we can persist data on the blockchain using the `setDoc` function.

> TODO: find and replace STEP_5_SET_DOC
 
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

### Listing Documents

To fetch the list of documents saved on the blockchain, we can use the `listDocs` function.

> TODO: find and replace STEP_6_LIST_DOCS
 
```javascript
const { items } = await listDocs({
  collection: "notes",
});
```

---

### Uploading Files

As for the documents, to upload assets we will need first to create a collection in the “Storage”. We can be name it “images”.

Once our collection is set, we can upload a file on chain using the `uploadFile` function.

> TODO: find and replace STEP_7_UPLOAD_FILE

```javascript
const { downloadUrl } = await uploadFile({
    collection: "images",
    data: file,
    filename,
});
```

In this particular workshop, we also want to save a reference within the document to its related asset.

> TODO: find and replace STEP_8_ADD_REFERENCE

```javascript
await setDoc({
    collection: "notes",
    doc: {
        key,
        data: {
            text: inputText,
            ...(url !== undefined && { url }) // <--- We add this reference
        },
    },
});
```

---

### Deployment

After we have developed and built our application, we can launch it.

To do this, we need the Juno CLI.

```bash
npm i -g @junobuild/cli
```

Once the installation is complete, we log in to grant access from our terminal to our satellite.

```bash
juno login
```

To inform the CLI about which satellites our application should be deployed to, we execute the initialization command once:

```bash
juno init
```

Finally, we deploy our app.

```bash
juno deploy
```

Congratulations! Your app is on chain 🎉.
