# InterNovel Functional Specification

This document provides detailed functional specifications for the features outlined in the InterNovel Feature Roadmap.

## 1. Fair Attribution

**Problem:** Ensuring each contributor receives clear and undeniable credit for their specific inputs.

**Proposal:** Implement a DID-based attribution system where all creative contributions (paragraphs, character elements, plot points) are cryptographically linked to the author's DID.

*   **Details:**
    *   Use the `did:peer` method for W3C Decentralized Identifiers (DIDs) to anchor user identities. This method is chosen for its ability to support DID document updates without requiring a blockchain, making it ideal for evolving user profiles and service endpoints.
    *   Keys are generated and managed locally per device using a hybrid cryptographic approach: `@noble/curves` is used for Ed25519/X25519 mathematical operations and `did:peer` formatting to guarantee cross-browser protocol compliance, while the native Web Crypto API is leveraged strictly for secure key persistence and entropy.
    *   Use Willow P2P for decentralized, local-first storage and synchronization of user-generated content and data.
    *   Cryptographically sign contributions with the author's DID.
    *   Display attribution information clearly and transparently within the platform.
*   **UX Principles:** Clarity, Simplicity & Intuitiveness
*   **Notes for Developers:** Implement robust local key management and signing utilities.

## 2. Technical Overhead & External Service Integration

**Problem:** Managing version control, document storage, and collaborative permissions can be cumbersome.

**Proposal:** Streamline document management using local-first architecture.

*   **Details:**
    *   **Document Storage Integration:** Internovel will use Willow P2P for robust, decentralized, local-first document storage and synchronization. This ensures content availability, offline support, and performance.
    *   **Permission Management:** Utilize Willow's fine-grained access control capabilities to manage collaborative permissions securely and decentrally.
*   **UX Principles:** Simplicity & Intuitiveness, Clarity, Flexibility
*   **Notes for Developers:** Implement the Willow P2P protocol and design the data schema to support fine-grained access control.

### 3. Intuitive Collaborative Writing Environment

**Problem:** Providing a user-friendly and efficient environment for collaborative writing.

**Proposal:** Build a responsive and intuitive writing environment with real-time collaboration features.

*   **Details:**
    *   Use React 18 and Zustand for the frontend, leveraging Tailwind CSS for components and styling.
    *   **The Plot Planner (Brainstorming & Outlining):** Powered by a block-based editor (BlockNote), this environment is designed for playful, non-linear brainstorming. Every Novel, Vertebra, and Chapter has its own dedicated Plot Planner attached to it.
    *   **The Novel Editor (Drafting & Writing):** Powered by a traditional, flowing rich-text editor (TipTap), this environment is designed for deep, uninterrupted writing. It supports real-time, Google Docs-style simultaneous co-editing (via Yjs CRDTs), complete with visible multi-user cursors and presence indicators.
    *   **Real-Time Animated Drag-and-Drop:** (Flash: To facilitate seamless brainstorming, the Plot Planner supports real-time, animated drag-and-drop. When a user drags a block, their cursor and display name are broadcast to all peers, and the drag animation is recreated on their screens using a 333ms throttle and CSS transitions for fluid motion.)
    *   Include auto-saving functionality to ensure no work is lost during the writing process.
    *   Implement presence indicators, near real-time updates, and activity notifications via Willow P2P.
    *   **Local Data Safety, Backups & Restore:** Because InterNovel is a local-first application, the user's device is the primary database. To ensure data safety even for users who choose not to sync their work to the P2P network, the platform will utilize the browser's File System Access API to allow users to save a physical, encrypted backup file (e.g., `.sqlite` or `.zip`) directly to their hard drive.
        *   **Settings Integration:** Users can configure backup schedules directly in their personal Settings panel.
        *   **Multi-Platform Automation:** On supported desktop browsers, backups will automatically save to a designated folder on the scheduled interval.
        *   **Fallback Reminders:** On unsupported browsers (Safari, Mobile), the system will display a scheduled reminder prompting the user to manually download their backup zip.
        *   **Restore Functionality:** A robust restore mechanism allows users to drag-and-drop or select a previous backup file to completely restore their local database state.
*   **UX Principles:** Simplicity & Intuitiveness, Engagement
*   **Notes for Developers:** Focus on performance and responsiveness, utilizing Tailwind's utility classes for efficient rendering and layout.

## 4. Authorization and Moderation

**Problem:** Managing user permissions and content moderation, particularly within educational settings.

**Proposal:** Implement a tiered authorization system and moderation process.

*   **Details:**
    *   **Authorization Levels:** Admin, Moderator, Creator.
    *   **Moderator Approval:** Moderators must approve creations of Creators.
    *   **Student Account Moderation:** Student accounts created by a Moderating teacher require approval from the teacher and optionally other collaborators.
*   **UX Principles:** Clarity, Flexibility
*   **Notes for Developers:** Define roles and permissions, implement approval workflows using Willow's access control.

## 5. Non-Linear Idea Development

**Problem:** Supporting the often non-sequential way authors develop story ideas and narrative arcs.

**Proposal:** Enhance the "Novel Spine" feature to allow authors to capture and organize ideas in a non-linear manner while maintaining an underlying chronological framework.

*   **Details:**
    *   Allow authors to create "Vertebrae" representing core ideas, plot points, or thematic elements (sections of the Novel).
    *   Enable assigning documents (chapters, paragraphs) to specific Vertebrae.
    *   Provide a dual-view Sidebar: a "Spine View" (hierarchical, organized by Vertebrae) and a "Flat List" (all documents).
    *   Implement robust sorting options for Vertebrae: Alphabetical, Storyline Date (manual time stamping), Last Edited, and Busyness (number of child documents).
    *   Enable tagging and categorization of Vertebrae with relevant themes and topics.
    *   When publishing or exporting the novel, Vertebrae act as structural metadata and are not included in the final output.
*   **UX Principles:** Flexibility, Engagement, Clarity
*   **Notes for Developers:** Integrate manual time stamping functionality with the existing Novel Spine feature.

## 6. Character Portability & Cross-Narrative Potential

**Problem:** Enabling characters to organically exist and evolve across different stories and authorial domains.

**Proposal:** Further develop the "Character Travel trunk" concept to facilitate character portability and cross-narrative potential.

*   **Details:**
    *   Data schemas in Willow P2P store character information, including name, bio, and a link to the owner's DID.
    *   Travel trunks store narrative samples for each character.
    *   **Proposals System:** A collaborative workflow system that will enable character usage requests between authors.
*   **UX Principles:** Flexibility, Engagement
*   **Notes for Developers:** Integrate with the DID-based attribution and Willow P2P access control.

## 7. Discoverability and Community Features (Overview)

**Problem:** Enabling authors to find fitting collaborators or allow their creative assets to be discovered.

**Proposal:** Enhance community features to facilitate character and novel discovery and collaboration.

## 8. The Casting Studio: Functional Specification

**Problem:** Providing a dedicated space for authors to showcase and discover characters for collaboration.

**Proposal:** Implement a "Casting Studio" feature with character listing, discovery, and proposal initiation capabilities.

*   **Details:**
    *   **Character Showcase:** A dedicated area where authors can publicly list their original characters.
    *   **Discovery Hub:** Allows authors to browse and search for characters developed by others.
    *   **Facilitating Collaboration:** Authors who discover a character can initiate a proposal to the character's original author.
*   **UX Principles:** Engagement, Simplicity & Intuitiveness

## 9. The Novel Café: Functional Specification

**Problem:** Providing a dedicated space for authors to showcase novels and find collaborators.

**Proposal:** Implement a "Novel Café" feature with novel listing, collaboration calls, and discovery capabilities for collaborators.

*   **Details:**
    *   **Novel Showcase:** A dedicated space where authors can feature their novels.
    *   **Collaboration Calls:** Authors can express their desire for collaborators on specific novels.
    *   **Discovery for Collaborators:** Allows authors looking for projects to browse novels.
*   **UX Principles:** Engagement, Simplicity & Intuitiveness

## 10. Intellectual Property (IP) Clarity & Forking

**Problem:** Defining and protecting ownership rights in a multi-author environment, specifically handling derivative works and unauthorized copying.

**Proposal:** Develop a DID-based licensing framework and a strict "Forking" mechanism that guarantees immutable attribution and clear lineage for all creative assets.

*   **Details:**
    *   **The Forking Toggle:** Novel owners (e.g., Alice) have a boolean setting to allow or disallow "Forking" of their novel. Once the first fork occurs, this setting becomes permanently locked (irreversible) to protect the rights of the users who have already created derivative works based on the initial permission.
    *   **Derivative Work Lineage:** When a user (e.g., Bob) forks a novel, a new cryptographic Share is created. However, all existing content retains the original author's (Alice's) cryptographic DID signatures.
    *   **The "Remix" UI:** When readers view Bob's forked version of the novel, any content that Bob modifies from Alice's original will be visually flagged.
        *   **Indicator:** A small, clickable branching arrow or pencil icon will appear in the margin near the modified text block.
        *   **Tooltip:** Hovering over the icon will display a tooltip with the edit timestamp. The format will be "Edited MMM DD, YYYY" (if edited on a previous day) or "Edited HH:MM, today" (if edited today).
        *   **Lineage View (Responsive):** Clicking the indicator opens a history view showing the lineage of the object.
            *   *Mobile/Tablet:* Displays as an accordion-style drop-down directly below the paragraph.
            *   *Desktop:* Displays in a clean side-panel.
        *   **Comparison Styling:** The comparison between the original author's version and the remixed version will use standard diff styling (red lettering for removed/original text, green lettering for added/remixed text) to clearly portray the changes.
    *   **Immutable Attribution:** Because Alice's original contributions were signed with her private key, Bob cannot alter her text without breaking the signature. Any modification forces the system to recognize the new text as Bob's derivative work, permanently preserving Alice's original state in the object's history.
*   **UX Principles:** Clarity, Transparency, Respect for Creators

## 11. Accessibility for Young Creators

**Problem:** Providing tools that are powerful yet not overwhelmingly technical for emerging writers.

**Proposal:** Prioritize simplicity and intuitiveness in the design of all features.

*   **Details:**
    *   Use a visually appealing and interactive environment, built with Tailwind CSS.
    *   Provide clear explanations of all features.
*   **UX Principles:** Simplicity & Intuitiveness, Clarity

## 13. Novel & Image Gallery Management

**Problem:** Authors need a way to manage multiple novels and associate visual assets (character portraits, locations) with them, without bloating the P2P network.

**Proposal:** Implement a Novel Switcher and a dedicated Image Gallery with built-in image manipulation and size constraints.

*   **Details:**
    *   **Novel Switcher:** A UI component allowing users to create new novels and switch between active projects.
    *   **Image Gallery:** A dedicated space within a novel to upload and manage images.
    *   **Image Constraints:** To ensure Willow P2P sync remains performant, all uploaded images are automatically scaled down and compressed client-side to a maximum size of 1MB before storage.
    *   **Image Manipulation:** The gallery includes a client-side editor allowing authors to crop, rotate, and flip images.
    *   **Immutability of Edits:** Modified images are saved as entirely new images in the gallery, with no metadata association to the original source image.
*   **UX Principles:** Simplicity & Intuitiveness, Performance

## 14. Contacts & Group Management

**Problem:** Open collaboration can lead to chaos. Authors need a way to restrict collaboration to trusted individuals and manage them logically.

**Proposal:** Implement a Contacts system with subgroup management to strictly control who can receive capabilities.

*   **Details:**
    *   **Forced Relationships:** Capabilities (permissions) can *only* be shared with individuals who are in the user's "Contacts" group.
    *   **Subgroups:** Users can create custom subgroups under their Contacts (e.g., "Editors", "Beta Readers") and issue capabilities to these specific subgroups.
    *   **Revocation via Removal:** Removing an individual from a subgroup, or from the Contacts list entirely, automatically terminates any capabilities awarded to them in that specific context.
*   **UX Principles:** Security, Organization, Clarity
