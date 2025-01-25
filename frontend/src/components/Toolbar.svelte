<script lang="ts">
	  import { languageList } from '$lib/languages';

    export let language: string;
    export let onLanguageChange: (newLanguage: string) => void;
    export let onUpload: () => void;

    let button: HTMLButtonElement;
    let languages: string[] = languageList;

    export function setText(newText: string) {
      button.innerText = newText;
    };
  </script>
  
  <div class="toolbar">
    <div class="language">
      <select id="language" bind:value={language} on:change={(e: any) => {onLanguageChange(e.target.value)}}>
        {#each languages as lang}
          <option value={lang.toLowerCase()} selected={lang.toLowerCase() === language.toLowerCase()}>{lang}</option>
        {/each}
      </select>
    </div>

    <dv class="save">
      <button bind:this={button} on:click={onUpload}>save</button>
    </dv>
  </div>
  
  <style>
    .toolbar {
      position: absolute;
      bottom: 0px;
      left: 50%;
      transform: translateX(-50%);
      display: flex;
      justify-content: space-between;
      z-index: 1;
      color: white;
    }

    .language {
      display: flex;
      align-items: center;
      font-family: 'Jetbrains Mono', monospace;
      font-weight: bold;

      background-color: rgba(10, 10, 10, 0.4);

      color: white;
    }

    .language:hover {
      cursor: pointer;
      background-color: rgba(10, 10, 10, 0.6);
    }

    #language {
      border: none !important;
      background-color: transparent;

      color: white;
      font-family: 'Jetbrains Mono', monospace;
      padding: 10px;
      margin-right: 10px;

      outline: none !important;
    }

    #language:hover {
      cursor: pointer;
    }

    #language::-ms-expand {
      display: none;
    }

    button {
      background-color: rgba(10, 10, 10, 0.4);
      border: none;
      color: white;
      font-family: 'Jetbrains Mono', monospace;
      padding: 10px;
      margin-left: 10px;
      outline: none;
      height: 100%;
    }

    button:hover {
      cursor: pointer;
      background-color: rgba(10, 10, 10, 0.6);
    }

    select {
      color: initial;
    }

  </style>
  