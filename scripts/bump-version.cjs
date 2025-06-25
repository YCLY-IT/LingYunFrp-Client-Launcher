const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// 1. 递增 package.json 版本号
const pkgPath = path.resolve(__dirname, '../package.json');
const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
const [major, minor, patch] = pkg.version.split('.').map(Number);
pkg.version = [major, minor, patch + 1].join('.');
fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2));

// 2. 同步到 src-tauri/tauri.conf.json
const tauriConfPath = path.resolve(__dirname, '../src-tauri/tauri.conf.json');
if (fs.existsSync(tauriConfPath)) {
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
  if (tauriConf.version !== undefined) {
    tauriConf.version = pkg.version;
    fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2));
  }
}

console.log('版本号已自动递增为', pkg.version);

// 3. 自动提交版本号变动
try {
  console.log('正在自动提交版本号变动...');
  execSync('git add package.json src-tauri/tauri.conf.json', { stdio: 'inherit' });
  execSync(`git commit -m "chore: bump version to ${pkg.version}"`, { stdio: 'inherit' });
  console.log('版本号变动已自动提交！');
} catch (error) {
  console.log('自动提交失败，请手动提交版本号变动：');
  console.log('git add package.json src-tauri/tauri.conf.json');
  console.log(`git commit -m "chore: bump version to ${pkg.version}"`);
} 