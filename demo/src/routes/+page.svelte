<script>
    import {activate_contract, arbitrate, check_state, create_new_contract, upload_position} from "./transaction.js";
	import Eurotaxisimulator from '$lib/components/Eurotaxisimulator.svelte'
	import arbitrum_logo_src from '$lib/assets/logo_monochrome.svg'
    import stylus_logo_src from '$lib/assets/stylus_white.svg'
    import github_logo_src from '$lib/assets/github-mark-white.svg'

	let polled_status = $state(0);
    let status_message = $derived.by(() => {
        let msgs = ['--', 'CREATED', 'ACTIVATED', 'LOCKED', 'FULFILLED', 'FAILED'];
        return msgs[polled_status];
	})

	let departure_index = $state(1);
    let arrival_index = $state(2);
	let driver_index = $state(0);

    let contract_id = 1;
    let filler_id = $state('');
    const checkpoints = [[8, 8], [22, 14], [28, 36], [38, 43], [55, 55]];
    let points_indexes = $state([3, 4, 5, 6, 7]);

    async function handle_create_contract() {
        await create_new_contract(contract_id, filler_id, 42, checkpoints);
    }

    async function handle_activate_contract() {
		await activate_contract(contract_id, 42);
        departure_index = 0;
        driver_index = departure_index;
        arrival_index = 4095;
        points_indexes = [
            checkpoints[0][0] * 64 + checkpoints[0][1],
            checkpoints[1][0] * 64 + checkpoints[1][1],
            checkpoints[2][0] * 64 + checkpoints[2][1],
            checkpoints[3][0] * 64 + checkpoints[3][1],
            checkpoints[4][0] * 64 + checkpoints[4][1],
        ];
	}

    async function handle_unlock() {
        await arbitrate(contract_id, 1);
        polled_status = 2;
	}
    async function handle_refund() {
		await arbitrate(contract_id, 2);
        polled_status = 5;
    }

    async function handle_upload_position() {
        await upload_position(contract_id, driver_index);
	}

    async function handle_check_state() {
       polled_status = await check_state(contract_id);
	}

    const position_interval = setInterval(handle_upload_position, 1000);
    const state_interval = setInterval(handle_check_state, 5000);

    $effect(() => {
       if (polled_status > 3) {
           clearInterval(position_interval);
           clearInterval(state_interval);
       }
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>
<div class="col-span-2 flex border-r border-b border-inherit">
	<div class="flex gap-4 items-center h-full w-1/2 border-r border-inherit px-14 py-4">
		<a class="h-[90%]" href="https://arbitrum.io">
			<img class="h-full" src={arbitrum_logo_src} alt="ARBITRUM" />
		</a>
		<a class="h-full" href="https://arbitrum.io/stylus">
			<img class="h-full" src={stylus_logo_src} alt="STYLUS" />
		</a>
	</div>
	<div class="flex justify-between items-center px-14 py-4 h-full w-1/2">
		<a class="font-grotesk text-[#737082] text-md" href="https://github.com/Saint-Luc">Saint-Luc</a>
		<img class="h-[40%]" src={github_logo_src} alt="GITHUB" />
		<a class="font-grotesk text-[#737082] text-md" href="https://github.com/QuartzIsNuggets">QuartzIsNuggets</a>
	</div>
</div>
<div class="flex border-b border-inherit">
	<div class="flex justify-center items-center h-full w-1/2 border-r border-inherit px-14 py-4">
		<h1 class="font-grotesk text-[#737082] text-md">Luc Sanglas</h1>
	</div>
	<div class="flex justify-center items-center h-full w-1/2 px-14">
		<h1 class="font-grotesk text-[#737082] text-md">Alexis Ronez</h1>
	</div>
</div>
<div class="row-span-6 col-span-2 flex border-r border-inherit text-[#737082] font-sl py-8">
	<div class="h-full w-1/2 flex flex-col gap-6 px-14">
		<p class="text-4xl font-black">CLIENT</p>
		<div class="flex flex-col gap-3">
			<input class="w-full text-xl font-black outline-0 bg-transparent border-b border-[#737082]" bind:value={filler_id} placeholder="filler ID" />
			<button class="border border-[#737082] w-full px-3 py-1.5 rounded-md" onclick={handle_create_contract}>Issue contract (pre-baked)</button>
		</div>
	</div>
	<div class="h-full w-1/2 flex flex-col gap-6 px-14">
		<p class="text-4xl font-black">DRIVER</p>
		<div class="flex flex-col gap-3">
			<button class="border border-[#737082] w-full px-3 py-1.5 rounded-md" onclick={handle_activate_contract}>Activate contract (pay collateral)</button>
		</div>
	</div>
</div>
<div class="row-span-6 flex">
	<Eurotaxisimulator bind:driver_index={driver_index} departure_index={departure_index} arrival_index={arrival_index} points_indexes={points_indexes} />
</div>
<div class="row-span-3 col-span-2 flex flex-col justify-center border-t border-r border-inherit px-14">
	<p class="text-white text-6xl font-bold">POLLED</p>
	<div class="flex w-full justify-between">
		<p class="text-white text-6xl font-bold">STATUS</p>
		<p class="text-white text-6xl font-bold border-b border-white">{status_message}</p>
	</div>
</div>
<div class="row-span-3 flex border-t border-inherit">
	<button onclick={handle_unlock} class="h-full w-1/2 border-r border-inherit text-white text-3xl font-bold">UNLOCK</button>
	<button onclick={handle_refund} class="h-full w-1/2 text-white text-3xl font-bold">REFUND</button>
</div>
