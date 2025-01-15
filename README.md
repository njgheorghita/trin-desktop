# Tauri + Vue 3

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Dependencies...

- install trin client [instructions](https://ethereum.github.io/trin/introduction/quickstart.html)
  - move the trin client `executable` to the `src-tauri` directory
    - the `executable` needs to be renamed to include your systems `-$TARGET_TRIPLE` suffix. 
    - instructions to rename the `executable` with your system's suffix are [here](https://v2.tauri.app/develop/sidecar/)
- use node V20 or lower to handle tailwind config CJS modules
- rustup needs to be installed
  - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

## Dev...

- `npm install`
- `npm run tauri dev`
- `npm run lint` (optional)
- `https://www.shadcn-vue.com/docs/introduction.html` for all the js components

## Releasing...

- update the version in `tauri.conf.json` & merge to master
- tag latest commit with the version number
- `git tag app-v*.*.*`
- `git push origin app-v*.*.*`

# Helpful info

- **Do not run trin client at the same times as trin-desktop**
- [Trin supported JSON RPC methods](https://github.com/ethereum/trin/blob/master/ethportal-api/src/eth.rs)
