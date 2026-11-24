## InterNovel: Vision Scope Document

### **1. Introduction & Project Vision**

InterNovel is a revolutionary collaborative writing platform designed to empower authors, particularly young creators such as high school students, to co-create narratives, share worlds, and retain clear ownership of their intellectual contributions. Our vision is to foster a dynamic literary ecosystem where creativity flourishes through seamless collaboration, the discovery of characters and novels, and shared storytelling, underpinned by modern, user-centric technology.

InterNovel will manifest as an engaging and intuitive local-first application. Architected with true decentralization and peer-to-peer data synchronization in mind, the platform will prioritize a smooth, performant user experience to ensure accessibility and ease of use for our target audience, while protecting their data sovereignty.

### **2. Core Problem Addressed**

Traditional collaborative writing often struggles with:

*   **Fair Attribution:** Ensuring each contributor receives clear and undeniable credit for their specific inputs.
*   **Intellectual Property (IP) Clarity:** Defining and protecting ownership rights in a multi-author environment.
*   **Non-Linear Idea Development:** Supporting the often non-sequential way authors develop story ideas and narrative arcs.
*   **Structural & Thematic Cohesion:** Managing the overall structure (e.g., sections, acts) and maintaining thematic consistency across multiple contributors and evolving narratives.
*   **Discoverability & Organic Collaboration:** Enabling authors to find fitting collaborators or allow their creative assets (like characters and novels) to be discovered and integrated into new narratives.
*   **Accessibility for Young Creators:** Providing tools that are powerful yet not overwhelmingly technical for emerging writers.
*   **Character Portability & Cross-Narrative Potential:** Enabling characters to organically exist and evolve across different stories and authorial domains.
*   **Technical Overhead:** Managing version control, document storage, and collaborative permissions can be cumbersome.

InterNovel aims to solve these by integrating decentralized identity principles (`did:peer`) and local-first peer-to-peer synchronization (Willow P2P) with user-friendly collaborative, organizational, and discovery tools.

### **3. Target Audience**

*   **Primary:** High school students and young aspiring authors interested in collaborative creative writing, character development, and sharing their literary works.
*   **Secondary:** Hobbyist writers, writing groups, and educators seeking innovative tools for creative projects.

### **4. Key Features & Strategic Pillars**

We will focus on delivering a core set of features that establish InterNovel's unique value proposition:

#### 4.1. Decentralized Identity at the Core

*   **Secure, Self-Managed Identity:** InterNovel provides each user with a secure, self-managed digital identity using `did:peer`. This identity is generated and managed locally per device using a hybrid cryptographic approach (`@noble/curves` for protocol compliance and Web Crypto API for secure entropy/storage), giving authors true control over their creative work and ensuring that their contributions are uniquely and verifiably linked to them.
*   **Immutable Attribution:** All creative contributions (paragraphs, character elements, plot points) will be verifiably linked to the author's identity, providing clear and immutable attribution.

#### 4.2. The "Character Travel trunk": A Novel Collaborative Paradigm

*   **Inter-Story Character Introduction:** Authors can propose one of their original characters for inclusion in another author's story.
*   **Travel trunk Contents:**
    *   **Character's Narrative Sample:** Specific narrative passage(s) featuring the character.
    *   **Character Biography Link:** A link back to the character's comprehensive bio page.
*   **Proposal & Acceptance:** When an author wishes to invite a character into their novel, they craft a proposal. The original author reviews and can accept, optionally granting permissions to use the character and collaborating on the integration.

#### 4.3. The "Novel Spine": Flexible Story Structuring

*   **Idea-Centric Organization:** Authors can construct a "Novel Spine" composed of "Vertebrae," where each Vertebra represents a core idea, plot point, scene concept, or thematic element.
*   **Manual Time Stamping:** Vertebrae can be manually time stamped to establish an inherent chronological framework.
*   **Tagging & Categorization:** Vertebrae can be tagged with relevant themes and topics.
*   **Non-Linear Development:** Build the foundational skeleton of the novel in a non-sequential manner.
*   **Dynamic Storyline:** Arrange and rearrange Vertebrae to experiment with different chronological sequences.

#### 4.4. Intuitive Collaborative Writing Environment

*   **Smooth and Responsive Interface:** The platform provides a smooth writing environment designed for easy collaboration.
*   **Real-Time Collaboration & Awareness:** Powered by Willow P2P, authors will sync changes in real-time when online, and merge seamlessly when offline.
*   **Core Writing Tools:** Text editing, version tracking, and tools for managing narrative structure.
*   **Permission System:** Authors will have granular control over who can access and contribute to their work via Willow's fine-grained access control.

#### 4.5. The "Casting Studio": Character Discovery & Showcase

*   **Character Showcase:** A dedicated area where authors can publicly or semi-publicly list their original characters.
*   **Discovery Hub:** Browse and search for characters developed by others.

#### 4.6. The "Novel Café": Novel Discovery & Collaboration Calls

*   **Novel Showcase:** A dedicated space where authors can feature their novels.
*   **Collaboration Calls:** Authors can express their desire for collaborators on specific novels.

#### 4.7. Local-First Foundation with Willow P2P

*   **Robust and Reliable Foundation:** Built on a local-first architecture utilizing the Willow P2P protocol. Data is stored locally on the user's device first, ensuring offline capability and true data ownership, then synchronized peer-to-peer with authorized collaborators.

#### 4.8. Authorization and Moderation

InterNovel will support a tiered authorization system to manage user permissions and content moderation, particularly within educational settings.

### **5. User Experience (UX) Principles**

Leveraging React and Tailwind CSS to implement these principles:

*   **Simplicity & Intuitiveness:** Prioritize ease of use, especially for the target audience of high school students. Abstract technical complexities.
*   **Engagement:** Create a visually appealing and interactive environment.
*   **Clarity:** Ensure all features, especially those related to IP, are clearly explained.
*   **Flexibility:** Support diverse writing styles and non-linear creative processes.
*   **Responsive Design:** Utilize Tailwind CSS utility classes to ensure the platform is accessible across all devices.

### **6. Success Metrics**

*   User adoption and active engagement.
*   Successful implementation of the `did:peer` based attribution system per device.
*   Stability and performance of the Willow P2P synchronization.

### **7. Future Vision**

*   **Enhanced DID Management:** Integration with W3C WebAuthn for even more user-friendly DID authentication.
*   **Advanced Collaborative Tools:** More sophisticated version control, branching narratives, full co-editing features.
*   **Monetization & Licensing:** Exploration of DID-based licensing frameworks.
