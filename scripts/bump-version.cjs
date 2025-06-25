const fs = require('fs');
const path = require('path');

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

// 3. 同步到 src-tauri/Cargo.toml
const cargoTomlPath = path.resolve(__dirname, '../src-tauri/Cargo.toml');
if (fs.existsSync(cargoTomlPath)) {
  let cargoContent = fs.readFileSync(cargoTomlPath, 'utf8');
  // 使用正则表达式更新版本号
  cargoContent = cargoContent.replace(/version = "[\d.]+"/, `version = "${pkg.version}"`);
  fs.writeFileSync(cargoTomlPath, cargoContent);
}

console.log('版本号已自动递增为', pkg.version);
console.log('请手动提交版本号变动：');
console.log('git add package.json src-tauri/tauri.conf.json src-tauri/Cargo.toml src-tauri/Cargo.lock');
console.log(`git commit -m "chore: bump version to ${pkg.version}"`); 