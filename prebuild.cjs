// tauri build の beforeBuildCommand から呼ばれる、OS 共通のプレビルドスクリプト。
// Windows では PowerShell スクリプト、Mac/Linux では bash スクリプトを呼び分ける。

const { execSync } = require('child_process');

const isWin = process.platform === 'win32';
const backendBuild = isWin
  ? 'powershell -NoProfile -ExecutionPolicy Bypass -File build-backend.ps1'
  : 'bash build-backend.sh';

console.log('> ' + backendBuild);
execSync(backendBuild, { stdio: 'inherit' });

console.log('> npm run build');
execSync('npm run build', { stdio: 'inherit' });
