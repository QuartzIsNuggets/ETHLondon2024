<script>
	import Eurotaxisimulator from '$lib/components/eurotaxisimulator.svelte'
	import arbitrum_logo_src from '$lib/assets/logo_monochrome.svg'
    import stylus_logo_src from '$lib/assets/stylus_white.svg'
    import github_logo_src from '$lib/assets/github-mark-white.svg'
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
		<h1 class="font-grotesk text-[#737082] text-md">Saint-Luc</h1>
		<img class="h-[40%]" src={github_logo_src} alt="GITHUB" />
		<h1 class="font-grotesk text-[#737082] text-md">QuartzIsNuggets</h1>
	</div>
</div>
<div class="flex border-b border-inherit">
	<div class="flex justify-center items-center gap-4 h-full w-1/2 border-r border-inherit px-14 py-4">
		<h1 class="font-grotesk text-[#737082] text-md">Luc Sanglas</h1>
	</div>
	<div class="flex justify-between items-center h-full w-1/2 px-14">
		<h1 class="font-grotesk text-[#737082] text-md">Alexis Ronez</h1>
	</div>
</div>
<div class="row-span-6 col-span-2 flex border-r border-inherit text-[#737082]">
	<div class="h-full w-1/2">
		<h1 class="text-4xl px-14 py-8 font-sl font-black">CLIENT</h1>
	</div>
	<div class="h-full w-1/2">
		<h1 class="text-4xl px-14 py-8 font-sl font-black">DRIVER</h1>
	</div>
</div>
<div class="row-span-6 flex justify-center">
	<Eurotaxisimulator />
</div>
<div class="row-span-3 col-span-2 border-t border-r border-inherit flex items-center justify-between">
	<h1 class="text-white text-6xl px-12 py-8 font-black">CURRENT<br>RISK FACTOR</h1>
	<h1 class="text-white text-6xl px-12 py-8 font-bold">2%</h1>
</div>
<div class="row-span-3 border-t border-inherit">
	<h1 class="w-100% text-white text-4xl font-sl font-black text-justify">OTHER POLLABLE INFORMATION</h1>
</div>

