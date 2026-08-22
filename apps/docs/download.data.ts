// Сведения о последнем выпуске, добытые на этапе сборки сайта.
//
// Загрузчик данных VitePress: выполняется один раз при сборке, результат
// вмерзает в статику. На странице скачивания не остаётся ни одного сетевого
// запроса — ни к GitHub, ни к чему-либо ещё.
//
// **Сборка сайта не должна падать из-за GitHub.** Недоступный API,
// исчерпанный лимит запросов, отсутствие релизов вовсе — всё это штатные
// состояния, и на каждое загрузчик отдаёт запасной вариант: ссылку
// на страницу релизов без подробностей. Сайт публикуется всегда.

export interface DownloadInfo {
  /** Есть ли настоящий релиз. `false` — показываем только ссылку на список. */
  available: boolean;
  version: string;
  /** ISO-дата публикации, форматируется на странице. */
  publishedAt: string;
  /** Прямая ссылка на инсталлятор либо на страницу релизов. */
  url: string;
  /** Имя файла инсталлятора. */
  name: string;
  /** Размер в байтах. */
  size: number;
  /** Ссылка на SHA256SUMS.txt, если он есть в релизе. */
  checksums: string;
  /** Страница релиза целиком — она же запасной вариант. */
  releasePage: string;
  /** Все версии. */
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
      // `releases/latest` сам пропускает предрелизы: тег с дефисом
      // на кнопку скачивания не попадает.
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
      // Инсталлятор, а не первый попавшийся ассет: рядом лежат `.sig`,
      // `latest.json` и файл сумм, и любой из них скачался бы вместо него.
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
      // Сети нет вовсе — например, сборка идёт локально в самолёте.
      return FALLBACK;
    }
  },
};
