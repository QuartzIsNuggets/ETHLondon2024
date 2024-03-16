<script>
    let grid_size = 64;
    let grid = Array(grid_size * grid_size).fill(null);
    let active_cell_index = 0;

    // Function to move the active cell
    function moveActiveCell(e) {
        const { key } = e;
        if (key === 'ArrowUp' && active_cell_index >= grid_size) {
            active_cell_index -= grid_size;
        } else if (key === 'ArrowDown' && active_cell_index < grid_size * (grid_size - 1)) {
            active_cell_index += grid_size;
        } else if (key === 'ArrowLeft' && active_cell_index % grid_size !== 0) {
            active_cell_index -= 1;
        } else if (key === 'ArrowRight' && active_cell_index % grid_size !== (grid_size - 1)) {
            active_cell_index += 1;
        }
    }
</script>

<div class="demo_grid aspect-square h-full outline-none" role="toolbar" tabindex="0" onkeydown={moveActiveCell}>
	{#each grid as _, index}
		<div class="{index === active_cell_index ? 'bg-red-500' : ''}"></div>
	{/each}
</div>

<style>
    .demo_grid {
        display: grid;
        grid-template-columns: repeat(64, 1fr);
        grid-template-rows: repeat(64, 1fr);
    }
</style>
