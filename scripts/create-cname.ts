import fs from 'node:fs/promises';

const cnameAssetPath = 'app/public/assets/CNAME';
const cnameAssetExists = await fs
  .access(cnameAssetPath)
  .then(() => true)
  .catch(() => false);
if (cnameAssetExists) {
  await fs.unlink(cnameAssetPath);
}

await fs.writeFile(cnameAssetPath, 'simpleicons.org');
