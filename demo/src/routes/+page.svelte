<script>
    import {create_new_contract} from "./transaction.js";
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

    let filler_id = $state('');
    const checkpoints = [[8, 8], [22, 14], [28, 36], [38, 43], [55, 55]];
    let points_indexes = $state([3, 4, 5, 6, 7]);

    async function handle_send_transaction() {
        await create_new_contract(1, filler_id, 42, checkpoints);
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

    async function handle_pay_collateral() {

	}

    async function handle_unlock() {

	}
    async function handle_refund() {

    }

    async function send_position() {
        driver_index;
	}

    const interval_id = setInterval(send_position, 1000);

    $effect(() => {
       if (polled_status > 3)
           clearInterval(interval_id);
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
			<button class="border border-[#737082] w-full px-3 py-1.5 rounded-md" onclick={handle_send_transaction}>Issue contract (pre-baked)</button>
		</div>
	</div>
	<div class="h-full w-1/2 flex flex-col gap-6 px-14">
		<p class="text-4xl font-black">DRIVER</p>
		<div class="flex flex-col gap-3">
			<button class="border border-[#737082] w-full px-3 py-1.5 rounded-md" onclick={handle_pay_collateral}>Pay collateral</button>
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
		<p class="text-white text-6xl font-bold">{status_message}</p>
	</div>
</div>
<div class="row-span-3 flex border-t border-inherit">
	<button onclick={handle_unlock} class="h-full w-1/2 border-r border-inherit text-white text-3xl font-bold">UNLOCK</button>
	<button onclick={handle_refund} class="h-full w-1/2 text-white text-3xl font-bold">REFUND</button>
</div>
