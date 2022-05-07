<script>
    let games;

    fetch('https://ghs-games.api.mattglei.ch/current-week', {mode: 'cors'})
        .then((r) => r.json())
        .then((r) => {
            games = r
        })
</script>

<main>
    {#if games == undefined}
        <p>Loading</p>
    {:else}
        {games["data"][0]["name"]}
    {/if}
</main>

