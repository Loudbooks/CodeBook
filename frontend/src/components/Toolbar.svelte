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
      backdrop-filter: blur(10px);

      color: white;
    }

    .language:hover {
      cursor: pointer;
      background-color: rgba(10, 10, 10, 0.6);
    }

    #language {
      background: url(data:image/svg+xml;base64,PHN2ZyBkYXRhLW5hbWU9IkxheWVyIDEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDQuOTUgMTAiPjxwYXRoIHN0eWxlPSJmaWxsOnRyYW5zcGFyZW50IiBkPSJNMCAwaDQuOTV2MTBIMHoiLz48cGF0aCBkPSJtMS40MSA0LjY3IDEuMDctMS40OSAxLjA2IDEuNDl6bTIuMTMuNjZMMi40OCA2LjgyIDEuNDEgNS4zM3oiIHN0eWxlPSJmaWxsOiNmZmYiLz48L3N2Zz4=
      ) no-repeat 95% 50%;      
      -moz-appearance: none; 
      -webkit-appearance: none; 
      appearance: none;

      border: none !important;
      background-color: transparent;

      color: white;
      font-family: 'Jetbrains Mono', monospace;
      padding: 10px;
      margin-right: 1px;

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
      backdrop-filter: blur(10px);
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
  