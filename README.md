# Blendio Tauri (ENG)

This is a Tauri app called "Blendio" that was developed for a [bachelor thesis](https://github.com/JJeris/bakalaura_darba_praktiska_dala). It contains basic functionality for managing Blender 3D versions and their project files, allowing the user to specify launch arguments before opening files, as well as choosing to delete .blend files or Blender versions themselves.

It was inspired by [Blenderbase](https://github.com/PhysicalAddons/blenderbase-public) and [Blender Launcher](https://github.com/Victor-IX/Blender-Launcher-V2) projects.

## Prerequisites

The project was developed for the Windows platform. Prerequisites to set up the project are:

- Download and install [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/),
- Download and install [Rust development dependencies](https://www.rust-lang.org/tools/install),
- Download and install [Node.js](https://nodejs.org/en),
- Download and install [NPM](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm),
- Set up an IDE that can use [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer), preferably [Visual Studio Code](https://code.visualstudio.com/).

## Commands

The main commands for installing the dependencies, running in development mode and building the app are listed below.

```ps
cd blendio-tauri        
npm install                     // Install Node.js dependencies.
npm run tauri dev               // Run development mode.
npm run tauri build             // Build the project.
npm run tauri build -- --debug  // Build the project in debug mode.
```

# Blendio Tauri (LV)

Šī ir Tauri lietojumprogramma "Blendio", kas tika izstrādāta [bakalaura darba](https://github.com/JJeris/bakalaura_darba_praktiska_dala) ietvarā. Tā satur pamatfunkcionalitāti Blender 3D versiju un to projektu failu pārvaldībai, atļaujot lietotājam norādīt komandrindas parametrus pirms failu atvēršanas, kā arī ļauj izvēlēties izdzēst .blend failus vai pašas Blender versijas.

Tās ideju iedvesmoja [Blenderbase](https://github.com/PhysicalAddons/blenderbase-public) un [Blender Launcher](https://github.com/Victor-IX/Blender-Launcher-V2) projekti.

## Priekšnosacījumi

Projekts tika izstrādāts un ir paredzēts darbībai Windows platformā. Priekšnosacījumi, lai uzstādītu projektu, ir:

- Lejupielādēt un instalēt  [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/),
- Lejupielādēt un instalēt [Rust izstrādes atkarības](https://www.rust-lang.org/tools/install),
- Lejupielādēt un instalēt [Node.js](https://nodejs.org/en),
- Lejupielādēt un instalēt [NPM](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm),
- Uzstādīt IDE, kas spēj izmantot [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer), piemēram, [Visual Studio Code](https://code.visualstudio.com/).

## Komandrindas pavēles

Zemāk ir redzamas galvenās komandrindas pavēles atkarību instalācijai, projekta iedarbināšanu izstrādes režīmā un projekta sakomplektēšanas.

```ps
cd blendio-tauri        
npm install                     // Instalē Node.js atkarības
npm run tauri dev               // Iedarbina izstrādes režīmā.
npm run tauri build             // Sakomplektē projektu.
npm run tauri build -- --debug  // Sakomplektē projektu atkļūdošanas režīmā.
```
