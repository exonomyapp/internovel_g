#!/usr/bin/env node
import { execSync } from 'node:child_process';

const checks = [
  {
    name: 'willow_gap_core wasm32 isolation',
    cmd: 'cargo tree -p willow_gap_core --target wasm32-unknown-unknown --no-default-features',
    forbidden: ['fjall', 'lsm-tree', 'async-fs', 'mio'],
  },
  {
    name: 'internovel_auth wasm32 isolation',
    cmd: 'cargo tree -p internovel_auth --target wasm32-unknown-unknown',
    forbidden: ['fjall', 'tokio-uring', 'mio'],
  },
];

for (const check of checks) {
  console.log(`Checking ${check.name}...`);
  const tree = execSync(check.cmd, { encoding: 'utf8' });
  for (const banned of check.forbidden) {
    if (tree.includes(banned)) {
      console.error(`FAIL: ${check.name} pulled forbidden native crate "${banned}" into wasm32!`);
      process.exit(1);
    }
  }
  console.log(`PASS: ${check.name}`);
}
