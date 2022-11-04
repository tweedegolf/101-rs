
<!--
This theme adds a link to the Rust playground to code snippets
  Usage:
```md
---
layout: playground
---
```rust
// Your code here
```
-->

<script setup lang="ts">
import { onBeforeMount, onMounted, watchEffect } from 'vue';

const props = defineProps({
  class: {
    type: String,
  },
});

const fetchWithTimeout = (url, options, timeout = 6000) => {
  return Promise.race([
    fetch(url, options),
    new Promise((_, reject) => setTimeout(() => reject(new Error('timeout')), timeout))
  ]) as Promise<Response>;
}

const fetchAvailableCrates = () =>
  fetchWithTimeout("https://play.rust-lang.org/meta/crates", {
    headers: {
      'Content-Type': "application/json",
    },
    method: 'POST',
    mode: 'cors',
  })
    .then(response => response.json()).then(response => response.crates.map(item => item["id"]));

const removePlaygroundButtons = () => {
  const buttons = document.querySelectorAll('button.playground-run,a.playground-open');
  Array.from(buttons).forEach(button => {
    button.parentNode?.removeChild(button);
  });
}

const allCratesAvailable = (code: string, playgroundCrates: string[]) => {
  // get list of `extern crate`'s from snippet
  const re = /extern\s+crate\s+([a-zA-Z_0-9]+)\s*;/g;
  const externCrates: string[] = [];
  while (true) {
    const res = re.exec(code);
    if (!res) {
      break;
    }
    externCrates.push(res[1]);
  }

  return externCrates.every((item) => playgroundCrates.includes(item));
}

const renderPlaygroundButtons = (playgroundCrates: string[]) => {
  const codeElements = document.querySelectorAll("pre.shiki-container .shiki-light");
  console.log(codeElements);
  Array.from(codeElements).forEach(codeElement => {
    const container = codeElement.closest(".slidev-code-wrapper");
    if (container) {
      const lineElements = Array.from(codeElement.querySelectorAll("span.line"));
      const code = lineElements.map(line => line.textContent).join("\n");

      const playgroundUrl = "https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=" + encodeURIComponent(code);
      const btnEdit = document.createElement("a");
      btnEdit.classList.add("playground-open");
      btnEdit.innerHTML = "🖉 Edit"
      btnEdit.setAttribute("target", "_blank");
      btnEdit.setAttribute("href", playgroundUrl);
      container.insertBefore(btnEdit, container.firstChild);
      if (!allCratesAvailable(code, playgroundCrates)) {        
        console.warn('Code block uses unavailable extern crates');
        return;
      }

      const btnRun = document.createElement("button");
      btnRun.classList.add("playground-run")
      btnRun.innerHTML = "▶ Run";
      
      const params = {
        version: "stable",
        optimize: "0",
        code,
        edition: "2021"
      };

      btnRun.onclick = () => {
        const resultBlock = document.createElement('pre');
        resultBlock.innerText = '<loading...>';
        container.appendChild(resultBlock);
        fetchWithTimeout("https://play.rust-lang.org/evaluate.json", {
          headers: {
            'Content-Type': "application/json",
          },
          method: 'POST',
          mode: 'cors',
          body: JSON.stringify(params)
        })
          .then(response => response.json())
          .then(response => {
            

            const resultContent = response.result.trim() === '' ? '<No output>' : response.result;
            resultBlock.innerText = resultContent;

          });
      }
      
      container.insertBefore(btnRun, container.firstChild);
    }
  });
}

(async () => {
  console.clear();
  console.log("playground");
  const availableCrates = await fetchAvailableCrates();
  await removePlaygroundButtons();
  await renderPlaygroundButtons(availableCrates);
})();
</script>

<template>
  <div class="slidev-layout">
    <div :class="props.class">
      <slot />
    </div>
  </div>
</template>
