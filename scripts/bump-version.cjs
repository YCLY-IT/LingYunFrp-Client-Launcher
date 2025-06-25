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
console.log('版本号已自动递增为', pkg.version); 