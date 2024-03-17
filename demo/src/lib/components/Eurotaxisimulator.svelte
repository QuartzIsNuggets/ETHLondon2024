<script>
    let { driver_index = 0, departure_index = 1, arrival_index = 2, points_indexes = [3, 4, 5, 6, 7] } = $props();
    let grid_size = 64;
    let grid = Array(grid_size * grid_size).fill(null);

    // Function to move the active cell
    function moveActiveCell(e) {
        const { key } = e;
        if (key === 'ArrowUp' && driver_index >= grid_size) {
            driver_index -= grid_size;
        } else if (key === 'ArrowDown' && driver_index < grid_size * (grid_size - 1)) {
            driver_index += grid_size;
        } else if (key === 'ArrowLeft' && driver_index % grid_size !== 0) {
            driver_index -= 1;
        } else if (key === 'ArrowRight' && driver_index % grid_size !== (grid_size - 1)) {
            driver_index += 1;
        }
    }

    function color(index) {
        if (index === driver_index)
            return 'bg-red-500';
        else if (index === departure_index)
            return 'bg-blue-500';
        else if (index === arrival_index)
            return 'bg-green-500';
        else if (points_indexes.includes(index))
            return 'bg-yellow-500'
        else
            return '';
	}
</script>

<div class="demo_grid aspect-square h-full outline-none" role="toolbar" tabindex="0" onkeydown={moveActiveCell}>
	{#each grid as _, index}
		<div class="{color(index)}"></div>
	{/each}
</div>

<style>
    .demo_grid {
        display: grid;
        grid-template-columns: repeat(64, 1fr);
        grid-template-rows: repeat(64, 1fr);
    }
</style>
