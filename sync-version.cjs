const fs = require('fs');
const path = require('path');

const version = process.argv[2];
if (!version) {
    console.error('請提供版本號，例如：node sync-version.cjs 1.2.3');
    process.exit(1);
}

// 更新 package.json
const pkgPath = path.join(__dirname, 'package.json');
const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8'));
pkg.version = version;
fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2));

// 更新 tauri.conf.json
const tauriConfPath = path.join(__dirname, 'src-tauri', 'tauri.conf.json');
const tauriConf = JSON.parse(fs.readFileSync(tauriConfPath, 'utf8'));
// if (!tauriConf.package) tauriConf.package = {};
tauriConf.version = version;
fs.writeFileSync(tauriConfPath, JSON.stringify(tauriConf, null, 2));

// 更新 Cargo.toml
const cargoTomlPath = path.join(__dirname, 'src-tauri', 'Cargo.toml');
let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
cargoToml = cargoToml.replace(/version\s*=\s*"[0-9]+\.[0-9]+\.[0-9]+"/, `version = "${version}"`);
fs.writeFileSync(cargoTomlPath, cargoToml);

console.log(`版本號已更新為 ${version}`);
