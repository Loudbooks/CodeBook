<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type * as Monaco from 'monaco-editor/esm/vs/editor/editor.api';
	import Toolbar from './Toolbar.svelte';
	import { backendPort } from '$lib/stores';
  import hljs from 'highlight.js';
  import { languageList } from '$lib/languages';

	export let content: string = "";
	export let theme: string = "Darcula";
	export let language: string = 'plaintext';

	let editor: Monaco.editor.IStandaloneCodeEditor;
	let monaco: typeof Monaco;
	let editorContainer: HTMLElement;
	let uploading = false;

  let toolbar: Toolbar;

  onMount(() => {
    window.addEventListener('resize', function() {
      editor.layout();
    });
  })

	const handleLanguageChange = (newLanguage: string) => {
		language = newLanguage.toLowerCase();

		const currentModel = editor.getModel();

		if (currentModel) {
			monaco.editor.setModelLanguage(currentModel, newLanguage);
		}
	};

	const handleSubmit = async () => {
    if (uploading) {
      return;
    }

		const xhr = new XMLHttpRequest();

		let domain = window.location.host;
    let secure = window.location.protocol === 'https:';

		if (domain.includes('localhost')) {
			xhr.open('POST', `http://localhost:${$backendPort}/upload`);
		} else {
			if (domain.match(/192\.168\.\d+\.\d+/)) {
				domain = domain.replace(/:\d+/, '');

				xhr.open('POST', `http://${domain}:${$backendPort}/upload`);
			} else {
				xhr.open('POST', `https://api.${domain}/upload`);
			}
		}

		xhr.setRequestHeader('Content-Type', 'plain/text');
		xhr.setRequestHeader('access-control-allow-methods', 'POST');
    xhr.setRequestHeader('language', language);

		xhr.send(editor.getValue());
		xhr.responseType = 'text';

		uploading = true;
    toolbar.setText('saving...');

		xhr.onload = function () {
			if (xhr.status !== 200) {
				setTimeout(() => {
					toolbar.setText('error: ' + xhr.response);

          uploading = false;
				}, 3000);

				return;
			} else {
        toolbar.setText('copied!');


        setTimeout(() => {
          toolbar.setText('save');
          uploading = false;
        }, 3000);
      }

      if (secure) {
        domain = 'https://' + domain;
      } else {
        domain = 'http://' + domain;
      }

      navigator.clipboard.writeText(domain + '/p/' + xhr.response);
			window.history.replaceState(null, '', domain + '/p/' + xhr.response);
		};
	};

	onMount(async () => {
		monaco = (await import('$lib/monaco')).default;

		const formattedTheme = theme.toLowerCase().replace(' ', '-');
		fetch(`/themes/${theme}.json`)
			.then((data) => data.json())
			.then((data) => {
				monaco.editor.defineTheme(formattedTheme, data);
				monaco.editor.setTheme(formattedTheme);

				editor = monaco.editor.create(editorContainer, {
					fontSize: 16,
					fontFamily: 'JetBrains Mono'
				});

				const model = monaco.editor.createModel(content, language);
				editor.setModel(model);

        editor.onDidPaste(async () => {
          const pastedValue = editor.getValue();

          updateLanguage(pastedValue);
        });

        editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
          handleSubmit();
        });
			});
	});

	onDestroy(() => {
		monaco?.editor.getModels().forEach((model) => model.dispose());
		editor?.dispose();
	});

  async function updateLanguage(newValue: string) {
    const detectedLang = await detectLanguageAsync(newValue);

    if (!detectedLang || !languageList.includes(detectedLang)) {
      monaco.editor.setModelLanguage(editor.getModel()!, 'plaintext');
      language = 'plaintext';
      return;
    }

    if (detectedLang !== language) {
      monaco.editor.setModelLanguage(editor.getModel()!, detectedLang);
      language = detectedLang;
    }
  }

  function detectLanguageAsync(text: string) {
    return new Promise<string>((resolve) => {
      setTimeout(() => {
        const detected = hljs.highlightAuto(text);
        resolve(detected.language || 'plaintext');
      }, 0); 
    });
  }

	export function getContent() {
		return editor?.getValue() || '';
	}
</script>

<div class="monaco-container">
	<Toolbar bind:this={toolbar} {language} onUpload={handleSubmit} onLanguageChange={handleLanguageChange} />

	<div class="container" bind:this={editorContainer}></div>
</div>

<style>
	.container {
		z-index: 0;
		width: 100%;
		height: calc(100vh);
	}

	:global(.monaco-editor) {
		z-index: 0;
		width: 100%;
		height: 100%;
	}
</style>
