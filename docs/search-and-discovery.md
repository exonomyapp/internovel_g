# Search and Discovery Architecture

InterNovel employs a 100% decentralized, local-first search architecture. Because it is impossible (and undesirable) for a single user's device to download the entire universe of all novels ever written, InterNovel uses a combination of **Structured Metadata Directories** and **Gossip Search (Query Routing)** to allow users to discover content across the network without centralized indexing servers.

## 1. Structured Metadata Directory (The Local Index)

Instead of relying on full-text search of every novel in existence, InterNovel leverages the structured data inherent in the writing process (Characters, Themes, Locations, Vertebrae, etc.).

### How it Works
1. **Extraction & Compression:** When a user publishes a novel, the application extracts all structured data.
2. **Key-Value Pairs:** This data is compressed into a JSON array of key-value pairs (e.g., `["Character:John", "Theme:Cyberpunk", "Location:Mars"]`).
3. **Directory Share:** This lightweight JSON metadata, along with the Novel's Title, Author DID, Synopsis, and `ShareAddress`, is published to a specific Earthstar "Directory Share".
4. **Selective Syncing:** A user's device **only** downloads the Directory Share data from members of their **Contacts List**, not the entire network. This keeps the local directory incredibly small and fast.

### The Compressed JSON Schema
To keep the Directory Share as lightweight as possible while retaining rich searchability, the structured data is flattened into a 1D array of prefixed strings. 

A typical published novel's metadata payload looks like this:
```json
{
  "novelId": "+novel.share.address",
  "authorDid": "did:peer:1z...",
  "title": "The Neon Rain",
  "synopsis": "A cyberpunk detective story...",
  "tags": [
    "Character:Elara Vance",
    "Character:Jax",
    "Theme:Cyberpunk",
    "Theme:Betrayal",
    "Location:Neo-Tokyo",
    "Location:The Underlevels",
    "Vertebra:The Heist",
    "Vertebra:The Escape"
  ]
}
```
**Why this format?**
- **Extensibility:** By using the `Category:Value` prefix pattern, we don't need to strictly define every possible array in the JSON schema. If a user creates a custom structured data type (e.g., "MagicSystem:Necromancy"), it seamlessly integrates into the search index.
- **Search UI Integration:** The UI can parse these prefixes to dynamically generate filter checkboxes (e.g., parsing all `Character:` tags to allow users to filter their search specifically by character names).

### Searching the Directory
Because the metadata is stored as key-value pairs, the search UI can provide advanced filtering. Users can use checkboxes to include, exclude, or limit searches to specific labels (e.g., "Search only Characters for 'John'").

## 2. Local-First Full-Text Search

Once a user discovers a novel via the Directory and chooses to sync it (by adding its `ShareAddress` to their local Earthstar Peer), the *entire* contents of that novel are downloaded to their device.

**Rule of Search:** All search queries *must* process all locally synced data (fully downloaded novels and the Contacts Directory) *before* routing the query outwardly to the network.

## 3. Gossip Search (The Network Index)

If a user wants to search beyond their immediate contacts and locally synced novels, InterNovel uses a P2P Query Routing mechanism (Gossip Search).

### The Inbox/Outbox Pattern
1. **The Query:** The searcher creates a "Query Document" containing the search terms, filters, a unique Query ID, and a `depth` limit (e.g., 2 levels of contacts).
2. **The Outbox:** This document is written to the searcher's personal Profile Share (their outbox).
3. **The Propagation:** Because the searcher's Contacts are syncing with their Profile Share, the Contacts' devices download the Query Document in the background.
4. **Background Processing:** When a Contact's InterNovel app (running as an active tab or a background PWA) receives the query, it checks the user's privacy settings:
   - **Allow Search:** The app silently searches the Contact's local, unpublished/private novels. If a match is found, a "Result Document" is routed back to the searcher.
   - **Allow Passthrough:** The app decrements the `depth` limit by 1 and writes the Query Document to *their* outbox, allowing *their* contacts to see it.
   - **Offline/Private:** The user can configure their app to allow passthrough routing without exposing their own local data.

### Always-On Relays & PWA Background Sync
For Gossip Search to work, devices must be online to receive, process, and pass along queries. 
- **Earthstar Relays:** To ensure connectivity even when browser tabs are closed, users can configure an "Always-On Relay" (a hosted Earthstar instance). By default, this can be set to the user's own `did:peer` or a community relay, and can be switched out in the settings without modifying code.
- **PWA Background Sync:** InterNovel utilizes Progressive Web App (PWA) technologies (like the Periodic Background Sync API) to allow the app to wake up periodically in the background, sync Earthstar shares, process pending Gossip Search queries, and route results, even when the UI is not actively presented to the user.
