import fs from 'node:fs/promises';

const sitemapAssetPath = 'app/public/assets/sitemap.xml';
const sitemapAssetExists = await fs
  .access(sitemapAssetPath)
  .then(() => true)
  .catch(() => false);
if (sitemapAssetExists) {
  await fs.unlink(sitemapAssetPath);
}

const protocol = 'https';
const currentDate = new Date().toISOString();
const domain = 'simpleicons.org';

await fs.writeFile(
  sitemapAssetPath,
  `<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"` +
    ` xmlns:xhtml="http://www.w3.org/1999/xhtml">\n` +
    `  <url>\n` +
    `    <loc>${protocol}://${domain}/</loc>\n` +
    `    <lastmod>${currentDate}</lastmod>\n` +
    `    <changefreq>weekly</changefreq>\n` +
    `  </url>\n` +
    `  <url>\n` +
    `    <loc>${protocol}://${domain}/preview</loc>\n` +
    `    <lastmod>${currentDate}</lastmod>\n` +
    `    <changefreq>weekly</changefreq>\n` +
    `  </url>\n` +
    `</urlset>`,
);
