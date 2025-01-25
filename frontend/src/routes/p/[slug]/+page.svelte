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
    <Loading />

    {#await Promise.all([metadata, content]) then}
        <MonacoEditor content={contentDataString} language={language} />
    {:catch error}
        <p>{error.message}</p>
    {/await}
</div>