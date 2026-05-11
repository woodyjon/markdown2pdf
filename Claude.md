A lightweight open-source project for converting markdown to PDF. Three entry points share a single Rust engine:

- Web playground (SvelteKit static site, deployed to Firebase Hosting)
- CLI binary (`markdown2pdf`, distributed via GitHub Releases)
- Claude skill (`skills/markdown2pdf/`)

Technical specs are in the specs-technical.md file.
Functional specs are in the specs-functional.md file.
You must refer to those files.
When you code and make modifications, update those 2 files if required, but make sure you stay true to the specs specified by the files and the user.
If you need to implement something that does not follow the current technical specs, ask for confirmation first.

User interface must be in english.

The development of the app must follow the specs in the specs-functional.md file. But if you need to develop scripts for specific uses, use python, in the scripts/ folder, using a venv in that folder.

Always double check what you do.

Do not create additional documentation files when I don't ask you to. Doc content is authored as plain markdown in `src/lib/docs/*.md` — adding a doc page = adding a `.md` file there and one entry to `src/lib/docs/index.ts`.

Use bun and not npm.

Add in readme how to setup, develop, deploy the app. Update when needed.

When testing in Chrome:
- First run `bun run dev` to start the development server (Vite on http://localhost:5173)
- The WASM module must be built first: `bun run build:wasm`

Avoid these Svelte / SvelteKit pitfalls:
- Avoid calling `fetch` eagerly during server-side rendering — put `fetch` calls inside `onMount` or a `load` function instead.
- Use `$env/dynamic/public` (not `$env/static/public`) for environment variables, so values resolve at runtime and the same `build/` can ship to multiple hosts.
- The site uses `@sveltejs/adapter-static` — do NOT add `+page.server.ts` or other server-only code without first switching adapters and updating the spec.

Deployment is to Firebase Hosting. The site is purely static (no Node runtime, no Docker). `./deploy.sh` runs `build:wasm` + `build` + `firebase deploy --only hosting`.
