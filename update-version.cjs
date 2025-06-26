const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// 1. 更新 package.json
const pkgPath = path.resolve(__dirname, 'package.json');
const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
const oldVersion = pkg.version;
const [major, minor, patch] = oldVersion.split('.').map(Number);
let newPatch = patch < 9 ? patch + 1 : 9;
const newVersion = [major, minor, newPatch].join('.');
pkg.version = newVersion;
fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
console.log(`package.json: ${oldVersion} -> ${newVersion}`);

// 2. 更新 src-tauri/tauri.conf.json
const tauriConfPath = path.resolve(__dirname, 'src-tauri/tauri.conf.json');
if (fs.existsSync(tauriConfPath)) {
  const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
  if (tauriConf.version) {
    const old = tauriConf.version;
    tauriConf.version = newVersion;
    fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2) + '\n');
    console.log(`tauri.conf.json: ${old} -> ${newVersion}`);
  }
}

// 3. 更新 src-tauri/config.json
const configJsonPath = path.resolve(__dirname, 'src-tauri/config.json');
if (fs.existsSync(configJsonPath)) {
  const configJson = JSON.parse(fs.readFileSync(configJsonPath, 'utf8'));
  let updated = false;
  if (configJson.dev && configJson.dev.version) {
    const old = configJson.dev.version;
    configJson.dev.version = newVersion;
    console.log(`config.json(dev): ${old} -> ${newVersion}`);
    updated = true;
  }
  if (configJson.prod && configJson.prod.version) {
    const old = configJson.prod.version;
    configJson.prod.version = newVersion;
    console.log(`config.json(prod): ${old} -> ${newVersion}`);
    updated = true;
  }
  if (updated) {
    fs.writeFileSync(configJsonPath, JSON.stringify(configJson, null, 2) + '\n');
  }
}

// 4. 更新 src-tauri/Cargo.toml
const cargoTomlPath = path.resolve(__dirname, 'src-tauri/Cargo.toml');
if (fs.existsSync(cargoTomlPath)) {
  let content = fs.readFileSync(cargoTomlPath, 'utf8');
  content = content.replace(
    /version\s*=\s*"(.*?)"/,
    `version = "${newVersion}"`
  );
  fs.writeFileSync(cargoTomlPath, content);
  console.log(`Cargo.toml: version -> ${newVersion}`);
}

// 5. 自动更新 Cargo.lock
const srcTauriPath = path.resolve(__dirname, 'src-tauri');
if (fs.existsSync(path.join(srcTauriPath, 'Cargo.toml'))) {
  try {
    execSync('cargo check', { cwd: srcTauriPath, stdio: 'inherit' });
    console.log('Cargo.lock 已自动更新');
  } catch (e) {
    console.log('Cargo.lock 更新失败，请手动运行: cd src-tauri && cargo check');
  }
}

// 6.自动只把以上文件git add
execSync('git add package.json src-tauri/tauri.conf.json src-tauri/config.json src-tauri/Cargo.toml src-tauri/Cargo.lock', { stdio: 'inherit' });

console.log('版本号已全部同步更新！, 并且自动提交到git啦uwu');
console.log('注意：config.json 变更后需重新构建 tauri 项目，二进制才会生效！'); 