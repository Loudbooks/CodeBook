<script lang="ts">
	import Loading from "../../../components/Loading.svelte";
	import MonacoEditor from "../../../components/MonacoEditor.svelte";

    export let data;
	const { metadata, content } = data;

    let language: string;
    let contentDataString: string;

    metadata.then((metadataData) => {
        language = metadataData.language;
    });

    content.then((contentData) => {
        contentDataString = contentData;
    });
</script>

<div>
    {#await Promise.all([metadata, content]) }
        <Loading />
    {:then [metadataData, contentData]}
        <MonacoEditor content={contentDataString} language={language} />
    {:catch error}
        <p>{error.message}</p>
    {/await}
</div>

<style lang="scss">
	#editor-container {
		width: 100vw;
		height: 100vh;

		z-index: 1;

		position: absolute;
	}
</style>