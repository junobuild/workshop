# Workshops Materials

This repository provides code samples and instructions that can be used to guide attendees in discovering Juno during workshops.

## Getting Started

Juno being framework-agnostic, the material is provided in various flavors. After cloning the repo, choose the framework that best suits your needs.

```bash
git clone https://github.com/buildwithjuno/workshop
cd workshop
cd react|angular|vue
```

## Workshop

We are developing a note-taking app and the core functionality is already functional. However, we still need to integrate Juno, which we plan to implement during the workshop.

By following the steps below and replacing the provided snippet, we will be able to implement the app and learn about building on Web3 simultaneously.

---

### Table of contents

1. [Initialization](#1-initialization)

---

### Initialization

Before we can integrate Juno into an app, we’ll need to create a satellite. This process is explained in detail in the [documentation](https://juno.build/docs/add-juno-to-an-app/create-a-satellite).

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
