import './../style/input.css';
import { renderMarkdownHtml } from './markdown';

const docModules = import.meta.glob('/docs/**/*.md', {
  query: '?raw',
  import: 'default',
  eager: true,
}) as Record<string, string>;

interface DocEntry {
  path: string;
  slug: string;
  title: string;
  group: 'docs' | 'user';
  content: string;
}

const docEntries: DocEntry[] = Object.entries(docModules)
  .filter(([p]) => !p.startsWith('/docs/agent/'))
  .map(([p, content]) => {
    const group: 'docs' | 'user' = p.startsWith('/docs/user/') ? 'user' : 'docs';
    const file = p.split('/').pop() ?? '';
    const slug = file.replace(/\.md$/, '');
    const title = slug
      .split('-')
      .map((w) => w.charAt(0).toUpperCase() + w.slice(1))
      .join(' ');
    return { path: p, slug, title, group, content };
  })
  .sort((a, b) => a.title.localeCompare(b.title));

type Route = 'studio' | 'casting' | 'cafe' | 'search' | 'auth' | 'profile' | 'doc';
type StudioTab = 'editor' | 'planner' | 'contacts' | 'gallery';
type EditorView = 'edit' | 'split' | 'preview';
type SpineSort = 'date' | 'alpha' | 'edited' | 'busy';
type ThemeMode = 'light' | 'dark' | 'system';
type CropAspect = 'free' | '16:9' | '4:3' | '1:1' | '2:3';
type CafeFilter = 'all' | 'collab' | 'forkable' | 'remixes';
type SearchSort = 'relevance' | 'date' | 'alpha';

interface Vertebra {
  id: string;
  kind: 'Idea' | 'Plot Point' | 'Theme' | 'Chapter';
  title: string;
  storylineDate: string;
  lastEditedTs: number;
  childCount: number;
  tags: string[];
  content: string;
  plotBlocks: string[];
}

interface GalleryImage {
  id: string;
  title: string;
  dataUrl: string;
  width: number;
  height: number;
  sizeKb: number;
}

interface CollabPitch {
  id: string;
  author: string;
  text: string;
  date: string;
}

interface Novel {
  id: string;
  title: string;
  synopsis: string;
  collabCall: string;
  seekingCollaborators: boolean;
  forkingAllowed: boolean;
  forkingLocked: boolean;
  favorited: boolean;
  remixedFromId?: string;
  remixedFromTitle?: string;
  remixedBaselineText?: string;
  lastVertebraId?: string;
  lastStudioTab?: StudioTab;
  lastEditorView?: EditorView;
  lastScrollTop?: number;
  lastRoute?: 'studio' | 'cafe';
  pitches: CollabPitch[];
  vertebrae: Vertebra[];
  gallery: GalleryImage[];
}

interface Account {
  displayName: string;
  bio: string;
  avatarInitials: string;
  avatarDataUrl: string;
  didShort: string;
  didLong: string;
  securityText: string;
  inMemory: boolean;
}

interface ContactSubgroup {
  name: string;
  members: string[];
}

interface CharacterTrunk {
  id: string;
  name: string;
  ownerName: string;
  ownerDid: string;
  bio: string;
  sample: string;
  proposedTo: string | null;
}

interface CropDraft {
  sourceDataUrl: string;
  sourceImg: HTMLImageElement | null;
  alt: string;
  aspect: CropAspect;
  zoom: number;
  rotation: number;
  flipX: boolean;
  flipY: boolean;
  cropX: number;
  cropY: number;
  cropW: number;
  cropH: number;
  insertOnSave: boolean;
}

interface NetworkItem {
  id: string;
  kind: 'novel' | 'trunk';
  hop: number;
  peerLabel: string;
  peerDid: string;
  title: string;
  summary: string;
  sample: string;
  tags: string[];
  storylineDate: string;
}

const PEER_NETWORK_CATALOG: NetworkItem[] = [
  {
    id: 'net-novel-astrolabe',
    kind: 'novel',
    hop: 1,
    peerLabel: 'Elena V. (Editors subgroup)',
    peerDid: 'did:peer:4zQmCoAuthorElena',
    title: 'The Brass Astrolabe',
    summary: 'Clockmaker siblings map forgotten transit tunnels beneath an observatory island.',
    sample: 'Every gear inside the east dome carried two signatures: the guild master and the runaway.',
    tags: ['Steampunk', 'Mystery', 'Location:Observatory'],
    storylineDate: '1896-11-04',
  },
  {
    id: 'net-trunk-vesper',
    kind: 'trunk',
    hop: 1,
    peerLabel: 'Elena V. (Editors subgroup)',
    peerDid: 'did:peer:4zQmCoAuthorElena',
    title: 'Captain Vesper Hale',
    summary: 'Archivist-navigator who reads pressure gauges by ear across fogbound canals.',
    sample: '"Turn the valve a quarter past cedar," Vesper murmured as the lantern dimmed.',
    tags: ['Character:Vesper', 'Maritime', 'Travel Trunk'],
    storylineDate: '1896-08-19',
  },
  {
    id: 'net-novel-tidepool',
    kind: 'novel',
    hop: 2,
    peerLabel: 'Marcus T. (Beta Readers)',
    peerDid: 'did:peer:4zQmReaderMarcus',
    title: 'Tidepool Cartography',
    summary: 'Interlocking novellas written from three coastal weather stations during an eclipse summer.',
    sample: 'When the tide retreated past Marker Nine, the sunken telegraph cable still ticked in Morse.',
    tags: ['Epistolary', 'Coastal', 'Location:Marker Nine'],
    storylineDate: '2028-06-14',
  },
  {
    id: 'net-novel-underground',
    kind: 'novel',
    hop: 3,
    peerLabel: 'Writers Guild Relay',
    peerDid: 'did:peer:4zQmGuildRelay03',
    title: 'Letters from the Underground Press',
    summary: 'Anthology of clandestine printmakers trading ciphered broadsheets across orbital colonies.',
    sample: 'No censor could confiscate lead type once the apprentices memorized every galley proof.',
    tags: ['Sci-Fi', 'Sci-Fi Print', 'Location:Ceres Station'],
    storylineDate: '2144-02-22',
  },
  {
    id: 'net-trunk-orion',
    kind: 'trunk',
    hop: 4,
    peerLabel: 'Far Neighborhood Relay',
    peerDid: 'did:peer:4zQmFarPeer04',
    title: 'Orion Kestrel',
    summary: 'Acoustic restorer rescuing damaged wax-cylinder recordings from abandoned libraries.',
    sample: 'Between scratches on cylinder twelve, another voice whispered the true chapter ending.',
    tags: ['Character:Orion', 'Archive', 'Mystery'],
    storylineDate: '1902-05-09',
  },
];

const STORAGE_KEY = 'internovel_workspace_v2';

const defaultState = {
  route: 'studio' as Route,
  activeDocPath: docEntries.find((d) => d.slug === 'intro')?.path ?? docEntries[0]?.path ?? '',
  docsMenuOpen: false,
  userMenuOpen: false,
  accountMenuOpen: false,
  settingsMenuOpen: false,
  mobileMenuOpen: false,
  newNovelOpen: false,
  editingNovelOpen: false,
  confirmDeleteNovelId: null as string | null,
  confirmDeleteVertebraId: null as string | null,
  editingBlockIdx: null as number | null,
  editingSubgroupIdx: null as number | null,
  editingGalleryIdx: null as number | null,
  editingTrunkId: null as string | null,
  confirmDeleteTrunkId: null as string | null,
  cafeFilter: 'all' as CafeFilter,
  editingCafeNovelId: null as string | null,
  confirmDeleteCafeNovelId: null as string | null,
  confirmDeleteAccountDid: null as string | null,
  themeMode: 'system' as ThemeMode,
  textScale: 100,
  alwaysOnRelayUrl: '',
  backupSchedule: 'Weekly',
  spineCollapsed: false,
  spineView: true,
  spineSort: 'date' as SpineSort,
  studioTab: 'editor' as StudioTab,
  editorView: 'split' as EditorView,
  splitRatio: 50,
  editorCursorPos: null as number | null,
  imageModalOpen: false,
  cropDraft: null as CropDraft | null,
  activeNovelIdx: 0,
  selectedVertebraId: 'v-1',
  searchQuery: '',
  searchNovelScope: 'all',
  searchSort: 'relevance' as SearchSort,
  searchCategories: {
    Chapters: true,
    Characters: true,
    Themes: true,
    Locations: true,
    PlotBlocks: true,
    Images: true,
    Docs: true,
  },
  gossipDepth: 2,
  gossipStatus: '',
  gossipHits: [] as NetworkItem[],
  importedNetworkIds: [] as string[],
  remixDiffOpenFor: null as string | null,
  statusNotice: '',
  activeAccount: {
    displayName: 'Alex Rivera',
    bio: 'Collaborative novelist & world-builder.',
    avatarInitials: 'AR',
    avatarDataUrl: '',
    didShort: 'did:peer:4zQmLocalAuthor9x',
    didLong: 'did:peer:4zQmLocalAuthor9x:z2vDInputDocument',
    securityText: 'amber-lantern-harbor',
    inMemory: false,
  } as Account | null,
  accounts: [
    {
      displayName: 'Alex Rivera',
      bio: 'Collaborative novelist & world-builder.',
      avatarInitials: 'AR',
      avatarDataUrl: '',
      didShort: 'did:peer:4zQmLocalAuthor9x',
      didLong: 'did:peer:4zQmLocalAuthor9x:z2vDInputDocument',
      securityText: 'amber-lantern-harbor',
      inMemory: false,
    },
  ] as Account[],
  subgroups: [
    { name: 'Editors', members: ['did:peer:4zQmCoAuthorElena'] },
    { name: 'Beta Readers', members: ['did:peer:4zQmReaderMarcus'] },
  ] as ContactSubgroup[],
  novels: [
    {
      id: 'novel-1',
      title: 'The Clockwork Archivist',
      synopsis: 'An apprentice archivist uncovers missing chapters across a subterranean library.',
      collabCall: 'Seeking a beta reader or historical architecture adviser for Act II subplots.',
      seekingCollaborators: true,
      forkingAllowed: true,
      forkingLocked: false,
      favorited: true,
      pitches: [
        {
          id: 'pitch-1',
          author: 'Elena V.',
          text: 'Happy to contribute botanical marginalia sketches for Chapter 2!',
          date: '2026-02-10',
        },
      ],
      gallery: [],
      vertebrae: [
        {
          id: 'v-1',
          kind: 'Idea',
          title: 'Prologue: Ink on Brass',
          storylineDate: '1894-03-12',
          lastEditedTs: 170,
          childCount: 2,
          tags: ['Steampunk', 'Archive', 'Location:Clocktower'],
          content:
            '# Prologue: Ink on Brass\n\nBegin drafting your opening scene here. **Structural Vertebrae** keep your outline organized privately while published chapters share seamlessly with collaborators.\n\n- Capture non-linear story beats in any order\n- Drag or drop images directly into this editor or use the **+ Image** button above to crop and insert artwork cleanly',
          plotBlocks: [
            'Archivist discovers a sealed brass ledger behind the east stacks',
            'Apprentice decodes the marginalia cipher before dawn',
          ],
        },
        {
          id: 'v-2',
          kind: 'Plot Point',
          title: 'Chapter 1: The Sealed Ledger',
          storylineDate: '1894-03-18',
          lastEditedTs: 210,
          childCount: 2,
          tags: ['Mystery', 'Co-Authored', 'Character:Lyra'],
          content:
            '# Chapter 1: The Sealed Ledger\n\nEvery contribution stays linked to the author who wrote it.\n\n> "Ink remembers who held the nib."',
          plotBlocks: [
            'Midnight rendezvous at the foundry',
            'Exchange Character Travel Trunk notes with co-author',
          ],
        },
      ],
    },
    {
      id: 'novel-2',
      title: 'Letters from Orbit',
      synopsis: 'Epistolary sci-fi told across deep-space relay stations.',
      collabCall: 'Open for co-authors writing parallel transmissions from outer belt relays.',
      seekingCollaborators: true,
      forkingAllowed: true,
      forkingLocked: true,
      favorited: false,
      pitches: [],
      gallery: [],
      vertebrae: [
        {
          id: 'v-3',
          kind: 'Theme',
          title: 'Transmission 01',
          storylineDate: '2140-09-01',
          lastEditedTs: 195,
          childCount: 1,
          tags: ['Sci-Fi', 'Epistolary', 'Location:Lagrange L4'],
          content: '# Transmission 01\n\nSignal delay: 14 minutes across Lagrange point L4.',
          plotBlocks: ['Opening telemetry log from Relay Four'],
        },
      ],
    },
  ] as Novel[],
  trunks: [
    {
      id: 'tr-1',
      name: 'Lyra Vance',
      ownerName: 'Alex Rivera',
      ownerDid: 'did:peer:4zQmClockworkLyra',
      bio: 'Former clocktower cartographer carrying a brass astrolabe keyed to lost libraries.',
      sample: 'Lyra tapped the bezel twice until the hidden ink surfaced along the margin.',
      proposedTo: null,
    },
  ] as CharacterTrunk[],
  lastPazzle: ['🦊', '🧭', '🕯️', '🪶', '🕰️', '🗝️', '📜', '🌙'],
  shuffledPin: [3, 8, 1, 6, 0, 9, 4, 2, 7, 5],
};

function normalizeLoadedNovels(rawNovels: Novel[]): Novel[] {
  return rawNovels.map((n) => {
    const vertebrae = Array.isArray(n.vertebrae)
      ? n.vertebrae.map((v) => ({
          ...v,
          tags: Array.isArray(v.tags) ? v.tags : [],
          plotBlocks: Array.isArray(v.plotBlocks) ? v.plotBlocks : [],
        }))
      : [];
    const validLastVertebra = vertebrae.some((v) => v.id === n.lastVertebraId)
      ? n.lastVertebraId
      : vertebrae[0]?.id ?? '';
    return {
      ...n,
      collabCall: n.collabCall ?? '',
      pitches: Array.isArray(n.pitches) ? n.pitches : [],
      gallery: Array.isArray(n.gallery) ? n.gallery : [],
      vertebrae,
      lastVertebraId: validLastVertebra,
      lastStudioTab: n.lastStudioTab ?? 'editor',
      lastEditorView: n.lastEditorView ?? 'split',
      lastScrollTop: typeof n.lastScrollTop === 'number' ? n.lastScrollTop : 0,
      lastRoute: n.lastRoute === 'cafe' ? 'cafe' : 'studio',
    };
  });
}

function loadState() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return structuredClone(defaultState);
    const parsed = JSON.parse(raw);
    const base = structuredClone(defaultState);
    const novels = Array.isArray(parsed.novels) ? normalizeLoadedNovels(parsed.novels) : normalizeLoadedNovels(base.novels);
    const activeIdx =
      typeof parsed.activeNovelIdx === 'number' && parsed.activeNovelIdx >= 0 && parsed.activeNovelIdx < novels.length
        ? parsed.activeNovelIdx
        : 0;
    const activeNv = novels[activeIdx];
    const restoredVertebra =
      activeNv && activeNv.vertebrae.some((v) => v.id === activeNv.lastVertebraId)
        ? activeNv.lastVertebraId!
        : typeof parsed.selectedVertebraId === 'string' && activeNv?.vertebrae.some((v) => v.id === parsed.selectedVertebraId)
          ? parsed.selectedVertebraId
          : activeNv?.vertebrae[0]?.id ?? base.selectedVertebraId;
    return {
      ...base,
      ...parsed,
      novels,
      activeNovelIdx: activeIdx,
      selectedVertebraId: restoredVertebra,
      studioTab: activeNv?.lastStudioTab ?? parsed.studioTab ?? base.studioTab,
      editorView: activeNv?.lastEditorView ?? parsed.editorView ?? base.editorView,
      searchCategories: {
        ...base.searchCategories,
        ...(parsed.searchCategories ?? {}),
      },
      docsMenuOpen: false,
      userMenuOpen: false,
      accountMenuOpen: false,
      settingsMenuOpen: false,
      mobileMenuOpen: false,
      newNovelOpen: false,
      editingNovelOpen: false,
      confirmDeleteNovelId: null,
      confirmDeleteVertebraId: null,
      editingBlockIdx: null,
      editingSubgroupIdx: null,
      editingGalleryIdx: null,
      editingTrunkId: null,
      confirmDeleteTrunkId: null,
      editingCafeNovelId: null,
      confirmDeleteCafeNovelId: null,
      confirmDeleteAccountDid: null,
      imageModalOpen: false,
      cropDraft: null,
      statusNotice: '',
    };
  } catch {
    return structuredClone(defaultState);
  }
}

const state = loadState();
let pendingRestoreScrollTop: number | null = null;
let scrollPersistTimer: number | null = null;

function persistState() {
  try {
    if (state.activeAccount?.inMemory) return;
    localStorage.setItem(
      STORAGE_KEY,
      JSON.stringify({
        route: state.route,
        themeMode: state.themeMode,
        textScale: state.textScale,
        alwaysOnRelayUrl: state.alwaysOnRelayUrl,
        backupSchedule: state.backupSchedule,
        spineCollapsed: state.spineCollapsed,
        spineView: state.spineView,
        spineSort: state.spineSort,
        studioTab: state.studioTab,
        editorView: state.editorView,
        splitRatio: state.splitRatio,
        activeNovelIdx: state.activeNovelIdx,
        selectedVertebraId: state.selectedVertebraId,
        activeAccount: state.activeAccount,
        accounts: state.accounts,
        subgroups: state.subgroups,
        novels: state.novels,
        trunks: state.trunks,
        importedNetworkIds: state.importedNetworkIds,
      })
    );
  } catch {
    // Ignore storage quota errors
  }
}

function escapeHtml(raw: string): string {
  return raw
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

function countWords(text: string): number {
  const trimmed = text.trim();
  return trimmed ? trimmed.split(/\s+/).length : 0;
}

function resolveNovelImage(url: string): string | null {
  if (!url.startsWith('internovel-img://')) return null;
  const imgId = url.slice('internovel-img://'.length);
  for (const novel of state.novels as Novel[]) {
    const hit = novel.gallery.find((g) => g.id === imgId);
    if (hit) return hit.dataUrl;
  }
  return null;
}

function applyTheme() {
  const prefersDark =
    typeof window !== 'undefined' &&
    window.matchMedia?.('(prefers-color-scheme: dark)').matches;
  const isDark = state.themeMode === 'dark' || (state.themeMode === 'system' && prefersDark);
  document.documentElement.classList.toggle('dark', isDark);
}

function sortedVertebrae(novel: Novel): Vertebra[] {
  const copy = [...novel.vertebrae];
  if (state.spineSort === 'alpha') copy.sort((a, b) => a.title.localeCompare(b.title));
  else if (state.spineSort === 'date') copy.sort((a, b) => a.storylineDate.localeCompare(b.storylineDate));
  else if (state.spineSort === 'edited') copy.sort((a, b) => b.lastEditedTs - a.lastEditedTs);
  else copy.sort((a, b) => b.childCount - a.childCount);
  return copy;
}

function closeAllMenus() {
  state.docsMenuOpen = false;
  state.userMenuOpen = false;
  state.accountMenuOpen = false;
  state.settingsMenuOpen = false;
  state.mobileMenuOpen = false;
}

function getActiveNovel(): Novel | undefined {
  if (!state.novels.length) return undefined;
  if (state.activeNovelIdx < 0 || state.activeNovelIdx >= state.novels.length) {
    state.activeNovelIdx = 0;
  }
  return state.novels[state.activeNovelIdx];
}

function captureActiveScrollTop(): number | undefined {
  const editorEl = document.getElementById('editor-textarea') as HTMLTextAreaElement | null;
  if (editorEl) return editorEl.scrollTop;
  const previewEl = document.getElementById('editor-preview') as HTMLElement | null;
  if (previewEl) return previewEl.scrollTop;
  return undefined;
}

function snapshotCurrentNovelLocation() {
  const nv = getActiveNovel();
  if (!nv) return;
  if (nv.vertebrae.some((v) => v.id === state.selectedVertebraId)) {
    nv.lastVertebraId = state.selectedVertebraId;
  }
  nv.lastStudioTab = state.studioTab;
  nv.lastEditorView = state.editorView;
  if (state.route === 'studio' || state.route === 'cafe') {
    nv.lastRoute = state.route;
  }
  const scrollTop = captureActiveScrollTop();
  if (typeof scrollTop === 'number') {
    nv.lastScrollTop = scrollTop;
  }
}

function restoreNovelLocation(
  idx: number,
  options?: { keepRoute?: boolean; overrideVertebraId?: string; overrideTab?: StudioTab }
) {
  state.activeNovelIdx = idx;
  const target = state.novels[idx] as Novel | undefined;
  if (!target) return;
  const requestedId = options?.overrideVertebraId ?? target.lastVertebraId;
  const validId =
    requestedId && target.vertebrae.some((v) => v.id === requestedId)
      ? requestedId
      : target.vertebrae[0]?.id ?? '';
  state.selectedVertebraId = validId;
  state.studioTab = options?.overrideTab ?? target.lastStudioTab ?? 'editor';
  state.editorView = target.lastEditorView ?? 'split';
  if (!options?.keepRoute && target.lastRoute) {
    state.route = target.lastRoute;
  }
  pendingRestoreScrollTop = options?.overrideVertebraId ? 0 : (target.lastScrollTop ?? 0);
  target.lastVertebraId = validId;
  target.lastStudioTab = state.studioTab;
  target.lastEditorView = state.editorView;
}

function switchToNovel(
  idx: number,
  options?: { keepRoute?: boolean; overrideVertebraId?: string; overrideTab?: StudioTab }
) {
  snapshotCurrentNovelLocation();
  restoreNovelLocation(idx, options);
}

function renderAvatarBadge(acc: Account, sizeClass = 'w-5 h-5 text-[10px]'): string {
  if (acc.avatarDataUrl) {
    return `<img src="${escapeHtml(acc.avatarDataUrl)}" alt="${escapeHtml(acc.displayName)}" class="${sizeClass} rounded-full object-cover shrink-0 border border-[#e5e5e0] dark:border-[#3a3a35]" />`;
  }
  const initials = acc.avatarInitials || acc.displayName.slice(0, 2).toUpperCase();
  return `<span class="${sizeClass} rounded-full bg-[#5A5A40] text-white inline-flex items-center justify-center shrink-0 font-bold">${escapeHtml(initials)}</span>`;
}

function insertMarkdownAtCursor(snippet: string) {
  const novel = getActiveNovel();
  const v = novel?.vertebrae.find((x) => x.id === state.selectedVertebraId);
  if (!v) return;

  const pos =
    typeof state.editorCursorPos === 'number' && state.editorCursorPos <= v.content.length
      ? state.editorCursorPos
      : v.content.length;
  const before = v.content.slice(0, pos);
  const after = v.content.slice(pos);
  const prefix = before.length > 0 && !before.endsWith('\n') ? '\n' : '';
  const suffix = after.length > 0 && !after.startsWith('\n') ? '\n' : '';
  v.content = `${before}${prefix}${snippet}${suffix}${after}`;
  v.lastEditedTs = Date.now();
  state.editorCursorPos = pos + prefix.length + snippet.length + suffix.length;
}

function insertGalleryImageIntoChapter(img: GalleryImage) {
  const markdownTag = `![${img.title || 'Scene Illustration'}](internovel-img://${img.id})\n`;
  insertMarkdownAtCursor(markdownTag);
  state.route = 'studio';
  state.studioTab = 'editor';
  if (state.editorView === 'edit') {
    state.editorView = 'split';
  }
  state.imageModalOpen = false;
  state.statusNotice = `Inserted "${img.title}" into the active chapter.`;
  render();
}

function startCropSessionFromDataUrl(dataUrl: string, defaultAlt: string, insertOnSave: boolean) {
  const imgEl = new Image();
  imgEl.onload = () => {
    state.cropDraft = {
      sourceDataUrl: dataUrl,
      sourceImg: imgEl,
      alt: defaultAlt.replace(/\.[^.]+$/, ''),
      aspect: 'free',
      zoom: 1,
      rotation: 0,
      flipX: false,
      flipY: false,
      cropX: 0.08,
      cropY: 0.08,
      cropW: 0.84,
      cropH: 0.84,
      insertOnSave,
    };
    state.imageModalOpen = true;
    render();
    drawCropCanvas();
  };
  imgEl.src = dataUrl;
}

function handleFileForCrop(file: File, insertOnSave: boolean) {
  if (!file.type.startsWith('image/')) {
    state.statusNotice = 'Please select an image file (PNG, JPEG, WebP, or GIF).';
    render();
    return;
  }
  const reader = new FileReader();
  reader.onload = () => {
    startCropSessionFromDataUrl(String(reader.result), file.name, insertOnSave);
  };
  reader.readAsDataURL(file);
}

function bakeCroppedImage(draft: CropDraft): { dataUrl: string; width: number; height: number; sizeKb: number } {
  const img = draft.sourceImg;
  if (!img) {
    return { dataUrl: draft.sourceDataUrl, width: 0, height: 0, sizeKb: 1 };
  }

  const rotRad = (draft.rotation * Math.PI) / 180;
  const swapped = draft.rotation % 180 !== 0;
  const rawW = swapped ? img.height : img.width;
  const rawH = swapped ? img.width : img.height;

  const stageMax = 1400;
  const baseScale = Math.min(1, stageMax / Math.max(rawW, rawH));
  const stageW = Math.max(1, Math.round(rawW * baseScale));
  const stageH = Math.max(1, Math.round(rawH * baseScale));

  const stage = document.createElement('canvas');
  stage.width = stageW;
  stage.height = stageH;
  const sctx = stage.getContext('2d')!;
  sctx.save();
  sctx.translate(stageW / 2, stageH / 2);
  sctx.rotate(rotRad);
  sctx.scale((draft.flipX ? -1 : 1) * draft.zoom, (draft.flipY ? -1 : 1) * draft.zoom);
  const drawW = img.width * baseScale;
  const drawH = img.height * baseScale;
  sctx.drawImage(img, -drawW / 2, -drawH / 2, drawW, drawH);
  sctx.restore();

  const sx = Math.max(0, Math.round(draft.cropX * stageW));
  const sy = Math.max(0, Math.round(draft.cropY * stageH));
  const sw = Math.max(16, Math.round(draft.cropW * stageW));
  const sh = Math.max(16, Math.round(draft.cropH * stageH));

  const out = document.createElement('canvas');
  out.width = Math.min(sw, stageW - sx);
  out.height = Math.min(sh, stageH - sy);
  out.getContext('2d')!.drawImage(stage, sx, sy, out.width, out.height, 0, 0, out.width, out.height);

  let quality = 0.88;
  let dataUrl = out.toDataURL('image/jpeg', quality);
  while (dataUrl.length * 0.75 > 980 * 1024 && quality > 0.35) {
    quality -= 0.12;
    dataUrl = out.toDataURL('image/jpeg', quality);
  }
  const sizeKb = Math.max(1, Math.round((dataUrl.length * 0.75) / 1024));
  return { dataUrl, width: out.width, height: out.height, sizeKb };
}

function drawCropCanvas() {
  const canvas = document.getElementById('crop-canvas') as HTMLCanvasElement | null;
  const draft = state.cropDraft;
  if (!canvas || !draft || !draft.sourceImg) return;

  const img = draft.sourceImg;
  const swapped = draft.rotation % 180 !== 0;
  const rawW = swapped ? img.height : img.width;
  const rawH = swapped ? img.width : img.height;

  const maxW = 540;
  const maxH = 360;
  const fit = Math.min(maxW / rawW, maxH / rawH);
  canvas.width = Math.max(180, Math.round(rawW * fit));
  canvas.height = Math.max(140, Math.round(rawH * fit));

  const ctx = canvas.getContext('2d');
  if (!ctx) return;

  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.save();
  ctx.translate(canvas.width / 2, canvas.height / 2);
  ctx.rotate((draft.rotation * Math.PI) / 180);
  ctx.scale((draft.flipX ? -1 : 1) * draft.zoom, (draft.flipY ? -1 : 1) * draft.zoom);
  const dw = img.width * fit;
  const dh = img.height * fit;
  ctx.drawImage(img, -dw / 2, -dh / 2, dw, dh);
  ctx.restore();

  const cx = draft.cropX * canvas.width;
  const cy = draft.cropY * canvas.height;
  const cw = draft.cropW * canvas.width;
  const ch = draft.cropH * canvas.height;

  ctx.fillStyle = 'rgba(20, 20, 18, 0.55)';
  ctx.fillRect(0, 0, canvas.width, cy);
  ctx.fillRect(0, cy + ch, canvas.width, canvas.height - (cy + ch));
  ctx.fillRect(0, cy, cx, ch);
  ctx.fillRect(cx + cw, cy, canvas.width - (cx + cw), ch);

  ctx.strokeStyle = '#ffffff';
  ctx.lineWidth = 2;
  ctx.strokeRect(cx, cy, cw, ch);

  ctx.strokeStyle = 'rgba(255,255,255,0.45)';
  ctx.lineWidth = 1;
  for (const frac of [1 / 3, 2 / 3]) {
    ctx.beginPath();
    ctx.moveTo(cx + cw * frac, cy);
    ctx.lineTo(cx + cw * frac, cy + ch);
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(cx, cy + ch * frac);
    ctx.lineTo(cx + cw, cy + ch * frac);
    ctx.stroke();
  }

  ctx.fillStyle = '#5A5A40';
  ctx.strokeStyle = '#ffffff';
  for (const [hx, hy] of [
    [cx, cy],
    [cx + cw, cy],
    [cx, cy + ch],
    [cx + cw, cy + ch],
  ]) {
    ctx.fillRect(hx - 5, hy - 5, 10, 10);
    ctx.strokeRect(hx - 5, hy - 5, 10, 10);
  }
}

function renderDropdownItems(items: DocEntry[]): string {
  return `
    ${items
      .map(
        (d) =>
          `<button type="button" data-open-doc="${escapeHtml(d.path)}" class="w-full text-left block px-2.5 py-1.5 rounded-lg text-xs hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] cursor-pointer ${
            state.route === 'doc' && state.activeDocPath === d.path
              ? 'bg-[#f5f5f0] dark:bg-[#2d2d2a] font-semibold text-[#5A5A40] dark:text-[#e6e6de]'
              : ''
          }">${escapeHtml(d.title)}</button>`
      )
      .join('')}
    <a href="/internovel-docs.zip" download="internovel-docs.zip" class="mt-1 block border-t border-[#e5e5e0] dark:border-[#2d2d2a] px-2.5 py-1.5 text-xs font-medium text-[#5A5A40] dark:text-[#d0d0c8] hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] rounded-b-lg">Download All Docs (.zip)</a>
  `;
}

function renderImageModalHtml(novel?: Novel): string {
  if (!state.imageModalOpen || !novel) return '';
  const draft = state.cropDraft;

  return `
    <div id="image-modal-backdrop" class="fixed inset-0 z-50 bg-black/50 backdrop-blur-xs flex items-center justify-center p-4">
      <div class="w-full max-w-3xl rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] shadow-2xl overflow-hidden text-xs">
        <div class="flex items-center justify-between px-5 py-3 border-b border-[#e5e5e0] dark:border-[#2d2d2a]">
          <h3 class="font-serif text-base font-bold text-[#5A5A40] dark:text-[#e6e6de]">
            ${draft ? 'Crop, Style & Insert Image' : 'Novel Image Manager & Uploader'}
          </h3>
          <button id="close-image-modal-btn" type="button" class="px-2 py-1 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] cursor-pointer">Close ×</button>
        </div>

        <div class="p-5 space-y-5 max-h-[82vh] overflow-y-auto">
          ${
            draft
              ? `
                <div class="grid gap-5 md:grid-cols-[1fr_240px]">
                  <div class="flex flex-col items-center justify-center rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-3 border border-[#e5e5e0] dark:border-[#2d2d2a]">
                    <canvas id="crop-canvas" class="max-w-full rounded cursor-crosshair touch-none"></canvas>
                    <p class="text-[11px] opacity-65 mt-2">Drag inside box to move crop · Drag bottom-right corner to resize</p>
                  </div>
                  <div class="space-y-3">
                    <label class="block">
                      <span class="font-medium">Caption / Alt Text</span>
                      <input id="crop-alt-input" type="text" value="${escapeHtml(draft.alt)}" placeholder="Describe this scene or portrait..." class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5" />
                    </label>

                    <div>
                      <div class="font-medium mb-1">Aspect Ratio</div>
                      <div class="grid grid-cols-5 gap-1">
                        ${(['free', '16:9', '4:3', '1:1', '2:3'] as const)
                          .map(
                            (ar) => `<button type="button" data-crop-aspect="${ar}" class="px-1.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${
                              draft.aspect === ar ? 'bg-[#5A5A40] text-white' : ''
                            }">${ar}</button>`
                          )
                          .join('')}
                      </div>
                    </div>

                    <div>
                      <div class="flex justify-between font-medium mb-1"><span>Zoom</span><span>${Math.round(draft.zoom * 100)}%</span></div>
                      <input id="crop-zoom-input" type="range" min="1" max="2.5" step="0.05" value="${draft.zoom}" class="w-full" />
                    </div>

                    <div class="flex flex-wrap gap-1.5">
                      <button id="crop-rot-left" type="button" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">↺ 90°</button>
                      <button id="crop-rot-right" type="button" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">↻ 90°</button>
                      <button id="crop-flip-x" type="button" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Flip H</button>
                      <button id="crop-flip-y" type="button" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Flip V</button>
                    </div>

                    <div class="pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] space-y-2">
                      <button id="crop-save-insert-btn" type="button" class="w-full px-3 py-2 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">Save & Insert into Chapter</button>
                      <button id="crop-save-only-btn" type="button" class="w-full px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Save Copy to Gallery</button>
                      <button id="crop-back-btn" type="button" class="w-full px-3 py-1 rounded-lg opacity-70 hover:opacity-100 cursor-pointer">← Back to Gallery Picker</button>
                    </div>
                  </div>
                </div>
              `
              : `
                <div id="modal-dropzone" class="rounded-2xl border-2 border-dashed border-[#5A5A40]/40 bg-[#f5f5f0] dark:bg-[#252522] p-6 text-center space-y-2 transition-colors">
                  <div class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">Drag & drop an image here, paste from clipboard, or browse</div>
                  <p class="opacity-70">Images automatically optimize under 1 MB and can be cropped before inserting into your chapter.</p>
                  <label class="inline-block mt-1 px-4 py-1.5 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">
                    Choose Image File
                    <input id="modal-file-input" type="file" accept="image/*" class="hidden" />
                  </label>
                </div>

                <div class="space-y-2">
                  <div class="font-medium">Novel Gallery (${novel.gallery.length}) — Click Insert to embed at cursor</div>
                  ${
                    novel.gallery.length
                      ? `<div class="grid gap-3 sm:grid-cols-2 md:grid-cols-3">
                          ${novel.gallery
                            .map(
                              (img, idx) => `
                              <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-2.5 space-y-2">
                                <img src="${img.dataUrl}" alt="${escapeHtml(img.title)}" class="w-full h-28 object-cover rounded-lg bg-[#f5f5f0] dark:bg-[#252522]" />
                                <div class="font-medium truncate">${escapeHtml(img.title)}</div>
                                <div class="text-[10px] opacity-65">${img.width}×${img.height} · ${img.sizeKb} KB</div>
                                <div class="flex gap-1">
                                  <button type="button" data-modal-insert-img="${idx}" class="flex-1 px-2 py-1 rounded bg-[#5A5A40] text-white cursor-pointer">Insert</button>
                                  <button type="button" data-modal-crop-img="${idx}" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Crop</button>
                                </div>
                              </div>`
                            )
                            .join('')}
                        </div>`
                      : `<p class="opacity-65 py-2">No images in this novel's gallery yet. Upload one above to insert it into your story.</p>`
                  }
                </div>
              `
          }
        </div>
      </div>
    </div>
  `;
}

// =========================================================================
// REAL FULL-TEXT & MULTI-ENTITY SEARCH ENGINE
// =========================================================================

interface SearchHit {
  uid: string;
  badge: 'Chapter' | 'Character' | 'Theme' | 'Location' | 'Plot Card' | 'Image' | 'Doc';
  title: string;
  subtitle: string;
  excerptRaw: string;
  tags: string[];
  storylineDate: string;
  score: number;
  novelId?: string;
  novelIdx?: number;
  vertebraId?: string;
  favorited?: boolean;
  openAction:
    | { type: 'studio'; novelIdx: number; vertebraId: string; tab: StudioTab }
    | { type: 'casting'; trunkId: string }
    | { type: 'doc'; docPath: string };
}

function tokenizeQuery(raw: string): string[] {
  return raw
    .trim()
    .toLowerCase()
    .split(/\s+/)
    .filter(Boolean);
}

function matchTokensAndScore(terms: string[], primaryTitle: string, secondaryFields: string[], tags: string[]): number {
  if (!terms.length) return 10;
  const titleLower = primaryTitle.toLowerCase();
  const tagsLower = tags.join(' ').toLowerCase();
  const fullBlob = `${titleLower} ${tagsLower} ${secondaryFields.join(' ').toLowerCase()}`;

  for (const term of terms) {
    if (!fullBlob.includes(term)) return 0;
  }

  let score = 10;
  for (const term of terms) {
    if (titleLower === term) score += 60;
    else if (titleLower.includes(term)) score += 35;
    if (tagsLower.includes(term)) score += 25;
  }
  return score;
}

function buildHighlightedSnippet(rawText: string, terms: string[]): string {
  const clean = rawText.replace(/\s+/g, ' ').trim();
  if (!clean) return '';
  let start = 0;
  if (terms.length) {
    const lower = clean.toLowerCase();
    for (const term of terms) {
      const idx = lower.indexOf(term);
      if (idx >= 0) {
        start = Math.max(0, idx - 45);
        break;
      }
    }
  }
  const end = Math.min(clean.length, start + 155);
  let slice = clean.slice(start, end);
  if (start > 0) slice = `…${slice}`;
  if (end < clean.length) slice = `${slice}…`;

  let escaped = escapeHtml(slice);
  for (const term of terms) {
    if (!term) continue;
    const safeRegex = term.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    escaped = escaped.replace(
      new RegExp(`(${safeRegex})`, 'gi'),
      '<mark class="bg-amber-200 dark:bg-amber-800/70 text-inherit px-0.5 rounded">$1</mark>'
    );
  }
  return escaped;
}

function gatherLocalSearchHits(): { hits: SearchHit[]; terms: string[] } {
  const terms = tokenizeQuery(state.searchQuery);
  const cats = state.searchCategories;
  const hits: SearchHit[] = [];

  (state.novels as Novel[]).forEach((novel, nIdx) => {
    if (state.searchNovelScope !== 'all' && novel.id !== state.searchNovelScope) {
      return;
    }

    for (const v of novel.vertebrae) {
      const hasCharTag = v.tags.some((t) => t.startsWith('Character:'));
      const hasLocTag = v.tags.some((t) => t.startsWith('Location:'));
      const hasThemeTag = v.tags.some((t) => !t.startsWith('Character:') && !t.startsWith('Location:'));

      const allowChapter =
        cats.Chapters ||
        (cats.Characters && hasCharTag) ||
        (cats.Locations && hasLocTag) ||
        (cats.Themes && hasThemeTag);

      if (allowChapter) {
        const score = matchTokensAndScore(terms, v.title, [novel.title, novel.synopsis, v.kind, v.content], v.tags);
        if (score > 0) {
          let badge: SearchHit['badge'] = 'Chapter';
          if (!cats.Chapters) {
            if (cats.Characters && hasCharTag) badge = 'Character';
            else if (cats.Locations && hasLocTag) badge = 'Location';
            else if (cats.Themes && hasThemeTag) badge = 'Theme';
          }
          hits.push({
            uid: `v:${novel.id}:${v.id}`,
            badge,
            title: `${novel.title} — ${v.title}`,
            subtitle: `${v.kind} · Storyline: ${v.storylineDate}`,
            excerptRaw: v.content,
            tags: v.tags,
            storylineDate: v.storylineDate,
            score,
            novelId: novel.id,
            novelIdx: nIdx,
            vertebraId: v.id,
            favorited: novel.favorited,
            openAction: { type: 'studio', novelIdx: nIdx, vertebraId: v.id, tab: 'editor' },
          });
        }
      }

      if (cats.PlotBlocks) {
        v.plotBlocks.forEach((block, bIdx) => {
          const score = matchTokensAndScore(terms, block, [novel.title, v.title], v.tags);
          if (score > 0) {
            hits.push({
              uid: `pb:${novel.id}:${v.id}:${bIdx}`,
              badge: 'Plot Card',
              title: block,
              subtitle: `Plot Planner in ${novel.title} · ${v.title} (Beat ${bIdx + 1})`,
              excerptRaw: v.content,
              tags: v.tags,
              storylineDate: v.storylineDate,
              score: score + 5,
              novelId: novel.id,
              novelIdx: nIdx,
              vertebraId: v.id,
              favorited: novel.favorited,
              openAction: { type: 'studio', novelIdx: nIdx, vertebraId: v.id, tab: 'planner' },
            });
          }
        });
      }
    }

    if (cats.Images) {
      novel.gallery.forEach((img) => {
        const score = matchTokensAndScore(terms, img.title, [novel.title, `${img.width}x${img.height}`], ['image', 'gallery']);
        if (score > 0) {
          hits.push({
            uid: `img:${novel.id}:${img.id}`,
            badge: 'Image',
            title: img.title,
            subtitle: `${novel.title} Gallery · ${img.width}×${img.height} (${img.sizeKb} KB)`,
            excerptRaw: `Markdown reference: ![${img.title}](internovel-img://${img.id})`,
            tags: ['Gallery Asset'],
            storylineDate: '9999-12-31',
            score,
            novelId: novel.id,
            novelIdx: nIdx,
            favorited: novel.favorited,
            openAction: { type: 'studio', novelIdx: nIdx, vertebraId: novel.vertebrae[0]?.id ?? '', tab: 'gallery' },
          });
        }
      });
    }
  });

  if (cats.Characters && state.searchNovelScope === 'all') {
    for (const t of state.trunks as CharacterTrunk[]) {
      const score = matchTokensAndScore(terms, t.name, [t.bio, t.sample, t.ownerName, t.proposedTo ?? ''], ['character', 'trunk']);
      if (score > 0) {
        hits.push({
          uid: `tr:${t.id}`,
          badge: 'Character',
          title: t.name,
          subtitle: `Character Travel Trunk by ${t.ownerName}${t.proposedTo ? ` · Proposed to ${t.proposedTo}` : ''}`,
          excerptRaw: `${t.bio} — "${t.sample}"`,
          tags: ['Travel Trunk'],
          storylineDate: '0000-00-00',
          score: score + 10,
          openAction: { type: 'casting', trunkId: t.id },
        });
      }
    }
  }

  if (cats.Docs && state.searchNovelScope === 'all') {
    for (const d of docEntries) {
      const score = matchTokensAndScore(terms, d.title, [d.content], [d.group]);
      if (score > 0) {
        hits.push({
          uid: `doc:${d.path}`,
          badge: 'Doc',
          title: d.title,
          subtitle: d.group === 'user' ? 'User Guide' : 'Documentation',
          excerptRaw: d.content,
          tags: [d.group],
          storylineDate: '0000-00-00',
          score: score - 5,
          openAction: { type: 'doc', docPath: d.path },
        });
      }
    }
  }

  if (state.searchSort === 'alpha') {
    hits.sort((a, b) => a.title.localeCompare(b.title));
  } else if (state.searchSort === 'date') {
    hits.sort((a, b) => b.storylineDate.localeCompare(a.storylineDate));
  } else {
    hits.sort((a, b) => b.score - a.score || a.title.localeCompare(b.title));
  }

  return { hits, terms };
}

function executeNetworkQuery() {
  const terms = tokenizeQuery(state.searchQuery);
  const depth = Math.max(0, Math.min(4, Number(state.gossipDepth) || 0));
  const matches = PEER_NETWORK_CATALOG.filter((item) => {
    if (item.hop > depth) return false;
    const score = matchTokensAndScore(terms, item.title, [item.summary, item.sample, item.peerLabel], item.tags);
    return score > 0;
  });

  state.gossipHits = matches;
  const qLabel = state.searchQuery.trim() || 'all peer catalog';
  state.gossipStatus =
    depth === 0
      ? 'Network Depth is 0 (local-only mode). Increase depth to 1–4 to query nearby collaborators.'
      : `Queried collaborators & relays up to ${depth} hop${depth === 1 ? '' : 's'} for "${qLabel}" — ${matches.length} peer result${matches.length === 1 ? '' : 's'} found.`;
}

function computeSearchHitsHtml(): string {
  const { hits, terms } = gatherLocalSearchHits();
  const networkHits = state.gossipHits as NetworkItem[];

  const localBlock = hits.length
    ? hits
        .map((h) => {
          const excerptHtml = buildHighlightedSnippet(h.excerptRaw, terms);
          const openPayload = escapeHtml(JSON.stringify(h.openAction));
          return `
            <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-3.5 space-y-2">
              <div class="flex flex-wrap items-start justify-between gap-2">
                <div class="space-y-0.5 min-w-0 flex-1">
                  <div class="flex flex-wrap items-center gap-2">
                    <span class="px-2 py-0.5 rounded bg-[#f5f5f0] dark:bg-[#252522] font-medium text-[10px] uppercase tracking-wide text-[#5A5A40] dark:text-[#d0d0c8]">${escapeHtml(h.badge)}</span>
                    <span class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">${escapeHtml(h.title)}</span>
                  </div>
                  <div class="text-[11px] opacity-70">${escapeHtml(h.subtitle)}${h.tags.length ? ` · ${h.tags.map(escapeHtml).join(', ')}` : ''}</div>
                </div>
                <div class="flex items-center gap-1.5 shrink-0">
                  ${
                    h.novelId
                      ? `<button type="button" data-fav-novel="${escapeHtml(h.novelId)}" class="px-2.5 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] text-xs cursor-pointer ${
                          h.favorited ? 'bg-[#5A5A40] text-white' : ''
                        }">${h.favorited ? 'Synced' : 'Favorite'}</button>`
                      : ''
                  }
                  <button type="button" data-search-open='${openPayload}' class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white text-xs font-medium cursor-pointer">Open →</button>
                </div>
              </div>
              ${excerptHtml ? `<p class="text-xs opacity-85 leading-relaxed">${excerptHtml}</p>` : ''}
            </div>
          `;
        })
        .join('')
    : `<p class="text-xs opacity-70 py-3">No matching items in your local workspace for those filters.</p>`;

  const networkBlock = networkHits.length
    ? `
      <div class="pt-4 mt-4 border-t border-[#e5e5e0] dark:border-[#2d2d2a] space-y-2">
        <div class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">Discovered from Collaborator Network (${networkHits.length})</div>
        ${networkHits
          .map((item) => {
            const alreadyImported = (state.importedNetworkIds as string[]).includes(item.id);
            const snippet = buildHighlightedSnippet(`${item.summary} — ${item.sample}`, terms);
            return `
              <div class="rounded-xl border border-[#5A5A40]/30 bg-[#f5f5f0]/60 dark:bg-[#252522]/60 p-3.5 space-y-2">
                <div class="flex flex-wrap items-start justify-between gap-2">
                  <div class="space-y-0.5 flex-1">
                    <div class="flex flex-wrap items-center gap-2">
                      <span class="px-2 py-0.5 rounded bg-[#5A5A40] text-white text-[10px] font-medium">${item.hop} hop${item.hop === 1 ? '' : 's'}</span>
                      <span class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-[10px] uppercase">${item.kind === 'novel' ? 'Peer Novel' : 'Character Trunk'}</span>
                      <span class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">${escapeHtml(item.title)}</span>
                    </div>
                    <div class="text-[11px] opacity-70">Hosted by ${escapeHtml(item.peerLabel)} · ${escapeHtml(item.peerDid)}</div>
                  </div>
                  <button type="button" data-import-network="${escapeHtml(item.id)}" ${alreadyImported ? 'disabled' : ''} class="px-3 py-1 rounded-lg text-xs font-medium ${
                    alreadyImported
                      ? 'border border-[#e5e5e0] dark:border-[#2d2d2a] opacity-60'
                      : 'bg-[#5A5A40] text-white cursor-pointer'
                  }">${alreadyImported ? 'Imported ✓' : item.kind === 'novel' ? 'Sync to Studio' : 'Import to Casting'}</button>
                </div>
                <p class="text-xs opacity-85">${snippet}</p>
              </div>
            `;
          })
          .join('')}
      </div>
    `
    : '';

  return `
    <div id="search-summary" class="text-[11px] opacity-70 pb-1">
      Showing ${hits.length} local match${hits.length === 1 ? '' : 'es'}${state.searchQuery.trim() ? ` for "${escapeHtml(state.searchQuery.trim())}"` : ''}
    </div>
    ${localBlock}
    ${networkBlock}
  `;
}

function bindSearchResultHandlers() {
  const container = document.getElementById('search-results');
  if (!container) return;

  container.querySelectorAll<HTMLElement>('[data-search-open]').forEach((el) => {
    el.addEventListener('click', () => {
      try {
        const action = JSON.parse(el.dataset.searchOpen ?? '{}');
        if (action.type === 'studio') {
          switchToNovel(action.novelIdx ?? 0, {
            keepRoute: true,
            overrideVertebraId: action.vertebraId || undefined,
            overrideTab: action.tab ?? 'editor',
          });
          state.route = 'studio';
          render();
        } else if (action.type === 'casting') {
          state.route = 'casting';
          render();
        } else if (action.type === 'doc') {
          state.activeDocPath = action.docPath;
          state.route = 'doc';
          render();
        }
      } catch {
        // Ignore malformed payloads
      }
    });
  });

  container.querySelectorAll<HTMLElement>('[data-fav-novel]').forEach((el) => {
    el.addEventListener('click', () => {
      const nv = state.novels.find((n: Novel) => n.id === el.dataset.favNovel);
      if (nv) {
        nv.favorited = !nv.favorited;
        persistState();
        container.innerHTML = computeSearchHitsHtml();
        bindSearchResultHandlers();
      }
    });
  });

  container.querySelectorAll<HTMLElement>('[data-import-network]').forEach((el) => {
    el.addEventListener('click', () => {
      const item = PEER_NETWORK_CATALOG.find((x) => x.id === el.dataset.importNetwork);
      if (!item) return;
      if (!(state.importedNetworkIds as string[]).includes(item.id)) {
        state.importedNetworkIds.push(item.id);
      }
      if (item.kind === 'novel') {
        snapshotCurrentNovelLocation();
        const vId = `v-net-${Date.now()}`;
        state.novels.push({
          id: `novel-net-${Date.now()}`,
          title: item.title,
          synopsis: item.summary,
          collabCall: `Synced from ${item.peerLabel}`,
          seekingCollaborators: true,
          forkingAllowed: true,
          forkingLocked: false,
          favorited: true,
          lastVertebraId: vId,
          lastStudioTab: 'editor',
          lastEditorView: 'split',
          lastScrollTop: 0,
          lastRoute: 'studio',
          pitches: [],
          gallery: [],
          vertebrae: [
            {
              id: vId,
              kind: 'Chapter',
              title: 'Synced Chapter 1',
              storylineDate: item.storylineDate,
              lastEditedTs: Date.now(),
              childCount: 1,
              tags: [...item.tags],
              content: `# ${item.title}\n\n${item.sample}`,
              plotBlocks: [item.summary],
            },
          ],
        });
        state.activeNovelIdx = state.novels.length - 1;
        state.selectedVertebraId = vId;
        state.studioTab = 'editor';
        state.editorView = 'split';
        state.statusNotice = `Synced peer novel "${item.title}" into your Studio.`;
      } else {
        state.trunks.push({
          id: `tr-net-${Date.now()}`,
          name: item.title,
          ownerName: item.peerLabel,
          ownerDid: item.peerDid,
          bio: item.summary,
          sample: item.sample,
          proposedTo: null,
        });
        state.statusNotice = `Imported Character Travel Trunk "${item.title}" into Casting Studio.`;
      }
      render();
    });
  });
}

function renderMainContent(): string {
  const novel = getActiveNovel();
  const items = novel ? sortedVertebrae(novel) : [];
  const selected = items.find((v) => v.id === state.selectedVertebraId) ?? items[0];
  if (selected && selected.id !== state.selectedVertebraId) {
    state.selectedVertebraId = selected.id;
  }

  if (state.route === 'doc') {
    const doc = docEntries.find((d) => d.path === state.activeDocPath) ?? docEntries[0];
    return `
      <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 md:p-8 space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2 pb-3 border-b border-[#e5e5e0] dark:border-[#2d2d2a]">
          <span class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">${escapeHtml(doc?.title ?? 'Documentation')}</span>
          <a href="/internovel-docs.zip" download="internovel-docs.zip" class="px-3 py-1.5 rounded-lg bg-[#f5f5f0] dark:bg-[#2d2d2a] text-xs font-medium text-[#5A5A40] dark:text-[#d0d0c8]">Download All Docs (.zip)</a>
        </div>
        <article class="prose prose-stone dark:prose-invert prose-micro max-w-none" style="font-size: ${state.textScale}%">
          ${renderMarkdownHtml(doc?.content ?? '')}
        </article>
      </div>
    `;
  }

  if (state.route === 'auth') {
    const acc = state.activeAccount as Account | null;
    return `
      <div class="grid gap-6 md:grid-cols-2">
        <section class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-3 text-xs">
          <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">Author Identity & Local Wallet</h1>
          <p class="opacity-80">Your stories and keys stay on your device. Create a new local identity or unlock with your recovery phrase.</p>
          <label class="block"><span class="font-medium">Author Display Name</span><input id="auth-name" type="text" value="${escapeHtml(acc?.displayName ?? 'Alex Rivera')}" class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" /></label>
          <label class="block"><span class="font-medium">Anti-Phishing Security Phrase</span><input id="auth-sec" type="text" value="${escapeHtml(acc?.securityText ?? 'amber-lantern-harbor')}" class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" /></label>
          <label class="block"><span class="font-medium">Passphrase</span><input id="auth-pass" type="password" placeholder="Choose a strong passphrase..." class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" /></label>
          <label class="flex items-center gap-2"><input id="auth-mem" type="checkbox" ${acc?.inMemory ? 'checked' : ''} /> <span>Ephemeral In-Memory Session (nothing saved to disk)</span></label>
          <div class="rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-3 space-y-2">
            <div class="font-medium">Visual Pazzle & Shuffled PIN Preview</div>
            <div class="flex flex-wrap gap-1.5">
              ${state.lastPazzle.map((e: string) => `<span class="px-2 py-1 rounded bg-white dark:bg-[#1a1a18] border border-[#e5e5e0] dark:border-[#2d2d2a]">${e}</span>`).join('')}
            </div>
            <div class="flex flex-wrap gap-1.5">
              ${state.shuffledPin.map((d: number) => `<span class="px-2 py-0.5 rounded bg-white dark:bg-[#1a1a18] border border-[#e5e5e0] dark:border-[#2d2d2a] font-mono">${d}</span>`).join('')}
            </div>
          </div>
          <div class="flex flex-wrap gap-2">
            <button id="auth-create-btn" type="button" class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">Create New Identity</button>
            ${
              acc
                ? `<button id="auth-update-btn" type="button" class="px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] font-medium cursor-pointer">Update Active Phrase</button>`
                : ''
            }
            <button id="auth-shuffle-btn" type="button" class="px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Shuffle Pad</button>
          </div>
        </section>

        <section class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-4 text-xs">
          <h2 class="font-serif text-lg font-bold text-[#5A5A40] dark:text-[#e6e6de]">Saved Author Identities & Device Pairing</h2>
          <div class="space-y-2">
            ${
              (state.accounts as Account[]).length
                ? (state.accounts as Account[])
                    .map(
                      (a, idx) => `
                    <div class="rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-3 flex flex-wrap items-center justify-between gap-2 ${
                      acc?.didShort === a.didShort ? 'border border-[#5A5A40]' : ''
                    }">
                      <div class="flex items-center gap-2.5 min-w-0 flex-1">
                        ${renderAvatarBadge(a, 'w-8 h-8 text-xs')}
                        <div class="min-w-0 flex-1">
                          <div class="font-medium">${escapeHtml(a.displayName)} ${acc?.didShort === a.didShort ? '<span class="text-[10px] text-[#5A5A40] dark:text-[#d0d0c8]">(Active)</span>' : ''}</div>
                          <div class="text-[11px] opacity-80">Phrase: <strong>${escapeHtml(a.securityText)}</strong></div>
                          <div class="font-mono text-[10px] opacity-65 truncate">${escapeHtml(a.didShort)}</div>
                        </div>
                      </div>
                      <div class="flex items-center gap-1.5">
                        ${
                          acc?.didShort !== a.didShort
                            ? `<button type="button" data-switch-acc="${idx}" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Use</button>`
                            : ''
                        }
                        ${
                          state.confirmDeleteAccountDid === a.didShort
                            ? `<button type="button" data-del-acc-confirm="${escapeHtml(a.didShort)}" class="px-2 py-1 rounded bg-red-700 text-white cursor-pointer">Confirm Delete</button>
                               <button type="button" data-del-acc-cancel class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>`
                            : `<button type="button" data-del-acc="${escapeHtml(a.didShort)}" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">Delete</button>`
                        }
                      </div>
                    </div>`
                    )
                    .join('')
                : `<p class="opacity-75">No saved identities yet. Create one on the left.</p>`
            }
          </div>
          <label class="block">
            <span class="font-medium">Import Wallet / Transfer Code</span>
            <textarea id="auth-import-code" rows="4" placeholder="Paste encrypted identity transfer code..." class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] p-2 font-mono text-[11px]"></textarea>
          </label>
          <div class="flex flex-wrap gap-2">
            <button id="auth-export-btn" type="button" class="px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Generate Transfer Code</button>
            <button id="auth-import-btn" type="button" class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white cursor-pointer">Import & Unlock</button>
          </div>
        </section>
      </div>
    `;
  }

  if (state.route === 'casting') {
    const novelsList = state.novels as Novel[];
    return `
      <div class="space-y-5">
        <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-3 text-xs">
          <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">Casting Studio & Character Travel Trunks</h1>
          <p class="opacity-80">Showcase portable characters with narrative samples, edit their biographies, or propose cross-story appearances.</p>
          <div class="grid gap-2 md:grid-cols-4 pt-1">
            <input id="trunk-name" type="text" placeholder="Character Name" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" />
            <input id="trunk-bio" type="text" placeholder="Short Character Bio" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" />
            <input id="trunk-sample" type="text" placeholder="Narrative Sample" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" />
            <button id="trunk-add-btn" type="button" class="rounded-lg bg-[#5A5A40] text-white px-3 py-1.5 font-medium cursor-pointer">Publish Travel Trunk</button>
          </div>
        </div>
        <div class="grid gap-4 md:grid-cols-2">
          ${(state.trunks as CharacterTrunk[])
            .map((t) => {
              const isEditing = state.editingTrunkId === t.id;
              const isConfirmDel = state.confirmDeleteTrunkId === t.id;
              if (isEditing) {
                return `
                  <div class="rounded-2xl border border-[#5A5A40] bg-white dark:bg-[#1a1a18] p-5 space-y-2.5 text-xs">
                    <div class="font-medium text-[#5A5A40] dark:text-[#e6e6de]">Edit Character Travel Trunk</div>
                    <input id="edit-trunk-name-${escapeHtml(t.id)}" type="text" value="${escapeHtml(t.name)}" class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5 font-medium" />
                    <input id="edit-trunk-bio-${escapeHtml(t.id)}" type="text" value="${escapeHtml(t.bio)}" placeholder="Character Bio" class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" />
                    <textarea id="edit-trunk-sample-${escapeHtml(t.id)}" rows="2" placeholder="Narrative sample..." class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5">${escapeHtml(t.sample)}</textarea>
                    <div class="flex gap-2">
                      <button type="button" data-save-trunk="${escapeHtml(t.id)}" class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white cursor-pointer">Save Changes</button>
                      <button type="button" data-cancel-trunk-edit class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>
                    </div>
                  </div>
                `;
              }
              return `
                <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-2.5 text-xs">
                  <div class="flex items-start justify-between gap-2">
                    <div>
                      <div class="font-serif text-base font-bold text-[#5A5A40] dark:text-[#e6e6de]">${escapeHtml(t.name)}</div>
                      <div class="text-[11px] opacity-70">by ${escapeHtml(t.ownerName)} · <span class="font-mono text-[10px]">${escapeHtml(t.ownerDid)}</span></div>
                    </div>
                    <div class="flex items-center gap-1">
                      <button type="button" data-edit-trunk="${escapeHtml(t.id)}" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Edit</button>
                      ${
                        isConfirmDel
                          ? `<button type="button" data-del-trunk-confirm="${escapeHtml(t.id)}" class="px-2 py-0.5 rounded bg-red-700 text-white cursor-pointer">Confirm</button>
                             <button type="button" data-del-trunk-cancel class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>`
                          : `<button type="button" data-del-trunk="${escapeHtml(t.id)}" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">Delete</button>`
                      }
                    </div>
                  </div>
                  <p>${escapeHtml(t.bio)}</p>
                  <blockquote class="border-l-2 border-[#5A5A40] pl-3 italic opacity-85">${escapeHtml(t.sample)}</blockquote>
                  <div class="flex flex-wrap items-center justify-between gap-2 pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a]">
                    ${
                      novelsList.length
                        ? `<div class="flex flex-wrap items-center gap-1.5">
                            <select id="propose-target-${escapeHtml(t.id)}" class="rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1 text-[11px]">
                              ${novelsList.map((nv) => `<option value="${escapeHtml(nv.title)}" ${novel?.id === nv.id ? 'selected' : ''}>${escapeHtml(nv.title)}</option>`).join('')}
                            </select>
                            <button type="button" data-propose-trunk="${escapeHtml(t.id)}" class="px-2.5 py-1 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">
                              ${t.proposedTo ? 'Re-Propose' : 'Propose for Novel'}
                            </button>
                            ${
                              t.proposedTo
                                ? `<button type="button" data-withdraw-trunk="${escapeHtml(t.id)}" class="px-2 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Withdraw (${escapeHtml(t.proposedTo)})</button>`
                                : ''
                            }
                          </div>`
                        : `<span class="opacity-65">Create a novel to propose this character.</span>`
                    }
                  </div>
                </div>
              `;
            })
            .join('')}
        </div>
      </div>
    `;
  }

  if (state.route === 'cafe') {
    const filteredNovels = (state.novels as Novel[]).filter((n) => {
      if (state.cafeFilter === 'collab') return n.seekingCollaborators;
      if (state.cafeFilter === 'forkable') return n.forkingAllowed;
      if (state.cafeFilter === 'remixes') return Boolean(n.remixedFromId);
      return true;
    });

    const filterBtn = (key: CafeFilter, label: string) =>
      `<button type="button" data-cafe-filter="${key}" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${
        state.cafeFilter === key ? 'bg-[#5A5A40] text-white' : ''
      }">${label}</button>`;

    return `
      <div class="space-y-5 text-xs">
        <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-3">
          <div class="flex flex-wrap items-center justify-between gap-2">
            <div>
              <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">Novel Café</h1>
              <p class="opacity-80">Discover featured novels, edit collaborator invitations, post pitches, and inspect live remix lineage diffs.</p>
            </div>
            <div class="flex flex-wrap gap-1">
              ${filterBtn('all', `All (${state.novels.length})`)}
              ${filterBtn('collab', 'Open Co-Author Calls')}
              ${filterBtn('forkable', 'Forkable')}
              ${filterBtn('remixes', 'Remixes')}
            </div>
          </div>
        </div>
        <div class="grid gap-4 md:grid-cols-2">
          ${
            filteredNovels.length
              ? filteredNovels
                  .map((n: Novel) => {
                    const isEditing = state.editingCafeNovelId === n.id;
                    const isConfirmDel = state.confirmDeleteCafeNovelId === n.id;
                    const origText = n.remixedBaselineText ?? n.vertebrae[0]?.content ?? '';
                    const curText = n.vertebrae[0]?.content ?? '';
                    const hasRealDiff = origText && curText && origText !== curText;

                    return `
                      <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-3">
                        ${
                          isEditing
                            ? `
                              <div class="space-y-2 border-b border-[#e5e5e0] dark:border-[#2d2d2a] pb-3">
                                <input id="cafe-edit-title-${escapeHtml(n.id)}" type="text" value="${escapeHtml(n.title)}" class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5 font-serif font-bold" />
                                <textarea id="cafe-edit-syn-${escapeHtml(n.id)}" rows="2" placeholder="Synopsis..." class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5">${escapeHtml(n.synopsis)}</textarea>
                                <input id="cafe-edit-call-${escapeHtml(n.id)}" type="text" value="${escapeHtml(n.collabCall)}" placeholder="Collaborator call note..." class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5" />
                                <label class="inline-flex items-center gap-1.5 cursor-pointer">
                                  <input id="cafe-edit-seeking-${escapeHtml(n.id)}" type="checkbox" ${n.seekingCollaborators ? 'checked' : ''} />
                                  <span>Seeking Collaborators</span>
                                </label>
                                <div class="flex gap-2">
                                  <button type="button" data-save-cafe-novel="${escapeHtml(n.id)}" class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white cursor-pointer">Save</button>
                                  <button type="button" data-cancel-cafe-edit class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>
                                </div>
                              </div>
                            `
                            : `
                              <div class="flex items-start justify-between gap-2">
                                <div>
                                  <div class="font-serif text-base font-bold text-[#5A5A40] dark:text-[#e6e6de]">${escapeHtml(n.title)}</div>
                                  <div class="text-[11px] opacity-75">
                                    ${
                                      n.forkingAllowed
                                        ? n.forkingLocked
                                          ? 'Forking Enabled (Locked)'
                                          : 'Forking Enabled'
                                        : 'Forking Disabled'
                                    }${n.seekingCollaborators ? ' · Seeking Collaborators' : ''}${n.remixedFromTitle ? ` · Remix of ${escapeHtml(n.remixedFromTitle)}` : ''}
                                  </div>
                                </div>
                                <div class="flex items-center gap-1">
                                  <button type="button" data-edit-cafe-novel="${escapeHtml(n.id)}" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Edit</button>
                                  ${
                                    isConfirmDel
                                      ? `<button type="button" data-del-cafe-confirm="${escapeHtml(n.id)}" class="px-2 py-0.5 rounded bg-red-700 text-white cursor-pointer">Confirm</button>
                                         <button type="button" data-del-cafe-cancel class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>`
                                      : `<button type="button" data-del-cafe-novel="${escapeHtml(n.id)}" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">Delete</button>`
                                  }
                                </div>
                              </div>
                              <p class="opacity-85">${escapeHtml(n.synopsis)}</p>
                              ${
                                n.seekingCollaborators && n.collabCall
                                  ? `<div class="rounded-lg bg-[#f5f5f0] dark:bg-[#252522] px-3 py-2 text-[11px]"><strong>Open Call:</strong> ${escapeHtml(n.collabCall)}</div>`
                                  : ''
                              }
                            `
                        }

                        <div class="flex flex-wrap gap-2 pt-1">
                          <button type="button" data-open-studio-novel="${escapeHtml(n.id)}" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Open in Studio</button>
                          ${
                            n.forkingAllowed
                              ? `<button type="button" data-fork-novel="${escapeHtml(n.id)}" class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white cursor-pointer">Fork / Remix Novel</button>`
                              : ''
                          }
                          <button type="button" data-toggle-remix="${escapeHtml(n.id)}" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Inspect Remix Lineage Diff</button>
                        </div>

                        ${
                          state.remixDiffOpenFor === n.id
                            ? `<div class="mt-2 rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] p-3 space-y-1.5">
                                <div class="font-semibold">Remix Lineage Comparison</div>
                                ${
                                  hasRealDiff
                                    ? `<p class="text-red-700 dark:text-red-400 line-through">- Original (${escapeHtml(n.remixedFromTitle ?? 'Source')}): ${escapeHtml(origText.slice(0, 180))}</p>
                                       <p class="text-emerald-700 dark:text-emerald-400">+ Remixed (${escapeHtml(n.title)}): ${escapeHtml(curText.slice(0, 180))}</p>`
                                    : `<p class="opacity-75">No prose divergence yet from baseline. Fork or edit Chapter 1 in Studio to inspect live differences.</p>`
                                }
                              </div>`
                            : ''
                        }

                        <div class="pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] space-y-2">
                          <div class="font-medium opacity-80">Collaboration Pitches (${n.pitches.length})</div>
                          ${n.pitches
                            .map(
                              (p, pIdx) => `
                              <div class="flex items-start justify-between gap-2 rounded-lg bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5">
                                <div><span class="font-medium">${escapeHtml(p.author)}:</span> ${escapeHtml(p.text)} <span class="opacity-55 text-[10px]">(${escapeHtml(p.date)})</span></div>
                                <button type="button" data-del-pitch="${escapeHtml(n.id)}:${pIdx}" class="text-red-700 dark:text-red-400 cursor-pointer" title="Delete pitch">×</button>
                              </div>`
                            )
                            .join('')}
                          <div class="flex gap-1.5">
                            <input id="cafe-pitch-input-${escapeHtml(n.id)}" type="text" placeholder="Offer a co-author pitch or note..." class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1" />
                            <button type="button" data-add-pitch="${escapeHtml(n.id)}" class="px-2.5 py-1 rounded-lg bg-[#5A5A40] text-white cursor-pointer">Post Pitch</button>
                          </div>
                        </div>
                      </div>
                    `;
                  })
                  .join('')
              : `<div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 opacity-75">No novels match the current filter.</div>`
          }
        </div>
      </div>
    `;
  }

  if (state.route === 'search') {
    const catEntries: [keyof typeof defaultState.searchCategories, string][] = [
      ['Chapters', 'Chapters'],
      ['Characters', 'Characters'],
      ['PlotBlocks', 'Plot Cards'],
      ['Themes', 'Themes & Tags'],
      ['Locations', 'Locations'],
      ['Images', 'Images'],
      ['Docs', 'Docs & Guides'],
    ];

    return `
      <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-4 text-xs">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div>
            <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">Search & Discovery</h1>
            <p class="opacity-75">Search across chapters, characters, plot planner cards, gallery images, docs, or query collaborators nearby.</p>
          </div>
          ${
            state.searchQuery.trim()
              ? `<button id="search-clear-btn" type="button" class="px-2.5 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Clear Search</button>`
              : ''
          }
        </div>

        <div class="flex flex-wrap gap-2">
          <input id="search-input" type="text" value="${escapeHtml(state.searchQuery)}" placeholder="Search words, character names, locations, plot cards, or docs..." class="flex-1 min-w-[220px] rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" />
          <select id="search-novel-scope" aria-label="Filter Novel" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5">
            <option value="all" ${state.searchNovelScope === 'all' ? 'selected' : ''}>All Novels & Community</option>
            ${(state.novels as Novel[])
              .map((n) => `<option value="${escapeHtml(n.id)}" ${state.searchNovelScope === n.id ? 'selected' : ''}>${escapeHtml(n.title)}</option>`)
              .join('')}
          </select>
          <select id="search-sort-select" aria-label="Sort Results" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5">
            <option value="relevance" ${state.searchSort === 'relevance' ? 'selected' : ''}>Sort: Best Match</option>
            <option value="date" ${state.searchSort === 'date' ? 'selected' : ''}>Sort: Storyline Date</option>
            <option value="alpha" ${state.searchSort === 'alpha' ? 'selected' : ''}>Sort: Alphabetical</option>
          </select>
          <label class="flex items-center gap-1.5"><span>Network Depth:</span><input id="gossip-depth" type="number" min="0" max="4" value="${state.gossipDepth}" class="w-14 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1" /></label>
          <button id="gossip-send-btn" type="button" class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">Send Network Query</button>
        </div>

        <div class="flex flex-wrap items-center gap-3">
          ${catEntries
            .map(
              ([key, label]) => `
              <label class="inline-flex items-center gap-1.5 cursor-pointer">
                <input type="checkbox" data-search-cat="${key}" ${state.searchCategories[key] ? 'checked' : ''} />
                <span>${label}</span>
              </label>`
            )
            .join('')}
          <button id="search-cats-all" type="button" class="text-[11px] underline opacity-75 hover:opacity-100 cursor-pointer">All</button>
          <button id="search-cats-none" type="button" class="text-[11px] underline opacity-75 hover:opacity-100 cursor-pointer">None</button>
        </div>

        ${state.gossipStatus ? `<p class="rounded-lg bg-[#f5f5f0] dark:bg-[#252522] px-3 py-2 text-[#5A5A40] dark:text-[#d0d0c8] font-medium">${escapeHtml(state.gossipStatus)}</p>` : ''}
        <div id="search-results" class="space-y-2">${computeSearchHitsHtml()}</div>
      </div>
    `;
  }

  if (state.route === 'profile') {
    const acc = state.activeAccount as Account | null;
    if (!acc) {
      return `
        <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-3 text-xs max-w-xl">
          <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">Author Profile</h1>
          <p class="opacity-80">No active author identity selected. Create or unlock an identity to customize your profile.</p>
          <button type="button" data-nav="auth" class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">Go to Identity & Wallet →</button>
        </div>
      `;
    }
    return `
      <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-6 space-y-4 text-xs max-w-2xl">
        <div class="flex items-center justify-between gap-4">
          <div class="flex items-center gap-4">
            ${
              acc.avatarDataUrl
                ? `<img src="${acc.avatarDataUrl}" alt="Avatar" class="w-14 h-14 rounded-full object-cover border border-[#e5e5e0]" />`
                : `<div class="w-14 h-14 rounded-full bg-[#5A5A40] text-white flex items-center justify-center font-serif text-lg font-bold">${escapeHtml(
                    acc.avatarInitials
                  )}</div>`
            }
            <div>
              <h1 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">Author Profile</h1>
              <div class="font-mono text-[11px] opacity-70 break-all">${escapeHtml(acc.didShort)}</div>
            </div>
          </div>
          ${
            acc.avatarDataUrl
              ? `<button id="profile-clear-avatar-btn" type="button" class="px-2.5 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Remove Avatar</button>`
              : ''
          }
        </div>
        <label class="block"><span class="font-medium">Display Name</span><input id="profile-name" type="text" value="${escapeHtml(acc.displayName)}" class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" /></label>
        <label class="block"><span class="font-medium">Author Bio</span><textarea id="profile-bio" rows="3" class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5">${escapeHtml(acc.bio)}</textarea></label>
        <label class="block"><span class="font-medium">Security Phrase</span><input id="profile-sec" type="text" value="${escapeHtml(acc.securityText)}" class="mt-1 w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" /></label>
        <label class="block"><span class="font-medium">Upload Avatar</span><input id="profile-avatar-file" type="file" accept="image/*" class="mt-1 block text-xs" /></label>
        <div class="flex flex-wrap items-center justify-between gap-2 pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a]">
          <div class="flex flex-wrap gap-2">
            <button id="profile-save-btn" type="button" class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">Save Profile</button>
            <button id="profile-export-btn" type="button" class="px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Export Backup (.json)</button>
          </div>
          ${
            state.confirmDeleteAccountDid === acc.didShort
              ? `<div class="flex gap-1.5">
                  <button type="button" data-del-acc-confirm="${escapeHtml(acc.didShort)}" class="px-2.5 py-1.5 rounded-lg bg-red-700 text-white cursor-pointer">Confirm Delete Identity</button>
                  <button type="button" data-del-acc-cancel class="px-2.5 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>
                </div>`
              : `<button type="button" data-del-acc="${escapeHtml(acc.didShort)}" class="px-2.5 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">Delete Identity</button>`
          }
        </div>
      </div>
    `;
  }

  // Empty novels state
  if (!novel) {
    return `
      <div class="rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-8 text-center space-y-3 text-xs max-w-lg mx-auto my-8">
        <h2 class="font-serif text-xl font-bold text-[#5A5A40] dark:text-[#e6e6de]">No Novels in Workspace</h2>
        <p class="opacity-75">Create a new novel to begin drafting chapters, plot cards, and artwork.</p>
        <button id="empty-create-novel-btn" type="button" class="px-4 py-2 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">+ Create First Novel</button>
      </div>
    `;
  }

  // Studio Workspace: Collapsible Left-Nav Novel Spine + Stateful Sliding Split + Complete CRUD
  let tabBody = '';
  if (state.studioTab === 'editor') {
    const content = selected?.content ?? '';
    tabBody = `
      <div class="space-y-3">
        <div class="flex flex-wrap items-center justify-between gap-2 pb-2 border-b border-[#e5e5e0] dark:border-[#2d2d2a]">
          <div class="flex flex-wrap items-center gap-2 text-xs">
            <input id="vertebra-title-input" type="text" value="${escapeHtml(selected?.title ?? '')}" class="font-serif font-bold text-sm rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1" />
            <select id="vertebra-kind-select" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1">
              ${(['Idea', 'Plot Point', 'Theme', 'Chapter'] as const)
                .map((k) => `<option value="${k}" ${selected?.kind === k ? 'selected' : ''}>${k}</option>`)
                .join('')}
            </select>
            <input id="vertebra-date-input" type="date" value="${escapeHtml(selected?.storylineDate ?? '2026-01-01')}" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1" />
          </div>
          <div class="flex items-center gap-2 text-xs">
            <span id="word-count" class="text-[11px] opacity-70">${countWords(content)} words · Saved locally</span>
            ${
              selected && novel.vertebrae.length > 1
                ? state.confirmDeleteVertebraId === selected.id
                  ? `<button id="confirm-del-vertebra-btn" type="button" class="px-2 py-1 rounded bg-red-700 text-white cursor-pointer">Confirm</button>
                     <button id="cancel-del-vertebra-btn" type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>`
                  : `<button id="del-vertebra-btn" type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">Delete Chapter</button>`
                : ''
            }
          </div>
        </div>

        <div class="flex flex-wrap items-center gap-1.5 text-xs">
          ${(selected?.tags ?? [])
            .map(
              (tag: string, tIdx: number) => `
              <span class="inline-flex items-center gap-1 rounded-full bg-[#f5f5f0] dark:bg-[#252522] border border-[#e5e5e0] dark:border-[#2d2d2a] px-2.5 py-0.5 text-[11px]">
                ${escapeHtml(tag)}
                <button type="button" data-rm-tag="${tIdx}" class="opacity-65 hover:opacity-100 cursor-pointer" title="Remove tag">×</button>
              </span>`
            )
            .join('')}
          <input id="new-tag-input" type="text" placeholder="Add tag (e.g. Character:Lyra)..." class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-0.5 text-[11px]" />
          <button id="new-tag-btn" type="button" class="px-2 py-0.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] text-[11px] cursor-pointer">+ Tag</button>
        </div>

        <div class="flex flex-wrap items-center justify-between gap-2">
          <div class="flex flex-wrap gap-1 text-xs">
            <button type="button" data-fmt="# Heading\n" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">H1</button>
            <button type="button" data-fmt="## Section\n" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">H2</button>
            <button type="button" data-fmt="**bold**" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Bold</button>
            <button type="button" data-fmt="*italic*" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Italic</button>
            <button type="button" data-fmt="> Quote\n" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Quote</button>
            <button type="button" data-fmt="- Item\n" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">List</button>
            <button id="open-image-modal-btn" type="button" class="px-2.5 py-1 rounded border border-[#5A5A40]/40 bg-[#f5f5f0] dark:bg-[#252522] text-[#5A5A40] dark:text-[#e6e6de] font-medium cursor-pointer">+ Image</button>
          </div>

          <div class="flex items-center gap-2 text-xs">
            ${
              state.editorView === 'split'
                ? `<span id="split-badge" class="hidden md:inline text-[11px] opacity-65 font-mono">${Math.round(state.splitRatio)}% / ${Math.round(100 - state.splitRatio)}%</span>`
                : ''
            }
            <div class="flex gap-1">
              <button type="button" data-eview="edit" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${state.editorView === 'edit' ? 'bg-[#5A5A40] text-white' : ''}">Edit</button>
              <button type="button" data-eview="split" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${state.editorView === 'split' ? 'bg-[#5A5A40] text-white' : ''}">Split</button>
              <button type="button" data-eview="preview" class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${state.editorView === 'preview' ? 'bg-[#5A5A40] text-white' : ''}">Preview</button>
            </div>
          </div>
        </div>

        <div id="split-container" class="${state.editorView === 'split' ? 'flex flex-col md:flex-row gap-3 md:gap-0 items-stretch min-h-[420px]' : 'block'}">
          ${
            state.editorView !== 'preview'
              ? `<div id="editor-pane" class="relative flex flex-col min-w-0 ${state.editorView === 'split' ? 'w-full md:shrink-0' : 'w-full'}" ${
                  state.editorView === 'split' ? `style="width: ${state.splitRatio}%"` : ''
                }>
                  <textarea id="editor-textarea" rows="18" class="w-full flex-1 min-h-[400px] rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] p-3.5 font-serif text-sm leading-relaxed resize-y">${escapeHtml(content)}</textarea>
                  <div id="editor-drop-overlay" class="hidden pointer-events-none absolute inset-0 rounded-xl border-2 border-dashed border-[#5A5A40] bg-white/85 dark:bg-[#1a1a18]/85 flex items-center justify-center text-xs font-medium text-[#5A5A40] dark:text-[#e6e6de]">Drop image to crop & insert into chapter</div>
                </div>`
              : ''
          }
          ${
            state.editorView === 'split'
              ? `<div id="split-divider" title="Drag to resize · Double-click to reset 50/50" class="hidden md:flex w-3.5 shrink-0 cursor-col-resize items-center justify-center group select-none">
                  <div class="w-1 h-12 rounded-full bg-[#e5e5e0] dark:bg-[#3a3a35] group-hover:bg-[#5A5A40] transition-colors"></div>
                </div>`
              : ''
          }
          ${
            state.editorView !== 'edit'
              ? `<article id="editor-preview" class="flex-1 min-w-0 prose prose-stone dark:prose-invert prose-micro max-w-none rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-4 min-h-[400px] overflow-y-auto" style="font-size: ${state.textScale}%">${renderMarkdownHtml(content, resolveNovelImage)}</article>`
              : ''
          }
        </div>
      </div>
    `;
  } else if (state.studioTab === 'planner') {
    tabBody = `
      <div class="space-y-3 text-xs">
        <p class="opacity-75">Block-based Plot Planner attached to <strong>${escapeHtml(selected?.title ?? '')}</strong>.</p>
        <div class="space-y-2">
          ${(selected?.plotBlocks ?? [])
            .map((b: string, idx: number) =>
              state.editingBlockIdx === idx
                ? `
                  <div class="rounded-xl border border-[#5A5A40] bg-[#f5f5f0] dark:bg-[#252522] p-3 flex items-center gap-2">
                    <input id="block-edit-input" type="text" value="${escapeHtml(b)}" class="flex-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-2.5 py-1" />
                    <button type="button" data-block-save="${idx}" class="px-2.5 py-1 rounded bg-[#5A5A40] text-white cursor-pointer">Save</button>
                    <button type="button" data-block-cancel class="px-2.5 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>
                  </div>`
                : `
                  <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] p-3 flex items-center justify-between gap-2">
                    <span class="flex-1">${idx + 1}. ${escapeHtml(b)}</span>
                    <div class="flex gap-1">
                      <button type="button" data-block-edit="${idx}" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Edit</button>
                      <button type="button" data-block-up="${idx}" class="px-1.5 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">↑</button>
                      <button type="button" data-block-dn="${idx}" class="px-1.5 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">↓</button>
                      <button type="button" data-block-del="${idx}" class="px-1.5 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">×</button>
                    </div>
                  </div>`
            )
            .join('')}
        </div>
        <div class="flex gap-2">
          <input id="plot-block-input" type="text" placeholder="Add beat or plot card..." class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" />
          <button id="plot-block-add" type="button" class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white cursor-pointer">Add Block</button>
        </div>
      </div>
    `;
  } else if (state.studioTab === 'contacts') {
    tabBody = `
      <div class="space-y-4 text-xs">
        <p class="opacity-80">Permissions can only be shared with collaborators in your Contacts subgroups. Removing someone immediately revokes their access.</p>
        <div class="flex gap-2">
          <input id="subgroup-new-input" type="text" placeholder="Create new subgroup (e.g. Illustrators)..." class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" />
          <button id="subgroup-new-btn" type="button" class="px-3 py-1.5 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">+ Subgroup</button>
        </div>
        <div class="space-y-2">
          ${(state.subgroups as ContactSubgroup[])
            .map(
              (grp, gIdx) => `
              <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-3 space-y-2">
                <div class="flex items-center justify-between gap-2">
                  ${
                    state.editingSubgroupIdx === gIdx
                      ? `<div class="flex flex-1 gap-1.5">
                          <input id="subgroup-rename-input" type="text" value="${escapeHtml(grp.name)}" class="flex-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-0.5" />
                          <button type="button" data-save-subgroup="${gIdx}" class="px-2 py-0.5 rounded bg-[#5A5A40] text-white cursor-pointer">Save</button>
                          <button type="button" data-cancel-subgroup class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Cancel</button>
                        </div>`
                      : `<span class="font-medium">${escapeHtml(grp.name)} (${grp.members.length})</span>
                         <div class="flex gap-1.5">
                           <button type="button" data-edit-subgroup="${gIdx}" class="text-[11px] px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Rename</button>
                           <button type="button" data-del-subgroup="${gIdx}" class="text-[11px] px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">Delete Group</button>
                         </div>`
                  }
                </div>
                <div class="flex flex-wrap gap-1.5">
                  ${
                    grp.members.length
                      ? grp.members
                          .map(
                            (m: string, mIdx: number) => `
                            <span class="inline-flex items-center gap-1 rounded bg-[#f5f5f0] dark:bg-[#252522] px-2 py-0.5 font-mono text-[11px]">
                              ${escapeHtml(m)}
                              <button type="button" data-rm-contact="${gIdx}:${mIdx}" class="text-red-700 dark:text-red-400 cursor-pointer" title="Remove & revoke">×</button>
                            </span>`
                          )
                          .join('')
                      : '<span class="opacity-60">No members in this subgroup yet</span>'
                  }
                </div>
              </div>`
            )
            .join('')}
        </div>
        <div class="flex flex-wrap gap-2">
          <input id="contact-did-input" type="text" placeholder="Collaborator DID (e.g. did:peer:4zQm...)" class="flex-1 min-w-[200px] rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1.5" />
          <select id="contact-group-select" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1.5">
            ${(state.subgroups as ContactSubgroup[]).map((g, i) => `<option value="${i}">${escapeHtml(g.name)}</option>`).join('')}
          </select>
          <button id="contact-add-btn" type="button" class="px-3 py-1.5 rounded-lg bg-[#5A5A40] text-white cursor-pointer">Add Collaborator</button>
        </div>
      </div>
    `;
  } else {
    tabBody = `
      <div class="space-y-4 text-xs">
        <div id="gallery-dropzone" class="rounded-2xl border-2 border-dashed border-[#5A5A40]/40 bg-[#f5f5f0] dark:bg-[#252522] p-6 text-center space-y-2">
          <div class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">Drag & drop artwork or portraits here to upload & crop</div>
          <p class="opacity-75">Images automatically compress under 1 MB. Edited copies save as brand-new gallery items, and one click inserts them into any chapter.</p>
          <label class="inline-block mt-1 px-4 py-1.5 rounded-lg bg-[#5A5A40] text-white font-medium cursor-pointer">
            Upload Image
            <input id="gallery-upload-input" type="file" accept="image/*" class="hidden" />
          </label>
        </div>
        ${
          novel.gallery.length
            ? `<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                ${novel.gallery
                  .map(
                    (img: GalleryImage, idx: number) => `
                    <div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] p-3 space-y-2">
                      <div class="overflow-hidden rounded-lg bg-[#f5f5f0] dark:bg-[#252522] flex items-center justify-center h-44">
                        <img src="${img.dataUrl}" alt="${escapeHtml(img.title)}" class="max-h-40 object-contain" />
                      </div>
                      ${
                        state.editingGalleryIdx === idx
                          ? `<div class="flex gap-1">
                              <input id="img-rename-input" type="text" value="${escapeHtml(img.title)}" class="flex-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-0.5 text-[11px]" />
                              <button type="button" data-img-save-title="${idx}" class="px-2 py-0.5 rounded bg-[#5A5A40] text-white cursor-pointer">Save</button>
                            </div>`
                          : `<div class="flex items-center justify-between gap-1">
                              <div class="font-medium truncate">${escapeHtml(img.title)}</div>
                              <button type="button" data-img-edit-title="${idx}" class="text-[11px] opacity-70 hover:opacity-100 cursor-pointer">Rename</button>
                            </div>`
                      }
                      <div class="text-[11px] opacity-65">${img.width}×${img.height} · ${img.sizeKb} KB</div>
                      <div class="flex flex-wrap gap-1.5 pt-1">
                        <button type="button" data-img-insert="${idx}" class="px-2.5 py-1 rounded bg-[#5A5A40] text-white font-medium cursor-pointer">Insert into Chapter</button>
                        <button type="button" data-img-crop="${idx}" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Crop / Edit Copy</button>
                        <button type="button" data-img-del="${idx}" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">Delete</button>
                      </div>
                    </div>`
                  )
                  .join('')}
              </div>`
            : `<div class="rounded-xl border border-dashed border-[#e5e5e0] dark:border-[#2d2d2a] p-6 text-center opacity-70">No images uploaded to this novel yet.</div>`
        }
      </div>
    `;
  }

  return `
    <div class="flex flex-col lg:flex-row gap-4 items-start">
      ${
        state.spineCollapsed
          ? `<aside class="w-full lg:w-11 shrink-0 rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-2 flex lg:flex-col items-center justify-between lg:justify-start gap-3">
              <button id="toggle-spine-collapse-btn" type="button" title="Expand Novel Spine" class="w-7 h-7 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] flex items-center justify-center text-xs font-bold hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] cursor-pointer">»</button>
              <span class="font-serif text-[11px] font-bold tracking-widest uppercase text-[#5A5A40] dark:text-[#d0d0c8] lg:[writing-mode:vertical-rl] lg:rotate-180 select-none">Novel Spine (${items.length})</span>
            </aside>`
          : `<aside class="w-full lg:w-68 shrink-0 rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-3 space-y-3">
              <div class="flex items-center justify-between gap-1.5">
                <h2 class="font-serif text-sm font-bold text-[#5A5A40] dark:text-[#e6e6de]">${state.spineView ? 'Novel Spine' : 'Flat List'}</h2>
                <div class="flex items-center gap-1">
                  <button id="toggle-spine-btn" type="button" class="text-[11px] px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">${state.spineView ? 'Flat' : 'Spine'}</button>
                  <button id="toggle-spine-collapse-btn" type="button" title="Collapse Novel Spine" class="text-[11px] px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">«</button>
                </div>
              </div>
              <select id="spine-sort-select" class="w-full rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1 text-xs">
                <option value="date" ${state.spineSort === 'date' ? 'selected' : ''}>Sort: Storyline Date</option>
                <option value="alpha" ${state.spineSort === 'alpha' ? 'selected' : ''}>Sort: Alphabetical</option>
                <option value="edited" ${state.spineSort === 'edited' ? 'selected' : ''}>Sort: Last Edited</option>
                <option value="busy" ${state.spineSort === 'busy' ? 'selected' : ''}>Sort: Busyness</option>
              </select>
              <div class="space-y-1">
                ${items
                  .map(
                    (item) => `<button type="button" data-vertebra="${escapeHtml(item.id)}" class="w-full text-left rounded-lg px-2.5 py-2 text-xs cursor-pointer ${
                      item.id === state.selectedVertebraId
                        ? 'bg-[#5A5A40] text-white'
                        : 'hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]'
                    }">${state.spineView ? `<div class="text-[10px] uppercase tracking-wide opacity-75">${escapeHtml(item.kind)} · ${escapeHtml(item.storylineDate)}</div>` : ''}<div class="font-medium">${escapeHtml(item.title)}</div></button>`
                  )
                  .join('')}
              </div>
              <div class="flex gap-1 pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a]">
                <input id="new-vertebra-input" type="text" placeholder="Add Vertebra / Chapter..." class="flex-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1 text-xs" />
                <button id="new-vertebra-btn" type="button" class="px-2.5 py-1 rounded-lg bg-[#5A5A40] text-white text-xs cursor-pointer">+</button>
              </div>
              <div class="pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] space-y-2 text-[11px]">
                <div class="flex items-center justify-between gap-1">
                  <label class="inline-flex items-center gap-1.5 ${novel.forkingLocked ? 'opacity-60' : 'cursor-pointer'}">
                    <input id="novel-fork-toggle" type="checkbox" ${novel.forkingAllowed ? 'checked' : ''} ${novel.forkingLocked ? 'disabled' : ''} />
                    <span>${novel.forkingLocked ? 'Forking Locked' : 'Allow Forking'}</span>
                  </label>
                  <button id="toggle-edit-novel-btn" type="button" class="px-2 py-0.5 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">
                    ${state.editingNovelOpen ? 'Done' : 'Novel Details'}
                  </button>
                </div>
                ${
                  state.editingNovelOpen
                    ? `<div class="rounded-xl bg-[#f5f5f0] dark:bg-[#252522] p-2.5 space-y-2">
                        <input id="studio-novel-title" type="text" value="${escapeHtml(novel.title)}" class="w-full rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-2 py-1 text-xs font-medium" />
                        <textarea id="studio-novel-syn" rows="2" placeholder="Novel synopsis..." class="w-full rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-2 py-1 text-xs">${escapeHtml(novel.synopsis)}</textarea>
                        <div class="flex items-center justify-between gap-1">
                          <button id="studio-save-novel-btn" type="button" class="px-2.5 py-1 rounded bg-[#5A5A40] text-white cursor-pointer">Save</button>
                          ${
                            state.confirmDeleteNovelId === novel.id
                              ? `<button id="studio-confirm-del-novel" type="button" class="px-2 py-1 rounded bg-red-700 text-white cursor-pointer">Confirm Delete</button>`
                              : `<button id="studio-del-novel-btn" type="button" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-red-700 dark:text-red-400 cursor-pointer">Delete Novel</button>`
                          }
                        </div>
                      </div>`
                    : ''
                }
              </div>
            </aside>`
      }

      <section class="flex-1 min-w-0 w-full rounded-2xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] p-5 space-y-4">
        <div class="flex flex-wrap items-center justify-between gap-2 border-b border-[#e5e5e0] dark:border-[#2d2d2a] pb-3">
          <div class="flex flex-wrap gap-1 text-xs">
            <button type="button" data-stab="editor" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${state.studioTab === 'editor' ? 'bg-[#5A5A40] text-white' : ''}">Novel Editor</button>
            <button type="button" data-stab="planner" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${state.studioTab === 'planner' ? 'bg-[#5A5A40] text-white' : ''}">Plot Planner</button>
            <button type="button" data-stab="contacts" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${state.studioTab === 'contacts' ? 'bg-[#5A5A40] text-white' : ''}">Contacts & Collaborators</button>
            <button type="button" data-stab="gallery" class="px-3 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${state.studioTab === 'gallery' ? 'bg-[#5A5A40] text-white' : ''}">Image Gallery (${novel.gallery.length})</button>
          </div>
          <span class="text-[11px] opacity-70">Vertebrae metadata excluded when publishing</span>
        </div>
        ${tabBody}
      </section>
    </div>
    ${renderImageModalHtml(novel)}
  `;
}

function render() {
  const root = document.getElementById('app');
  if (!root) return;
  applyTheme();

  const docsList = docEntries.filter((d) => d.group === 'docs');
  const userList = docEntries.filter((d) => d.group === 'user');
  const navBtnClass = (r: Route) =>
    `px-2.5 py-1.5 rounded-lg cursor-pointer ${
      state.route === r
        ? 'bg-[#f5f5f0] dark:bg-[#2d2d2a] font-semibold text-[#5A5A40] dark:text-[#e6e6de]'
        : 'hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a]'
    }`;

  root.innerHTML = `
    <div class="min-h-screen flex flex-col">
      <header class="sticky top-0 z-40 border-b border-[#e5e5e0] dark:border-[#2d2d2a] bg-white/95 dark:bg-[#1a1a18]/95 backdrop-blur px-4 py-2.5">
        <div class="max-w-7xl mx-auto flex flex-wrap items-center justify-between gap-2">
          <div class="flex items-center gap-2">
            <button type="button" data-nav="studio" class="font-serif text-lg font-bold text-[#5A5A40] dark:text-[#e6e6de] cursor-pointer">InterNovel</button>
            <button id="mobile-menu-btn" type="button" class="md:hidden px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-xs cursor-pointer">Menu</button>
            <nav class="hidden md:flex items-center gap-1 text-xs">
              <button type="button" data-nav="studio" class="${navBtnClass('studio')}">Studio</button>
              <button type="button" data-nav="casting" class="${navBtnClass('casting')}">Casting Studio</button>
              <button type="button" data-nav="cafe" class="${navBtnClass('cafe')}">Novel Café</button>
              <button type="button" data-nav="search" class="${navBtnClass('search')}">Search</button>

              <div class="relative" data-menu-wrap>
                <button id="docs-menu-btn" type="button" class="px-2.5 py-1.5 rounded-lg font-medium text-[#5A5A40] dark:text-[#d0d0c8] hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] cursor-pointer">Docs</button>
                ${
                  state.docsMenuOpen
                    ? `<div class="absolute left-0 mt-1 w-60 rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] shadow-lg p-1.5 z-50">${renderDropdownItems(docsList)}</div>`
                    : ''
                }
              </div>

              <div class="relative" data-menu-wrap>
                <button id="user-menu-btn" type="button" class="px-2.5 py-1.5 rounded-lg font-medium text-[#5A5A40] dark:text-[#d0d0c8] hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] cursor-pointer">User</button>
                ${
                  state.userMenuOpen
                    ? `<div class="absolute left-0 mt-1 w-56 rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] shadow-lg p-1.5 z-50">${renderDropdownItems(userList)}</div>`
                    : ''
                }
              </div>
            </nav>
          </div>

          <div class="flex flex-wrap items-center gap-2">
            <div class="flex items-center gap-1">
              <select id="novel-select" aria-label="Active Novel" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2.5 py-1 text-xs">
                ${
                  state.novels.length
                    ? (state.novels as Novel[])
                        .map(
                          (n, idx) => `<option value="${idx}" ${idx === state.activeNovelIdx ? 'selected' : ''}>${escapeHtml(n.title)}</option>`
                        )
                        .join('')
                    : `<option value="-1">No Novels</option>`
                }
              </select>
              <button id="new-novel-toggle" type="button" class="px-2 py-1 rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] text-xs cursor-pointer">+ Novel</button>
            </div>

            <div class="relative" data-menu-wrap>
              <button id="account-menu-btn" type="button" class="inline-flex items-center gap-2 rounded-full border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] pl-1.5 pr-3 py-1 text-xs font-medium cursor-pointer">
                ${
                  state.activeAccount
                    ? `${renderAvatarBadge(state.activeAccount, 'w-5 h-5 text-[10px]')}<span>${escapeHtml(state.activeAccount.displayName)}</span>`
                    : '<span class="px-1.5">Sign In / Identity</span>'
                }
              </button>
              ${
                state.accountMenuOpen
                  ? `<div class="absolute right-0 mt-1 w-64 rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] shadow-lg p-2 z-50 text-xs space-y-1">
                      ${(state.accounts as Account[])
                        .map(
                          (a, idx) => `<button type="button" data-switch-acc="${idx}" class="w-full text-left px-2.5 py-1.5 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] cursor-pointer flex items-center gap-2">${renderAvatarBadge(a, 'w-6 h-6 text-[10px]')}<div class="min-w-0 flex-1"><div class="font-medium truncate">${escapeHtml(a.displayName)}</div><div class="text-[10px] opacity-70 truncate">${escapeHtml(a.didShort)}</div></div></button>`
                        )
                        .join('')}
                      <button type="button" data-nav="profile" class="w-full text-left px-2.5 py-1.5 rounded-lg hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] cursor-pointer">Author Profile</button>
                      <button type="button" data-nav="auth" class="w-full text-left px-2.5 py-1.5 rounded-lg font-medium text-[#5A5A40] dark:text-[#d0d0c8] hover:bg-[#f5f5f0] dark:hover:bg-[#2d2d2a] cursor-pointer">Create / Manage Identities →</button>
                    </div>`
                  : ''
              }
            </div>

            <div class="relative" data-menu-wrap>
              <button id="settings-menu-btn" type="button" class="rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] px-2.5 py-1 text-xs cursor-pointer">Settings</button>
              ${
                state.settingsMenuOpen
                  ? `<div class="absolute right-0 mt-1 w-72 rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] shadow-lg p-3 z-50 text-xs space-y-3">
                      <div>
                        <div class="font-medium mb-1">Theme</div>
                        <div class="grid grid-cols-3 gap-1">
                          ${(['light', 'dark', 'system'] as const)
                            .map(
                              (t) => `<button type="button" data-theme="${t}" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer ${
                                state.themeMode === t ? 'bg-[#5A5A40] text-white' : ''
                              }">${t.charAt(0).toUpperCase() + t.slice(1)}</button>`
                            )
                            .join('')}
                        </div>
                      </div>
                      <div>
                        <div class="flex justify-between font-medium mb-1"><span>Text Scale</span><span id="scale-label">${state.textScale}%</span></div>
                        <input id="scale-input" type="range" min="25" max="300" step="10" value="${state.textScale}" class="w-full" />
                      </div>
                      <div>
                        <label class="block font-medium mb-1">Always-On Relay URL (Optional)</label>
                        <input id="relay-input" type="text" value="${escapeHtml(state.alwaysOnRelayUrl)}" placeholder="https://relay.exosystems.net" class="w-full rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1" />
                      </div>
                      <div>
                        <label class="block font-medium mb-1">Local Backup Schedule</label>
                        <select id="backup-select" class="w-full rounded border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-2 py-1">
                          <option value="Daily" ${state.backupSchedule === 'Daily' ? 'selected' : ''}>Daily</option>
                          <option value="Weekly" ${state.backupSchedule === 'Weekly' ? 'selected' : ''}>Weekly</option>
                          <option value="Manual" ${state.backupSchedule === 'Manual' ? 'selected' : ''}>Manual Only</option>
                        </select>
                      </div>
                      <div class="flex gap-1.5 pt-1 border-t border-[#e5e5e0] dark:border-[#2d2d2a]">
                        <button id="settings-export-btn" type="button" class="flex-1 px-2 py-1 rounded bg-[#5A5A40] text-white cursor-pointer">Export Backup</button>
                        <label class="flex-1 px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] text-center cursor-pointer">
                          Restore
                          <input id="settings-restore-input" type="file" accept=".json" class="hidden" />
                        </label>
                      </div>
                    </div>`
                  : ''
              }
            </div>
          </div>
        </div>

        ${
          state.mobileMenuOpen
            ? `<nav class="md:hidden pt-2 mt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] flex flex-wrap gap-1.5 text-xs">
                <button type="button" data-nav="studio" class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522] cursor-pointer">Studio</button>
                <button type="button" data-nav="casting" class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522] cursor-pointer">Casting Studio</button>
                <button type="button" data-nav="cafe" class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522] cursor-pointer">Novel Café</button>
                <button type="button" data-nav="search" class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522] cursor-pointer">Search</button>
                <button type="button" data-nav="profile" class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522] cursor-pointer">Profile</button>
                <button type="button" data-nav="auth" class="px-2 py-1 rounded bg-[#f5f5f0] dark:bg-[#252522] cursor-pointer">Identity</button>
                ${docsList.map((d) => `<button type="button" data-open-doc="${escapeHtml(d.path)}" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Doc: ${escapeHtml(d.title)}</button>`).join('')}
                ${userList.map((d) => `<button type="button" data-open-doc="${escapeHtml(d.path)}" class="px-2 py-1 rounded border border-[#e5e5e0] dark:border-[#2d2d2a] cursor-pointer">Guide: ${escapeHtml(d.title)}</button>`).join('')}
              </nav>`
            : ''
        }

        ${
          state.newNovelOpen
            ? `<div class="max-w-7xl mx-auto mt-2 pt-2 border-t border-[#e5e5e0] dark:border-[#2d2d2a] flex flex-wrap items-center gap-2 text-xs">
                <input id="new-novel-input" type="text" placeholder="New novel title..." class="flex-1 min-w-[180px] rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1" />
                <input id="new-novel-syn-input" type="text" placeholder="Short synopsis (optional)..." class="flex-1 min-w-[200px] rounded-lg border border-[#e5e5e0] dark:border-[#2d2d2a] bg-[#f5f5f0] dark:bg-[#252522] px-3 py-1" />
                <button id="new-novel-create" type="button" class="px-3 py-1 rounded-lg bg-[#5A5A40] text-white cursor-pointer">Create Novel</button>
              </div>`
            : ''
        }
      </header>
      ${
        state.statusNotice
          ? `<div class="max-w-7xl w-full mx-auto px-4 pt-3"><div class="rounded-xl border border-[#e5e5e0] dark:border-[#2d2d2a] bg-white dark:bg-[#1a1a18] px-4 py-2 text-xs flex items-center justify-between"><span>${escapeHtml(state.statusNotice)}</span><button id="clear-notice-btn" type="button" class="opacity-60 hover:opacity-100 cursor-pointer">×</button></div></div>`
          : ''
      }
      <main class="flex-1 max-w-7xl w-full mx-auto p-4 md:p-6">${renderMainContent()}</main>
    </div>
  `;

  persistState();
  bindEvents(root);
  if (pendingRestoreScrollTop !== null) {
    const targetTop = pendingRestoreScrollTop;
    pendingRestoreScrollTop = null;
    requestAnimationFrame(() => {
      const editorEl = document.getElementById('editor-textarea') as HTMLTextAreaElement | null;
      const previewEl = document.getElementById('editor-preview') as HTMLElement | null;
      if (editorEl) editorEl.scrollTop = targetTop;
      if (previewEl) previewEl.scrollTop = targetTop;
    });
  }
  if (state.cropDraft) {
    drawCropCanvas();
  }
}

function downloadJsonBackup() {
  const blob = new Blob(
    [
      JSON.stringify(
        {
          exportedAt: new Date().toISOString(),
          accounts: state.accounts,
          subgroups: state.subgroups,
          novels: state.novels,
          trunks: state.trunks,
        },
        null,
        2
      ),
    ],
    { type: 'application/json' }
  );
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `internovel-backup-${new Date().toISOString().slice(0, 10)}.json`;
  a.click();
  URL.revokeObjectURL(url);
}

function deleteNovelById(novelId: string) {
  const idx = (state.novels as Novel[]).findIndex((n) => n.id === novelId);
  if (idx < 0) return;
  const removed = state.novels[idx];
  state.novels.splice(idx, 1);
  if (state.activeNovelIdx >= state.novels.length) {
    state.activeNovelIdx = Math.max(0, state.novels.length - 1);
  }
  restoreNovelLocation(state.activeNovelIdx, { keepRoute: true });
  state.confirmDeleteNovelId = null;
  state.confirmDeleteCafeNovelId = null;
  state.editingNovelOpen = false;
  state.statusNotice = `Deleted novel "${removed.title}".`;
  render();
}

function deleteIdentityByDid(didShort: string) {
  state.accounts = (state.accounts as Account[]).filter((a) => a.didShort !== didShort);
  if (state.activeAccount?.didShort === didShort) {
    state.activeAccount = state.accounts[0] ?? null;
  }
  state.confirmDeleteAccountDid = null;
  state.statusNotice = 'Removed identity from local wallet.';
  render();
}

function bindEvents(root: HTMLElement) {
  root.querySelectorAll<HTMLElement>('[data-nav]').forEach((el) => {
    el.addEventListener('click', () => {
      snapshotCurrentNovelLocation();
      state.route = el.dataset.nav as Route;
      if (state.route === 'studio' || state.route === 'cafe') {
        const nv = getActiveNovel();
        if (nv) nv.lastRoute = state.route;
      }
      state.statusNotice = '';
      closeAllMenus();
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-open-doc]').forEach((el) => {
    el.addEventListener('click', () => {
      state.activeDocPath = el.dataset.openDoc ?? state.activeDocPath;
      state.route = 'doc';
      state.statusNotice = '';
      closeAllMenus();
      render();
    });
  });

  document.getElementById('mobile-menu-btn')?.addEventListener('click', (ev) => {
    ev.stopPropagation();
    state.mobileMenuOpen = !state.mobileMenuOpen;
    render();
  });

  document.getElementById('docs-menu-btn')?.addEventListener('click', (ev) => {
    ev.stopPropagation();
    const next = !state.docsMenuOpen;
    closeAllMenus();
    state.docsMenuOpen = next;
    render();
  });

  document.getElementById('user-menu-btn')?.addEventListener('click', (ev) => {
    ev.stopPropagation();
    const next = !state.userMenuOpen;
    closeAllMenus();
    state.userMenuOpen = next;
    render();
  });

  document.getElementById('account-menu-btn')?.addEventListener('click', (ev) => {
    ev.stopPropagation();
    const next = !state.accountMenuOpen;
    closeAllMenus();
    state.accountMenuOpen = next;
    render();
  });

  document.getElementById('settings-menu-btn')?.addEventListener('click', (ev) => {
    ev.stopPropagation();
    const next = !state.settingsMenuOpen;
    closeAllMenus();
    state.settingsMenuOpen = next;
    render();
  });

  document.getElementById('clear-notice-btn')?.addEventListener('click', () => {
    state.statusNotice = '';
    render();
  });

  document.getElementById('novel-select')?.addEventListener('change', (ev) => {
    const val = Number((ev.target as HTMLSelectElement).value);
    if (val >= 0 && val < state.novels.length) {
      switchToNovel(val);
      render();
    }
  });

  document.getElementById('new-novel-toggle')?.addEventListener('click', () => {
    state.newNovelOpen = !state.newNovelOpen;
    render();
  });

  document.getElementById('empty-create-novel-btn')?.addEventListener('click', () => {
    state.newNovelOpen = true;
    render();
  });

  document.getElementById('new-novel-create')?.addEventListener('click', () => {
    const input = document.getElementById('new-novel-input') as HTMLInputElement | null;
    const synInput = document.getElementById('new-novel-syn-input') as HTMLInputElement | null;
    const title = input?.value.trim() ?? '';
    if (!title) return;
    const id = `v-${Date.now()}`;
    snapshotCurrentNovelLocation();
    state.novels.push({
      id: `novel-${Date.now()}`,
      title,
      synopsis: synInput?.value.trim() || 'New collaborative novel workspace.',
      collabCall: '',
      seekingCollaborators: false,
      forkingAllowed: true,
      forkingLocked: false,
      favorited: true,
      lastVertebraId: id,
      lastStudioTab: 'editor',
      lastEditorView: 'split',
      lastScrollTop: 0,
      lastRoute: 'studio',
      pitches: [],
      gallery: [],
      vertebrae: [
        {
          id,
          kind: 'Idea',
          title: 'Chapter 1',
          storylineDate: new Date().toISOString().slice(0, 10),
          lastEditedTs: Date.now(),
          childCount: 1,
          tags: ['Draft'],
          content: `# ${title}\n\nStart writing...`,
          plotBlocks: ['Opening premise'],
        },
      ],
    });
    state.activeNovelIdx = state.novels.length - 1;
    state.selectedVertebraId = id;
    state.studioTab = 'editor';
    state.editorView = 'split';
    state.newNovelOpen = false;
    state.route = 'studio';
    render();
  });

  root.querySelectorAll<HTMLElement>('[data-theme]').forEach((el) => {
    el.addEventListener('click', () => {
      state.themeMode = el.dataset.theme as ThemeMode;
      render();
    });
  });

  document.getElementById('scale-input')?.addEventListener('input', (ev) => {
    state.textScale = Number((ev.target as HTMLInputElement).value);
    const label = document.getElementById('scale-label');
    if (label) label.textContent = `${state.textScale}%`;
    root.querySelectorAll<HTMLElement>('article.prose').forEach((a) => {
      a.style.fontSize = `${state.textScale}%`;
    });
    persistState();
  });

  document.getElementById('relay-input')?.addEventListener('input', (ev) => {
    state.alwaysOnRelayUrl = (ev.target as HTMLInputElement).value;
    persistState();
  });

  document.getElementById('backup-select')?.addEventListener('change', (ev) => {
    state.backupSchedule = (ev.target as HTMLSelectElement).value;
    persistState();
  });

  document.getElementById('settings-export-btn')?.addEventListener('click', () => {
    downloadJsonBackup();
    state.statusNotice = 'Exported local workspace backup (.json).';
    state.settingsMenuOpen = false;
    render();
  });

  document.getElementById('settings-restore-input')?.addEventListener('change', async (ev) => {
    const file = (ev.target as HTMLInputElement).files?.[0];
    if (!file) return;
    try {
      const parsed = JSON.parse(await file.text());
      if (Array.isArray(parsed.novels)) state.novels = normalizeLoadedNovels(parsed.novels);
      if (Array.isArray(parsed.accounts)) state.accounts = parsed.accounts;
      if (Array.isArray(parsed.subgroups)) state.subgroups = parsed.subgroups;
      if (Array.isArray(parsed.trunks)) state.trunks = parsed.trunks;
      state.statusNotice = `Restored workspace backup from "${file.name}".`;
    } catch {
      state.statusNotice = 'Could not restore backup: invalid JSON file.';
    }
    state.settingsMenuOpen = false;
    render();
  });

  // Left-nav Novel Spine + Novel Edit/Delete
  document.getElementById('toggle-spine-collapse-btn')?.addEventListener('click', () => {
    state.spineCollapsed = !state.spineCollapsed;
    render();
  });

  document.getElementById('toggle-spine-btn')?.addEventListener('click', () => {
    state.spineView = !state.spineView;
    render();
  });

  document.getElementById('spine-sort-select')?.addEventListener('change', (ev) => {
    state.spineSort = (ev.target as HTMLSelectElement).value as SpineSort;
    render();
  });

  root.querySelectorAll<HTMLElement>('[data-vertebra]').forEach((el) => {
    el.addEventListener('click', () => {
      state.selectedVertebraId = el.dataset.vertebra ?? state.selectedVertebraId;
      state.confirmDeleteVertebraId = null;
      state.editorCursorPos = null;
      const nv = getActiveNovel();
      if (nv) {
        nv.lastVertebraId = state.selectedVertebraId;
        nv.lastScrollTop = 0;
      }
      render();
    });
  });

  document.getElementById('new-vertebra-btn')?.addEventListener('click', () => {
    const input = document.getElementById('new-vertebra-input') as HTMLInputElement | null;
    const title = input?.value.trim() ?? '';
    if (!title) return;
    const id = `v-${Date.now()}`;
    const nv = getActiveNovel();
    nv?.vertebrae.push({
      id,
      kind: 'Idea',
      title,
      storylineDate: new Date().toISOString().slice(0, 10),
      lastEditedTs: Date.now(),
      childCount: 1,
      tags: ['Draft'],
      content: `# ${title}\n\nNew section content.`,
      plotBlocks: [`Outline for ${title}`],
    });
    state.selectedVertebraId = id;
    if (nv) {
      nv.lastVertebraId = id;
      nv.lastScrollTop = 0;
    }
    render();
  });

  document.getElementById('novel-fork-toggle')?.addEventListener('change', (ev) => {
    const novel = getActiveNovel();
    if (!novel || novel.forkingLocked) return;
    novel.forkingAllowed = (ev.target as HTMLInputElement).checked;
    render();
  });

  document.getElementById('toggle-edit-novel-btn')?.addEventListener('click', () => {
    state.editingNovelOpen = !state.editingNovelOpen;
    state.confirmDeleteNovelId = null;
    render();
  });

  document.getElementById('studio-save-novel-btn')?.addEventListener('click', () => {
    const novel = getActiveNovel();
    if (!novel) return;
    const title = (document.getElementById('studio-novel-title') as HTMLInputElement | null)?.value.trim();
    const syn = (document.getElementById('studio-novel-syn') as HTMLTextAreaElement | null)?.value.trim() ?? '';
    if (title) novel.title = title;
    novel.synopsis = syn;
    state.editingNovelOpen = false;
    state.statusNotice = `Updated details for "${novel.title}".`;
    render();
  });

  document.getElementById('studio-del-novel-btn')?.addEventListener('click', () => {
    const novel = getActiveNovel();
    if (novel) {
      state.confirmDeleteNovelId = novel.id;
      render();
    }
  });

  document.getElementById('studio-confirm-del-novel')?.addEventListener('click', () => {
    if (state.confirmDeleteNovelId) deleteNovelById(state.confirmDeleteNovelId);
  });

  // Vertebra Metadata + Tags + Delete
  document.getElementById('vertebra-title-input')?.addEventListener('change', (ev) => {
    const v = getActiveNovel()?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
    if (v) {
      v.title = (ev.target as HTMLInputElement).value.trim() || v.title;
      v.lastEditedTs = Date.now();
      render();
    }
  });

  document.getElementById('vertebra-kind-select')?.addEventListener('change', (ev) => {
    const v = getActiveNovel()?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
    if (v) {
      v.kind = (ev.target as HTMLSelectElement).value as Vertebra['kind'];
      render();
    }
  });

  document.getElementById('vertebra-date-input')?.addEventListener('change', (ev) => {
    const v = getActiveNovel()?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
    if (v) {
      v.storylineDate = (ev.target as HTMLInputElement).value;
      render();
    }
  });

  document.getElementById('del-vertebra-btn')?.addEventListener('click', () => {
    state.confirmDeleteVertebraId = state.selectedVertebraId;
    render();
  });

  document.getElementById('cancel-del-vertebra-btn')?.addEventListener('click', () => {
    state.confirmDeleteVertebraId = null;
    render();
  });

  document.getElementById('confirm-del-vertebra-btn')?.addEventListener('click', () => {
    const novel = getActiveNovel();
    if (!novel || novel.vertebrae.length <= 1) return;
    novel.vertebrae = novel.vertebrae.filter((v) => v.id !== state.selectedVertebraId);
    state.selectedVertebraId = novel.vertebrae[0]?.id ?? '';
    novel.lastVertebraId = state.selectedVertebraId;
    novel.lastScrollTop = 0;
    state.confirmDeleteVertebraId = null;
    state.statusNotice = 'Deleted chapter from novel spine.';
    render();
  });

  document.getElementById('new-tag-btn')?.addEventListener('click', () => {
    const input = document.getElementById('new-tag-input') as HTMLInputElement | null;
    const tag = input?.value.trim() ?? '';
    const v = getActiveNovel()?.vertebrae.find((x) => x.id === state.selectedVertebraId);
    if (tag && v && !v.tags.includes(tag)) {
      v.tags.push(tag);
      render();
    }
  });

  root.querySelectorAll<HTMLElement>('[data-rm-tag]').forEach((el) => {
    el.addEventListener('click', () => {
      const idx = Number(el.dataset.rmTag);
      const v = getActiveNovel()?.vertebrae.find((x) => x.id === state.selectedVertebraId);
      if (v) {
        v.tags.splice(idx, 1);
        render();
      }
    });
  });

  root.querySelectorAll<HTMLElement>('[data-stab]').forEach((el) => {
    el.addEventListener('click', () => {
      state.studioTab = el.dataset.stab as StudioTab;
      snapshotCurrentNovelLocation();
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-eview]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editorView = el.dataset.eview as EditorView;
      snapshotCurrentNovelLocation();
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-fmt]').forEach((el) => {
    el.addEventListener('click', () => {
      insertMarkdownAtCursor(el.dataset.fmt ?? '');
      render();
    });
  });

  // Editor typing + cursor tracking + drag-drop + clipboard paste for images
  const editorEl = document.getElementById('editor-textarea') as HTMLTextAreaElement | null;
  if (editorEl) {
    const syncCursor = () => {
      state.editorCursorPos = editorEl.selectionStart;
    };
    editorEl.addEventListener('click', syncCursor);
    editorEl.addEventListener('keyup', syncCursor);
    editorEl.addEventListener('select', syncCursor);

    editorEl.addEventListener('scroll', () => {
      const nv = getActiveNovel();
      if (nv) nv.lastScrollTop = editorEl.scrollTop;
      if (scrollPersistTimer !== null) window.clearTimeout(scrollPersistTimer);
      scrollPersistTimer = window.setTimeout(() => {
        persistState();
      }, 250);
    });

    editorEl.addEventListener('input', () => {
      const nv = getActiveNovel();
      const v = nv?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
      if (!v) return;
      state.editorCursorPos = editorEl.selectionStart;
      v.content = editorEl.value;
      v.lastEditedTs = Date.now();
      if (nv) {
        nv.lastVertebraId = v.id;
        nv.lastScrollTop = editorEl.scrollTop;
      }
      const preview = document.getElementById('editor-preview');
      if (preview) preview.innerHTML = renderMarkdownHtml(v.content, resolveNovelImage);
      const wc = document.getElementById('word-count');
      if (wc) wc.textContent = `${countWords(v.content)} words · Saved locally`;
      persistState();
    });

    const overlay = document.getElementById('editor-drop-overlay');
    editorEl.addEventListener('dragover', (ev) => {
      ev.preventDefault();
      overlay?.classList.remove('hidden');
    });
    editorEl.addEventListener('dragleave', () => {
      overlay?.classList.add('hidden');
    });
    editorEl.addEventListener('drop', (ev) => {
      ev.preventDefault();
      overlay?.classList.add('hidden');
      const file = ev.dataTransfer?.files?.[0];
      if (file) {
        state.editorCursorPos = editorEl.selectionStart;
        handleFileForCrop(file, true);
      }
    });
    editorEl.addEventListener('paste', (ev) => {
      const items = ev.clipboardData?.items;
      if (!items) return;
      for (const item of Array.from(items)) {
        if (item.type.startsWith('image/')) {
          const file = item.getAsFile();
          if (file) {
            ev.preventDefault();
            state.editorCursorPos = editorEl.selectionStart;
            handleFileForCrop(file, true);
            break;
          }
        }
      }
    });
  }

  const previewScrollEl = document.getElementById('editor-preview') as HTMLElement | null;
  if (previewScrollEl) {
    previewScrollEl.addEventListener('scroll', () => {
      const nv = getActiveNovel();
      if (nv && state.editorView === 'preview') {
        nv.lastScrollTop = previewScrollEl.scrollTop;
        if (scrollPersistTimer !== null) window.clearTimeout(scrollPersistTimer);
        scrollPersistTimer = window.setTimeout(() => {
          persistState();
        }, 250);
      }
    });
  }

  // Stateful sliding split divider
  const splitDivider = document.getElementById('split-divider');
  const splitContainer = document.getElementById('split-container');
  const editorPane = document.getElementById('editor-pane');
  if (splitDivider && splitContainer && editorPane) {
    let dragging = false;

    splitDivider.addEventListener('pointerdown', (ev) => {
      dragging = true;
      splitDivider.setPointerCapture(ev.pointerId);
      ev.preventDefault();
    });

    splitDivider.addEventListener('pointermove', (ev) => {
      if (!dragging) return;
      const rect = splitContainer.getBoundingClientRect();
      if (rect.width <= 0) return;
      const pct = Math.min(80, Math.max(20, ((ev.clientX - rect.left) / rect.width) * 100));
      state.splitRatio = Math.round(pct * 10) / 10;
      editorPane.style.width = `${state.splitRatio}%`;
      const badge = document.getElementById('split-badge');
      if (badge) {
        badge.textContent = `${Math.round(state.splitRatio)}% / ${Math.round(100 - state.splitRatio)}%`;
      }
    });

    const stopDrag = () => {
      if (!dragging) return;
      dragging = false;
      persistState();
    };
    splitDivider.addEventListener('pointerup', stopDrag);
    splitDivider.addEventListener('pointercancel', stopDrag);
    splitDivider.addEventListener('dblclick', () => {
      state.splitRatio = 50;
      editorPane.style.width = '50%';
      const badge = document.getElementById('split-badge');
      if (badge) badge.textContent = '50% / 50%';
      persistState();
    });
  }

  // Image Manager Modal & Cropper wiring
  document.getElementById('open-image-modal-btn')?.addEventListener('click', () => {
    state.cropDraft = null;
    state.imageModalOpen = true;
    render();
  });

  document.getElementById('close-image-modal-btn')?.addEventListener('click', () => {
    state.imageModalOpen = false;
    state.cropDraft = null;
    render();
  });

  document.getElementById('image-modal-backdrop')?.addEventListener('click', (ev) => {
    if (ev.target === ev.currentTarget) {
      state.imageModalOpen = false;
      state.cropDraft = null;
      render();
    }
  });

  const modalDropzone = document.getElementById('modal-dropzone');
  if (modalDropzone) {
    modalDropzone.addEventListener('dragover', (ev) => {
      ev.preventDefault();
      modalDropzone.classList.add('border-[#5A5A40]');
    });
    modalDropzone.addEventListener('dragleave', () => {
      modalDropzone.classList.remove('border-[#5A5A40]');
    });
    modalDropzone.addEventListener('drop', (ev) => {
      ev.preventDefault();
      const file = ev.dataTransfer?.files?.[0];
      if (file) handleFileForCrop(file, true);
    });
  }

  document.getElementById('modal-file-input')?.addEventListener('change', (ev) => {
    const file = (ev.target as HTMLInputElement).files?.[0];
    if (file) handleFileForCrop(file, true);
  });

  root.querySelectorAll<HTMLElement>('[data-modal-insert-img]').forEach((el) => {
    el.addEventListener('click', () => {
      const img = getActiveNovel()?.gallery[Number(el.dataset.modalInsertImg)];
      if (img) insertGalleryImageIntoChapter(img);
    });
  });

  root.querySelectorAll<HTMLElement>('[data-modal-crop-img]').forEach((el) => {
    el.addEventListener('click', () => {
      const img = getActiveNovel()?.gallery[Number(el.dataset.modalCropImg)];
      if (img) startCropSessionFromDataUrl(img.dataUrl, `${img.title} (Cropped)`, true);
    });
  });

  // Cropper controls
  const cropCanvas = document.getElementById('crop-canvas') as HTMLCanvasElement | null;
  if (cropCanvas && state.cropDraft) {
    let mode: 'none' | 'move' | 'resize' = 'none';
    let startX = 0;
    let startY = 0;
    let origRect = { x: 0, y: 0, w: 0, h: 0 };

    cropCanvas.addEventListener('pointerdown', (ev) => {
      const draft = state.cropDraft;
      if (!draft) return;
      const rect = cropCanvas.getBoundingClientRect();
      const nx = (ev.clientX - rect.left) / rect.width;
      const ny = (ev.clientY - rect.top) / rect.height;

      const brX = draft.cropX + draft.cropW;
      const brY = draft.cropY + draft.cropH;
      if (Math.abs(nx - brX) < 0.06 && Math.abs(ny - brY) < 0.06) {
        mode = 'resize';
      } else if (nx >= draft.cropX && nx <= brX && ny >= draft.cropY && ny <= brY) {
        mode = 'move';
      } else {
        mode = 'resize';
        draft.cropX = Math.max(0, Math.min(0.85, nx));
        draft.cropY = Math.max(0, Math.min(0.85, ny));
        draft.cropW = 0.1;
        draft.cropH = 0.1;
      }

      startX = nx;
      startY = ny;
      origRect = { x: draft.cropX, y: draft.cropY, w: draft.cropW, h: draft.cropH };
      cropCanvas.setPointerCapture(ev.pointerId);
    });

    cropCanvas.addEventListener('pointermove', (ev) => {
      const draft = state.cropDraft;
      if (!draft || mode === 'none') return;
      const rect = cropCanvas.getBoundingClientRect();
      const nx = (ev.clientX - rect.left) / rect.width;
      const ny = (ev.clientY - rect.top) / rect.height;
      const dx = nx - startX;
      const dy = ny - startY;

      if (mode === 'move') {
        draft.cropX = Math.max(0, Math.min(1 - draft.cropW, origRect.x + dx));
        draft.cropY = Math.max(0, Math.min(1 - draft.cropH, origRect.y + dy));
      } else if (mode === 'resize') {
        let w = Math.max(0.12, Math.min(1 - draft.cropX, origRect.w + dx));
        let h = Math.max(0.12, Math.min(1 - draft.cropY, origRect.h + dy));
        if (draft.aspect !== 'free') {
          const [aw, ah] = draft.aspect.split(':').map(Number);
          const canvasRatio = rect.width / rect.height;
          h = Math.min(1 - draft.cropY, (w * canvasRatio * ah) / aw);
          w = Math.min(1 - draft.cropX, (h * aw) / (canvasRatio * ah));
        }
        draft.cropW = Math.max(0.1, w);
        draft.cropH = Math.max(0.1, h);
      }
      drawCropCanvas();
    });

    const stopCrop = () => {
      mode = 'none';
    };
    cropCanvas.addEventListener('pointerup', stopCrop);
    cropCanvas.addEventListener('pointercancel', stopCrop);
  }

  document.getElementById('crop-alt-input')?.addEventListener('input', (ev) => {
    if (state.cropDraft) state.cropDraft.alt = (ev.target as HTMLInputElement).value;
  });

  root.querySelectorAll<HTMLElement>('[data-crop-aspect]').forEach((el) => {
    el.addEventListener('click', () => {
      const draft = state.cropDraft;
      if (!draft) return;
      draft.aspect = el.dataset.cropAspect as CropAspect;
      if (draft.aspect !== 'free') {
        const [aw, ah] = draft.aspect.split(':').map(Number);
        const canvas = document.getElementById('crop-canvas') as HTMLCanvasElement | null;
        const canvasRatio = canvas ? canvas.width / canvas.height : 1.5;
        let w = draft.cropW;
        let h = (w * canvasRatio * ah) / aw;
        if (draft.cropY + h > 1) {
          h = 1 - draft.cropY;
          w = (h * aw) / (canvasRatio * ah);
        }
        draft.cropW = Math.max(0.12, Math.min(1 - draft.cropX, w));
        draft.cropH = Math.max(0.12, Math.min(1 - draft.cropY, h));
      }
      render();
    });
  });

  document.getElementById('crop-zoom-input')?.addEventListener('input', (ev) => {
    if (state.cropDraft) {
      state.cropDraft.zoom = Number((ev.target as HTMLInputElement).value);
      drawCropCanvas();
    }
  });

  document.getElementById('crop-rot-left')?.addEventListener('click', () => {
    if (state.cropDraft) {
      state.cropDraft.rotation = (state.cropDraft.rotation + 270) % 360;
      drawCropCanvas();
    }
  });

  document.getElementById('crop-rot-right')?.addEventListener('click', () => {
    if (state.cropDraft) {
      state.cropDraft.rotation = (state.cropDraft.rotation + 90) % 360;
      drawCropCanvas();
    }
  });

  document.getElementById('crop-flip-x')?.addEventListener('click', () => {
    if (state.cropDraft) {
      state.cropDraft.flipX = !state.cropDraft.flipX;
      drawCropCanvas();
    }
  });

  document.getElementById('crop-flip-y')?.addEventListener('click', () => {
    if (state.cropDraft) {
      state.cropDraft.flipY = !state.cropDraft.flipY;
      drawCropCanvas();
    }
  });

  document.getElementById('crop-back-btn')?.addEventListener('click', () => {
    state.cropDraft = null;
    render();
  });

  const commitCrop = (shouldInsert: boolean) => {
    const draft = state.cropDraft;
    const novel = getActiveNovel();
    if (!draft || !novel) return;
    const baked = bakeCroppedImage(draft);
    const img: GalleryImage = {
      id: `img-${Date.now()}`,
      title: draft.alt.trim() || `Illustration ${novel.gallery.length + 1}`,
      dataUrl: baked.dataUrl,
      width: baked.width,
      height: baked.height,
      sizeKb: baked.sizeKb,
    };
    novel.gallery.push(img);
    state.cropDraft = null;
    if (shouldInsert) {
      insertGalleryImageIntoChapter(img);
    } else {
      state.imageModalOpen = false;
      state.statusNotice = `Saved "${img.title}" (${img.sizeKb} KB) to Novel Image Gallery.`;
      render();
    }
  };

  document.getElementById('crop-save-insert-btn')?.addEventListener('click', () => commitCrop(true));
  document.getElementById('crop-save-only-btn')?.addEventListener('click', () => commitCrop(false));

  // Gallery Tab Drag & Drop + Card CRUD
  const galleryDropzone = document.getElementById('gallery-dropzone');
  if (galleryDropzone) {
    galleryDropzone.addEventListener('dragover', (ev) => {
      ev.preventDefault();
      galleryDropzone.classList.add('border-[#5A5A40]');
    });
    galleryDropzone.addEventListener('dragleave', () => {
      galleryDropzone.classList.remove('border-[#5A5A40]');
    });
    galleryDropzone.addEventListener('drop', (ev) => {
      ev.preventDefault();
      const file = ev.dataTransfer?.files?.[0];
      if (file) handleFileForCrop(file, false);
    });
  }

  document.getElementById('gallery-upload-input')?.addEventListener('change', (ev) => {
    const file = (ev.target as HTMLInputElement).files?.[0];
    if (file) handleFileForCrop(file, false);
  });

  root.querySelectorAll<HTMLElement>('[data-img-edit-title]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingGalleryIdx = Number(el.dataset.imgEditTitle);
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-img-save-title]').forEach((el) => {
    el.addEventListener('click', () => {
      const idx = Number(el.dataset.imgSaveTitle);
      const val = (document.getElementById('img-rename-input') as HTMLInputElement | null)?.value.trim();
      const img = getActiveNovel()?.gallery[idx];
      if (img && val) img.title = val;
      state.editingGalleryIdx = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-img-insert]').forEach((el) => {
    el.addEventListener('click', () => {
      const img = getActiveNovel()?.gallery[Number(el.dataset.imgInsert)];
      if (img) insertGalleryImageIntoChapter(img);
    });
  });

  root.querySelectorAll<HTMLElement>('[data-img-crop]').forEach((el) => {
    el.addEventListener('click', () => {
      const img = getActiveNovel()?.gallery[Number(el.dataset.imgCrop)];
      if (img) startCropSessionFromDataUrl(img.dataUrl, `${img.title} (Edited Copy)`, false);
    });
  });

  root.querySelectorAll<HTMLElement>('[data-img-del]').forEach((el) => {
    el.addEventListener('click', () => {
      getActiveNovel()?.gallery.splice(Number(el.dataset.imgDel), 1);
      state.editingGalleryIdx = null;
      render();
    });
  });

  // Plot Planner CRUD
  document.getElementById('plot-block-add')?.addEventListener('click', () => {
    const input = document.getElementById('plot-block-input') as HTMLInputElement | null;
    const txt = input?.value.trim() ?? '';
    if (!txt) return;
    const v = getActiveNovel()?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
    if (v) {
      v.plotBlocks.push(txt);
      v.childCount = v.plotBlocks.length;
    }
    render();
  });

  root.querySelectorAll<HTMLElement>('[data-block-edit]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingBlockIdx = Number(el.dataset.blockEdit);
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-block-cancel]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingBlockIdx = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-block-save]').forEach((el) => {
    el.addEventListener('click', () => {
      const i = Number(el.dataset.blockSave);
      const val = (document.getElementById('block-edit-input') as HTMLInputElement | null)?.value.trim();
      const v = getActiveNovel()?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
      if (v && val && i < v.plotBlocks.length) {
        v.plotBlocks[i] = val;
      }
      state.editingBlockIdx = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-block-up]').forEach((el) => {
    el.addEventListener('click', () => {
      const i = Number(el.dataset.blockUp);
      const v = getActiveNovel()?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
      if (!v || i <= 0) return;
      [v.plotBlocks[i - 1], v.plotBlocks[i]] = [v.plotBlocks[i], v.plotBlocks[i - 1]];
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-block-dn]').forEach((el) => {
    el.addEventListener('click', () => {
      const i = Number(el.dataset.blockDn);
      const v = getActiveNovel()?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
      if (!v || i + 1 >= v.plotBlocks.length) return;
      [v.plotBlocks[i], v.plotBlocks[i + 1]] = [v.plotBlocks[i + 1], v.plotBlocks[i]];
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-block-del]').forEach((el) => {
    el.addEventListener('click', () => {
      const i = Number(el.dataset.blockDel);
      const v = getActiveNovel()?.vertebrae.find((x: Vertebra) => x.id === state.selectedVertebraId);
      if (!v) return;
      v.plotBlocks.splice(i, 1);
      v.childCount = v.plotBlocks.length;
      state.editingBlockIdx = null;
      render();
    });
  });

  // Collaborators & Contacts Subgroups CRUD
  document.getElementById('subgroup-new-btn')?.addEventListener('click', () => {
    const name = (document.getElementById('subgroup-new-input') as HTMLInputElement | null)?.value.trim() ?? '';
    if (!name) return;
    if (!(state.subgroups as ContactSubgroup[]).some((g) => g.name.toLowerCase() === name.toLowerCase())) {
      state.subgroups.push({ name, members: [] });
    }
    render();
  });

  root.querySelectorAll<HTMLElement>('[data-edit-subgroup]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingSubgroupIdx = Number(el.dataset.editSubgroup);
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-cancel-subgroup]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingSubgroupIdx = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-save-subgroup]').forEach((el) => {
    el.addEventListener('click', () => {
      const idx = Number(el.dataset.saveSubgroup);
      const val = (document.getElementById('subgroup-rename-input') as HTMLInputElement | null)?.value.trim();
      if (val && state.subgroups[idx]) state.subgroups[idx].name = val;
      state.editingSubgroupIdx = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-subgroup]').forEach((el) => {
    el.addEventListener('click', () => {
      const idx = Number(el.dataset.delSubgroup);
      state.subgroups.splice(idx, 1);
      state.editingSubgroupIdx = null;
      render();
    });
  });

  document.getElementById('contact-add-btn')?.addEventListener('click', () => {
    const did = (document.getElementById('contact-did-input') as HTMLInputElement | null)?.value.trim();
    const gIdx = Number((document.getElementById('contact-group-select') as HTMLSelectElement | null)?.value ?? 0);
    if (!did) return;
    const grp = state.subgroups[gIdx];
    if (grp && !grp.members.includes(did)) grp.members.push(did);
    render();
  });

  root.querySelectorAll<HTMLElement>('[data-rm-contact]').forEach((el) => {
    el.addEventListener('click', () => {
      const [gIdx, mIdx] = (el.dataset.rmContact ?? '').split(':').map(Number);
      state.subgroups[gIdx]?.members.splice(mIdx, 1);
      state.statusNotice = 'Removed contact and revoked subgroup permissions.';
      render();
    });
  });

  // Auth / Identities CRUD
  document.getElementById('auth-shuffle-btn')?.addEventListener('click', () => {
    state.shuffledPin = [...state.shuffledPin].sort(() => Math.random() - 0.5);
    state.lastPazzle = [...state.lastPazzle].sort(() => Math.random() - 0.5);
    render();
  });

  document.getElementById('auth-create-btn')?.addEventListener('click', () => {
    const displayName = (document.getElementById('auth-name') as HTMLInputElement | null)?.value.trim() || 'Author';
    const securityText = (document.getElementById('auth-sec') as HTMLInputElement | null)?.value.trim() || 'harbor';
    const inMemory = Boolean((document.getElementById('auth-mem') as HTMLInputElement | null)?.checked);
    const shortDid = `did:peer:4zQm${Math.random().toString(36).slice(2, 12)}`;
    const acc: Account = {
      displayName,
      bio: 'Collaborative novelist.',
      avatarInitials: displayName.slice(0, 2).toUpperCase(),
      avatarDataUrl: '',
      didShort: shortDid,
      didLong: `${shortDid}:z2vD${Math.random().toString(36).slice(2, 16)}`,
      securityText,
      inMemory,
    };
    state.accounts.push(acc);
    state.activeAccount = acc;
    state.statusNotice = inMemory
      ? `Started ephemeral session for ${displayName}.`
      : `Created local identity for ${displayName}.`;
    render();
  });

  document.getElementById('auth-update-btn')?.addEventListener('click', () => {
    if (!state.activeAccount) return;
    const displayName = (document.getElementById('auth-name') as HTMLInputElement | null)?.value.trim();
    const securityText = (document.getElementById('auth-sec') as HTMLInputElement | null)?.value.trim();
    const inMemory = Boolean((document.getElementById('auth-mem') as HTMLInputElement | null)?.checked);
    if (displayName) {
      state.activeAccount.displayName = displayName;
      state.activeAccount.avatarInitials = displayName.slice(0, 2).toUpperCase();
    }
    if (securityText) state.activeAccount.securityText = securityText;
    state.activeAccount.inMemory = inMemory;
    const match = (state.accounts as Account[]).find((a) => a.didShort === state.activeAccount?.didShort);
    if (match) Object.assign(match, state.activeAccount);
    state.statusNotice = 'Updated active author identity settings.';
    render();
  });

  document.getElementById('auth-export-btn')?.addEventListener('click', () => {
    const area = document.getElementById('auth-import-code') as HTMLTextAreaElement | null;
    if (area && state.activeAccount) {
      area.value = `inw4:${btoa(JSON.stringify(state.activeAccount))}`;
    }
  });

  document.getElementById('auth-import-btn')?.addEventListener('click', () => {
    const raw = (document.getElementById('auth-import-code') as HTMLTextAreaElement | null)?.value.trim() ?? '';
    const cleaned = raw.startsWith('inw4:') ? raw.slice(5) : raw;
    try {
      const acc = JSON.parse(atob(cleaned)) as Account;
      if (acc.displayName && acc.didShort) {
        if (!(state.accounts as Account[]).some((a) => a.didShort === acc.didShort)) {
          state.accounts.push(acc);
        }
        state.activeAccount = acc;
        state.statusNotice = `Imported and unlocked wallet for ${acc.displayName}.`;
        render();
      }
    } catch {
      state.statusNotice = 'Invalid transfer code.';
      render();
    }
  });

  root.querySelectorAll<HTMLElement>('[data-switch-acc]').forEach((el) => {
    el.addEventListener('click', () => {
      const idx = Number(el.dataset.switchAcc);
      state.activeAccount = state.accounts[idx] ?? state.activeAccount;
      state.accountMenuOpen = false;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-acc]').forEach((el) => {
    el.addEventListener('click', () => {
      state.confirmDeleteAccountDid = el.dataset.delAcc ?? null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-acc-cancel]').forEach((el) => {
    el.addEventListener('click', () => {
      state.confirmDeleteAccountDid = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-acc-confirm]').forEach((el) => {
    el.addEventListener('click', () => {
      const did = el.dataset.delAccConfirm;
      if (did) deleteIdentityByDid(did);
    });
  });

  // Casting Studio CRUD
  document.getElementById('trunk-add-btn')?.addEventListener('click', () => {
    const name = (document.getElementById('trunk-name') as HTMLInputElement | null)?.value.trim() ?? '';
    const bio = (document.getElementById('trunk-bio') as HTMLInputElement | null)?.value.trim() ?? '';
    const sample =
      (document.getElementById('trunk-sample') as HTMLInputElement | null)?.value.trim() ||
      'Character travel trunk narrative sample.';
    if (!name) return;
    state.trunks.push({
      id: `tr-${Date.now()}`,
      name,
      ownerName: state.activeAccount?.displayName ?? 'Local Author',
      ownerDid: state.activeAccount?.didShort ?? 'did:peer:4zQmLocalAuthor',
      bio,
      sample,
      proposedTo: null,
    });
    render();
  });

  root.querySelectorAll<HTMLElement>('[data-edit-trunk]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingTrunkId = el.dataset.editTrunk ?? null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-cancel-trunk-edit]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingTrunkId = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-save-trunk]').forEach((el) => {
    el.addEventListener('click', () => {
      const tid = el.dataset.saveTrunk;
      const trunk = (state.trunks as CharacterTrunk[]).find((x) => x.id === tid);
      if (!trunk) return;
      const name = (document.getElementById(`edit-trunk-name-${tid}`) as HTMLInputElement | null)?.value.trim();
      const bio = (document.getElementById(`edit-trunk-bio-${tid}`) as HTMLInputElement | null)?.value.trim() ?? '';
      const sample = (document.getElementById(`edit-trunk-sample-${tid}`) as HTMLTextAreaElement | null)?.value.trim() ?? '';
      if (name) trunk.name = name;
      trunk.bio = bio;
      if (sample) trunk.sample = sample;
      state.editingTrunkId = null;
      state.statusNotice = `Updated character trunk "${trunk.name}".`;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-trunk]').forEach((el) => {
    el.addEventListener('click', () => {
      state.confirmDeleteTrunkId = el.dataset.delTrunk ?? null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-trunk-cancel]').forEach((el) => {
    el.addEventListener('click', () => {
      state.confirmDeleteTrunkId = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-trunk-confirm]').forEach((el) => {
    el.addEventListener('click', () => {
      const tid = el.dataset.delTrunkConfirm;
      state.trunks = (state.trunks as CharacterTrunk[]).filter((x) => x.id !== tid);
      state.confirmDeleteTrunkId = null;
      state.statusNotice = 'Removed character travel trunk.';
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-propose-trunk]').forEach((el) => {
    el.addEventListener('click', () => {
      const tid = el.dataset.proposeTrunk;
      const t = state.trunks.find((x: CharacterTrunk) => x.id === tid);
      const selectEl = document.getElementById(`propose-target-${tid}`) as HTMLSelectElement | null;
      const targetTitle = selectEl?.value || getActiveNovel()?.title;
      if (t && targetTitle) {
        t.proposedTo = targetTitle;
        state.statusNotice = `Sent cross-story character proposal for ${t.name} -> "${targetTitle}".`;
        render();
      }
    });
  });

  root.querySelectorAll<HTMLElement>('[data-withdraw-trunk]').forEach((el) => {
    el.addEventListener('click', () => {
      const t = state.trunks.find((x: CharacterTrunk) => x.id === el.dataset.withdrawTrunk);
      if (t) {
        t.proposedTo = null;
        state.statusNotice = `Withdrew character proposal for ${t.name}.`;
        render();
      }
    });
  });

  // Novel Cafe CRUD
  root.querySelectorAll<HTMLElement>('[data-cafe-filter]').forEach((el) => {
    el.addEventListener('click', () => {
      state.cafeFilter = el.dataset.cafeFilter as CafeFilter;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-edit-cafe-novel]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingCafeNovelId = el.dataset.editCafeNovel ?? null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-cancel-cafe-edit]').forEach((el) => {
    el.addEventListener('click', () => {
      state.editingCafeNovelId = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-save-cafe-novel]').forEach((el) => {
    el.addEventListener('click', () => {
      const nid = el.dataset.saveCafeNovel;
      const n = (state.novels as Novel[]).find((x) => x.id === nid);
      if (!n) return;
      const title = (document.getElementById(`cafe-edit-title-${nid}`) as HTMLInputElement | null)?.value.trim();
      const syn = (document.getElementById(`cafe-edit-syn-${nid}`) as HTMLTextAreaElement | null)?.value.trim() ?? '';
      const call = (document.getElementById(`cafe-edit-call-${nid}`) as HTMLInputElement | null)?.value.trim() ?? '';
      const seeking = Boolean((document.getElementById(`cafe-edit-seeking-${nid}`) as HTMLInputElement | null)?.checked);
      if (title) n.title = title;
      n.synopsis = syn;
      n.collabCall = call;
      n.seekingCollaborators = seeking;
      state.editingCafeNovelId = null;
      state.statusNotice = `Updated Novel Café listing for "${n.title}".`;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-cafe-novel]').forEach((el) => {
    el.addEventListener('click', () => {
      state.confirmDeleteCafeNovelId = el.dataset.delCafeNovel ?? null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-cafe-cancel]').forEach((el) => {
    el.addEventListener('click', () => {
      state.confirmDeleteCafeNovelId = null;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-cafe-confirm]').forEach((el) => {
    el.addEventListener('click', () => {
      if (el.dataset.delCafeConfirm) deleteNovelById(el.dataset.delCafeConfirm);
    });
  });

  root.querySelectorAll<HTMLElement>('[data-open-studio-novel]').forEach((el) => {
    el.addEventListener('click', () => {
      const idx = (state.novels as Novel[]).findIndex((x) => x.id === el.dataset.openStudioNovel);
      if (idx >= 0) {
        switchToNovel(idx, { keepRoute: true });
        state.route = 'studio';
        const nv = getActiveNovel();
        if (nv) nv.lastRoute = 'studio';
        render();
      }
    });
  });

  root.querySelectorAll<HTMLElement>('[data-add-pitch]').forEach((el) => {
    el.addEventListener('click', () => {
      const nid = el.dataset.addPitch;
      const n = (state.novels as Novel[]).find((x) => x.id === nid);
      const text = (document.getElementById(`cafe-pitch-input-${nid}`) as HTMLInputElement | null)?.value.trim();
      if (!n || !text) return;
      n.pitches.push({
        id: `pitch-${Date.now()}`,
        author: state.activeAccount?.displayName ?? 'Guest Author',
        text,
        date: new Date().toISOString().slice(0, 10),
      });
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-del-pitch]').forEach((el) => {
    el.addEventListener('click', () => {
      const [nid, pIdxStr] = (el.dataset.delPitch ?? '').split(':');
      const n = (state.novels as Novel[]).find((x) => x.id === nid);
      if (n) {
        n.pitches.splice(Number(pIdxStr), 1);
        render();
      }
    });
  });

  root.querySelectorAll<HTMLElement>('[data-fork-novel]').forEach((el) => {
    el.addEventListener('click', () => {
      const orig = state.novels.find((n: Novel) => n.id === el.dataset.forkNovel);
      if (!orig || !orig.forkingAllowed) return;
      snapshotCurrentNovelLocation();
      orig.forkingLocked = true;
      const firstId = orig.vertebrae[0]?.id ?? '';
      const forked: Novel = {
        ...structuredClone(orig),
        id: `novel-fork-${Date.now()}`,
        title: `${orig.title} (Remix)`,
        forkingLocked: false,
        remixedFromId: orig.id,
        remixedFromTitle: orig.title,
        remixedBaselineText: orig.vertebrae[0]?.content ?? '',
        lastVertebraId: firstId,
        lastStudioTab: 'editor',
        lastEditorView: 'split',
        lastScrollTop: 0,
        lastRoute: 'cafe',
        pitches: [],
      };
      state.novels.push(forked);
      state.activeNovelIdx = state.novels.length - 1;
      state.selectedVertebraId = firstId || state.selectedVertebraId;
      state.statusNotice = `Forked "${orig.title}". Original forking policy is now permanently locked to protect derivative lineage.`;
      render();
    });
  });

  root.querySelectorAll<HTMLElement>('[data-toggle-remix]').forEach((el) => {
    el.addEventListener('click', () => {
      const id = el.dataset.toggleRemix ?? null;
      state.remixDiffOpenFor = state.remixDiffOpenFor === id ? null : id;
      render();
    });
  });

  // Real Search & Network Gossip Query
  const refreshSearchResults = () => {
    const target = document.getElementById('search-results');
    if (target) {
      target.innerHTML = computeSearchHitsHtml();
      bindSearchResultHandlers();
    }
  };

  document.getElementById('search-input')?.addEventListener('input', (ev) => {
    state.searchQuery = (ev.target as HTMLInputElement).value;
    refreshSearchResults();
  });

  document.getElementById('search-input')?.addEventListener('keydown', (ev) => {
    if (ev.key === 'Enter') {
      executeNetworkQuery();
      render();
    }
  });

  document.getElementById('search-clear-btn')?.addEventListener('click', () => {
    state.searchQuery = '';
    state.gossipHits = [];
    state.gossipStatus = '';
    render();
  });

  document.getElementById('search-novel-scope')?.addEventListener('change', (ev) => {
    state.searchNovelScope = (ev.target as HTMLSelectElement).value;
    refreshSearchResults();
  });

  document.getElementById('search-sort-select')?.addEventListener('change', (ev) => {
    state.searchSort = (ev.target as HTMLSelectElement).value as SearchSort;
    refreshSearchResults();
  });

  root.querySelectorAll<HTMLInputElement>('[data-search-cat]').forEach((el) => {
    el.addEventListener('change', () => {
      const cat = el.dataset.searchCat as keyof typeof state.searchCategories;
      state.searchCategories[cat] = el.checked;
      refreshSearchResults();
    });
  });

  document.getElementById('search-cats-all')?.addEventListener('click', () => {
    (Object.keys(state.searchCategories) as (keyof typeof state.searchCategories)[]).forEach((k) => {
      state.searchCategories[k] = true;
    });
    render();
  });

  document.getElementById('search-cats-none')?.addEventListener('click', () => {
    (Object.keys(state.searchCategories) as (keyof typeof state.searchCategories)[]).forEach((k) => {
      state.searchCategories[k] = false;
    });
    render();
  });

  document.getElementById('gossip-depth')?.addEventListener('input', (ev) => {
    state.gossipDepth = Number((ev.target as HTMLInputElement).value);
  });

  document.getElementById('gossip-send-btn')?.addEventListener('click', () => {
    executeNetworkQuery();
    render();
  });

  bindSearchResultHandlers();

  // Profile CRUD
  document.getElementById('profile-avatar-file')?.addEventListener('change', (ev) => {
    const file = (ev.target as HTMLInputElement).files?.[0];
    if (!file || !state.activeAccount) return;
    const reader = new FileReader();
    reader.onload = () => {
      const rawUrl = String(reader.result);
      const saveAvatar = (finalUrl: string) => {
        if (!state.activeAccount) return;
        state.activeAccount.avatarDataUrl = finalUrl;
        const match = (state.accounts as Account[]).find((a) => a.didShort === state.activeAccount?.didShort);
        if (match) match.avatarDataUrl = finalUrl;
        state.statusNotice = 'Updated author avatar.';
        render();
      };
      const img = new Image();
      img.onload = () => {
        try {
          const maxDim = 256;
          const scale = Math.min(1, maxDim / Math.max(img.width || maxDim, img.height || maxDim));
          const w = Math.max(1, Math.round(img.width * scale));
          const h = Math.max(1, Math.round(img.height * scale));
          const canvas = document.createElement('canvas');
          canvas.width = w;
          canvas.height = h;
          const ctx = canvas.getContext('2d');
          if (!ctx) {
            saveAvatar(rawUrl);
            return;
          }
          ctx.drawImage(img, 0, 0, w, h);
          saveAvatar(canvas.toDataURL('image/webp', 0.85));
        } catch {
          saveAvatar(rawUrl);
        }
      };
      img.onerror = () => saveAvatar(rawUrl);
      img.src = rawUrl;
    };
    reader.readAsDataURL(file);
  });

  document.getElementById('profile-clear-avatar-btn')?.addEventListener('click', () => {
    if (!state.activeAccount) return;
    state.activeAccount.avatarDataUrl = '';
    const match = (state.accounts as Account[]).find((a) => a.didShort === state.activeAccount?.didShort);
    if (match) match.avatarDataUrl = '';
    render();
  });

  document.getElementById('profile-save-btn')?.addEventListener('click', () => {
    const name = (document.getElementById('profile-name') as HTMLInputElement | null)?.value.trim();
    const bio = (document.getElementById('profile-bio') as HTMLTextAreaElement | null)?.value.trim() ?? '';
    const sec = (document.getElementById('profile-sec') as HTMLInputElement | null)?.value.trim();
    if (state.activeAccount) {
      if (name) {
        state.activeAccount.displayName = name;
        state.activeAccount.avatarInitials = name.slice(0, 2).toUpperCase();
      }
      state.activeAccount.bio = bio;
      if (sec) state.activeAccount.securityText = sec;
      const match = (state.accounts as Account[]).find((a) => a.didShort === state.activeAccount?.didShort);
      if (match) Object.assign(match, state.activeAccount);
      state.statusNotice = 'Profile updated.';
      render();
    }
  });

  document.getElementById('profile-export-btn')?.addEventListener('click', downloadJsonBackup);
}

// Close dropdown menus cleanly when clicking outside or pressing Escape
document.addEventListener('click', (ev) => {
  const target = ev.target as HTMLElement | null;
  if (!target?.closest('[data-menu-wrap]') && !target?.closest('#mobile-menu-btn')) {
    if (state.docsMenuOpen || state.userMenuOpen || state.accountMenuOpen || state.settingsMenuOpen) {
      closeAllMenus();
      render();
    }
  }
});

document.addEventListener('keydown', (ev) => {
  if (ev.key === 'Escape') {
    closeAllMenus();
    state.newNovelOpen = false;
    state.imageModalOpen = false;
    state.cropDraft = null;
    render();
  }
});

render();
