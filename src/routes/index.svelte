<script>
    import { onMount } from 'svelte';
    import { fly, fade } from 'svelte/transition';
    import dayjs from 'dayjs';

    let games;

    onMount(() => {
        fetch('https://ghs-games.api.mattglei.ch/current-week', {mode: 'cors'})
            .then((r) => r.json())
            .then((r) => {
                games = r;

                let firstGame = games["data"][0];
                name = firstGame["name"];
                time = dayjs(firstGame["time"]);
                location = firstGame["location"];
            });

        const interval = setInterval(() => {
            if (games == undefined) {
                return;
            }
            let data = games["data"];
            index++;
            if (data.length == index) {
                index = 0;
            }
            let game = data[index];
            name = game["name"];
            time = dayjs(game["time"]);
            location = game["location"];
        }, 5000);
    })

    let index = 0;
    let name;
    let time;
    let location;
</script>

<main>
	{#if games == undefined}
		<p>Loading data from custom proxy</p>
	{:else}
        {#key index}
            <h1 in:fly="{{ y: -100,  duration: 2000}}">{name}</h1>
        {/key}
        <h2 in:fade><span class="highlight">{time.format('h:mm A')}</span> @ {location}</h2>
	{/if}
	<p class="copyright">© Matt Gleich 2022 - mattglei.ch</p>
</main>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500&display=swap');

	main > * {
		margin: 0;
	}

	main {
		font-family: 'Inter';
		display: flex;
		align-items: center;
		justify-content: center;
		flex-direction: column;
		height: 100vh;
		background: black;
		color: white;
	}

	h2 {
		font-size: 5vh;
	}

	h1 {
		font-size: 100px;
	}

	.copyright {
		position: absolute;
		left: 10px;
		bottom: 5px;
	}

	.highlight {
		background: white;
		color: black;
	}
</style>
