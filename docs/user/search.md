# Search and Discovery

InterNovel is designed to be a decentralized, local-first application. This means there is no central server holding everyone's novels. While this is great for privacy and ownership, it changes how searching works.

## How Search Works

Because your device doesn't download the entire universe of novels, you cannot perform a full-text search across every novel ever written. Instead, InterNovel uses a two-tiered search system:

### 1. The Contacts Directory
When an author publishes a novel, InterNovel automatically extracts the structured data (like Characters, Themes, and Locations) and creates a lightweight "Directory" entry. 

Your device automatically downloads the Directory entries for anyone in your **Contacts List**. 

When you search, you are instantly searching this local directory. You can use checkboxes to refine your search (e.g., "Search only Characters for 'Elara'"). If you find a novel you like, you can "Favorite" it to sync the full text to your device.

### 2. Network Search (Gossip Search)
If you want to search beyond your immediate contacts, you can send a "Network Query". 

This sends your search request to your contacts. If their app is open, it will silently search their local novels. If they have allowed it in their privacy settings, it will also pass your query along to *their* contacts. As matches are found, the results will ripple back to you.

## Background Sync
For Network Search to work, devices need to be online. InterNovel can be installed as a Progressive Web App (PWA). Depending on your browser and operating system, this allows InterNovel to occasionally wake up in the background to sync data and process network searches, even when you aren't actively using the app.

## Always-On Relays
To ensure your data is always available and your searches can always be routed, you can configure an "Always-On Relay" in your Settings. This is a hosted server that acts as a permanent peer in your network.
