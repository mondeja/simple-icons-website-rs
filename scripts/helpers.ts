import fs from 'node:fs/promises';
import path from 'path';
import process from 'node:process';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export const getGithubToken = async () => {
  if (process.env.GITHUB_TOKEN) {
    return process.env.GITHUB_TOKEN;
  }

  const envFilePath = path.resolve(__dirname, '..', '.env');
  const envFileExists = await fs
    .access(envFilePath)
    .then(() => true)
    .catch(() => false);
  if (!envFileExists) {
    throw new Error(`.env file not found at ${envFilePath}`);
  }

  const envFile = await fs.readFile(envFilePath, 'utf8');
  for (const line of envFile.split('\n')) {
    const [key, value] = line.split('=');
    if (key.trim() === 'GITHUB_TOKEN') {
      return value.split('"')[1].trim();
    }
  }
  throw new Error(`GITHUB_TOKEN not found in .env file at ${envFilePath}`);
};
