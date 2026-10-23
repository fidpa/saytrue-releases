# Third-Party Licenses

Saytrue includes code adapted from the MIT-licensed projects below and ships open-source components under the Mozilla Public License 2.0 (see [MPL-2.0 components](#mpl-20-components)). These components are licensed to you under their own terms; the Saytrue End-User License Agreement does not restrict them ([EULA](https://saytrue.de/en/eula), § 15).

---

## cjpais/handy

**Repository:** https://github.com/cjpais/handy
**License:** MIT License
**Copyright:** (c) 2025 CJ Pais

**Used in:**
- `src-tauri/src/native_audio/recorder.rs` - Audio capture architecture
- `src-tauri/src/native_audio/resampler.rs` - Audio resampling (rubato FFT)
- `src-tauri/src/native_audio/device.rs` - cpal device handling
- `src-tauri/src/native_audio/mod.rs` - Module structure
- `src-tauri/src/text.rs` - Text filtering (filler words, stutter)
- `src-tauri/src/vad/pipeline.rs` - VAD threshold settings

```
MIT License

Copyright (c) 2025 CJ Pais

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## run-llama/chat-ui (@llamaindex/chat-ui)

**Repository:** https://github.com/run-llama/chat-ui
**License:** MIT License
**Copyright:** (c) 2025 LlamaIndex

**Used in:**
- `src/components/ChatInput.tsx` - Chat input UI pattern (IME support, auto-resize)

```
MIT License

Copyright (c) 2025 LlamaIndex

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## MPL-2.0 components

The following components are licensed under the [Mozilla Public License 2.0](https://mozilla.org/MPL/2.0/). You can obtain their Source Code Form at the locations below, free of charge.

| Component | Version | Modified | Source Code Form | Used for |
|-----------|---------|----------|------------------|----------|
| rusty_foundationmodels | 0.2.0 | **yes** | [`third-party/rusty_foundationmodels/`](../../third-party/rusty_foundationmodels/) in this repository (changes: [MODIFICATIONS.md](../../third-party/rusty_foundationmodels/MODIFICATIONS.md)) | Apple Intelligence on-device model (macOS and iPhone, Apple Silicon) |
| cssparser | 0.29.6 | no | https://crates.io/crates/cssparser/0.29.6 | via Tauri |
| cssparser | 0.36.0 | no | https://crates.io/crates/cssparser/0.36.0 | via Tauri |
| cssparser-macros | 0.6.1 | no | https://crates.io/crates/cssparser-macros/0.6.1 | via Tauri |
| dtoa-short | 0.3.5 | no | https://crates.io/crates/dtoa-short/0.3.5 | via Tauri |
| selectors | 0.24.0 | no | https://crates.io/crates/selectors/0.24.0 | via Tauri |
| selectors | 0.36.1 | no | https://crates.io/crates/selectors/0.36.1 | via Tauri |
| option-ext | 0.2.0 | no | https://crates.io/crates/option-ext/0.2.0 | via `dirs` |

Dual-licensed components are used under their non-copyleft option: DOMPurify (MPL-2.0 or Apache-2.0) under Apache-2.0, JSZip (MIT or GPL-3.0-or-later) under MIT, r-efi (MIT, Apache-2.0 or LGPL-2.1-or-later) under MIT, self_cell (Apache-2.0 or GPL-2.0-only) under Apache-2.0.

Questions about the source code of these components: https://github.com/fidpa/saytrue-releases/issues or marc@saytrue.de

---

## Note

All other dependencies are standard npm and Cargo packages under permissive licenses (MIT, Apache-2.0, BSD, ISC and similar).
