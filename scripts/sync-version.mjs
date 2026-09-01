#!/usr/bin/env node

import { readFileSync, writeFileSync } from 'node:fs';

const VERSION_PATTERN = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;
const FILES = {
  packageJson: 'package.json',
  cargoToml: 'src-tauri/Cargo.toml',
  cargoLock: 'src-tauri/Cargo.lock',
  tauriConfig: 'src-tauri/tauri.conf.json',
};

function readText(path) {
  return readFileSync(path, 'utf-8');
}

function writeText(path, content) {
  writeFileSync(path, content);
}

function readJson(path) {
  return JSON.parse(readText(path));
}

function writeJson(path, value) {
  writeText(path, `${JSON.stringify(value, null, 2)}\n`);
}

function findCargoPackageVersion(content) {
  const lines = content.split('\n');
  let inPackage = false;

  for (const line of lines) {
    const trimmed = line.trim();
    if (trimmed === '[package]') {
      inPackage = true;
      continue;
    }
    if (inPackage && trimmed.startsWith('[')) {
      break;
    }

    if (inPackage) {
      const match = line.match(/^version\s*=\s*"([^"]+)"/);
      if (match) {
        return match[1];
      }
    }
  }

  throw new Error(`${FILES.cargoToml} does not contain a [package] version`);
}

function updateCargoPackageVersion(content, version) {
  const lines = content.split('\n');
  let inPackage = false;
  let updated = false;

  const updatedLines = lines.map((line) => {
    const trimmed = line.trim();
    if (trimmed === '[package]') {
      inPackage = true;
      return line;
    }
    if (inPackage && trimmed.startsWith('[')) {
      inPackage = false;
      return line;
    }

    if (inPackage && /^version\s*=/.test(line)) {
      updated = true;
      return `version = "${version}"`;
    }

    return line;
  });

  if (!updated) {
    throw new Error(`${FILES.cargoToml} does not contain a [package] version`);
  }

  return updatedLines.join('\n');
}

function findCargoLockPackageVersion(content, packageName) {
  const blocks = content.match(/^\[\[package\]\]\n(?:(?!^\[\[package\]\]).*\n?)*/gm) ?? [];

  for (const block of blocks) {
    if (block.includes(`name = "${packageName}"`)) {
      const match = block.match(/^version\s*=\s*"([^"]+)"/m);
      if (match) {
        return match[1];
      }
    }
  }

  return null;
}

function updateCargoLockPackageVersion(content, packageName, version) {
  let updated = false;

  return content.replace(/^\[\[package\]\]\n(?:(?!^\[\[package\]\]).*\n?)*/gm, (block) => {
    if (!block.includes(`name = "${packageName}"`)) {
      return block;
    }

    updated = true;
    return block.replace(/^version\s*=\s*"[^"]+"/m, `version = "${version}"`);
  }).replace(/$/, () => {
    if (!updated) {
      throw new Error(`${FILES.cargoLock} does not contain package "timeflow"`);
    }
    return '';
  });
}

function readVersions() {
  const packageJson = readJson(FILES.packageJson);
  const tauriConfig = readJson(FILES.tauriConfig);
  const cargoToml = readText(FILES.cargoToml);
  const cargoLock = readText(FILES.cargoLock);

  return {
    [FILES.packageJson]: packageJson.version,
    [FILES.cargoToml]: findCargoPackageVersion(cargoToml),
    [FILES.tauriConfig]: tauriConfig.version,
    [FILES.cargoLock]: findCargoLockPackageVersion(cargoLock, packageJson.name),
  };
}

function checkVersions() {
  const versions = readVersions();
  const manifestVersions = [
    versions[FILES.packageJson],
    versions[FILES.cargoToml],
    versions[FILES.tauriConfig],
  ];
  const expected = manifestVersions[0];
  const inSync = manifestVersions.every((version) => version === expected);

  for (const [path, version] of Object.entries(versions)) {
    if (version) {
      console.log(`${path}: ${version}`);
    }
  }

  if (!inSync) {
    console.error('Version mismatch: package.json, src-tauri/Cargo.toml, and src-tauri/tauri.conf.json must match.');
    process.exit(1);
  }

  if (versions[FILES.cargoLock] && versions[FILES.cargoLock] !== expected) {
    console.error(`${FILES.cargoLock} is out of sync with ${FILES.cargoToml}. Run: make version-set VERSION=${expected}`);
    process.exit(1);
  }
}

function setVersions(version) {
  if (!VERSION_PATTERN.test(version)) {
    console.error(`Invalid VERSION "${version}". Expected semantic version format like 1.2.3.`);
    process.exit(2);
  }

  const packageJson = readJson(FILES.packageJson);
  packageJson.version = version;
  writeJson(FILES.packageJson, packageJson);

  const tauriConfig = readJson(FILES.tauriConfig);
  tauriConfig.version = version;
  writeJson(FILES.tauriConfig, tauriConfig);

  writeText(FILES.cargoToml, updateCargoPackageVersion(readText(FILES.cargoToml), version));
  writeText(FILES.cargoLock, updateCargoLockPackageVersion(readText(FILES.cargoLock), packageJson.name, version));

  checkVersions();
}

const command = process.argv[2];
const version = process.argv[3] ?? process.env.VERSION;

if (command === 'check') {
  checkVersions();
} else if (command === 'set') {
  setVersions(version ?? '');
} else {
  console.error('Usage: sync-version.mjs check | sync-version.mjs set <version>');
  process.exit(2);
}
