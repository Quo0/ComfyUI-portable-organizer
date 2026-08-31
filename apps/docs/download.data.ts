// A VitePress data loader: it runs once at build time and the result is
// frozen into the static output, so the download page makes no network
// request of its own.
//
// The site build must never fail because of GitHub. An unreachable API, an
// exhausted rate limit, no releases at all — every one of them is a normal
// state, and each returns the fallback below.

export interface DownloadInfo {
  /** `false` — there is no real release, show only the link to the list. */
  available: boolean;
  version: string;
  /** ISO date, formatted on the page. */
  publishedAt: string;
  url: string;
  name: string;
  /** In bytes. */
  size: number;
  /** Empty when the release carries no SHA256SUMS.txt. */
  checksums: string;
  releasePage: string;
  allReleases: string;
}

const REPO = 'Quo0/ComfyUI-portable-organizer';
const RELEASES = `https://github.com/${REPO}/releases`;

const FALLBACK: DownloadInfo = {
  available: false,
  version: '',
  publishedAt: '',
  url: RELEASES,
  name: '',
  size: 0,
  checksums: '',
  releasePage: RELEASES,
  allReleases: RELEASES,
};

export default {
  async load(): Promise<DownloadInfo> {
    try {
      // `releases/latest` skips prereleases on its own: a tag with a hyphen
      // never reaches the download button.
      const response = await fetch(`https://api.github.com/repos/${REPO}/releases/latest`, {
        headers: { accept: 'application/vnd.github+json' },
      });
      if (!response.ok) return FALLBACK;

      const release = (await response.json()) as {
        tag_name?: string;
        published_at?: string;
        html_url?: string;
        assets?: { name: string; size: number; browser_download_url: string }[];
      };

      const assets = release.assets ?? [];
      // The installer, not the first asset that comes up: `.sig`,
      // `latest.json` and the checksum file sit right next to it.
      const installer = assets.find((a) => a.name.endsWith('-setup.exe'));
      if (!installer) return FALLBACK;

      return {
        available: true,
        version: (release.tag_name ?? '').replace(/^v/, ''),
        publishedAt: release.published_at ?? '',
        url: installer.browser_download_url,
        name: installer.name,
        size: installer.size,
        checksums:
          assets.find((a) => a.name === 'SHA256SUMS.txt')?.browser_download_url ?? '',
        releasePage: release.html_url ?? RELEASES,
        allReleases: RELEASES,
      };
    } catch {
      return FALLBACK;
    }
  },
};
