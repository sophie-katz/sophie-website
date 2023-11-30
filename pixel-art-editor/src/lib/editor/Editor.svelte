<!--
Copyright (c) 2023 Sophie Katz

This file is part of Sophie's Website.

Sophie's Website is free software: you can redistribute it and/or modify it under the terms of the
GNU General Public License as published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

Sophie's Website is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.
 
You should have received a copy of the GNU General Public License along with Sophie's Website. If
not, see <https://www.gnu.org/licenses/>.
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import * as PIXI from 'pixi.js';
	import { Layer } from './layer';
	import { Viewport } from './viewport';

	let wrapper: HTMLElement;
	let application: PIXI.Application;
	let viewport: Viewport;
	let layers: Layer[];
	let currentLayer: number = 0;

	onMount(() => {
		application = new PIXI.Application({
			width: 640,
			height: 480
		});

		viewport = new Viewport();

		wrapper.appendChild(application.view as unknown as HTMLElement);

		layers = [new Layer(application.renderer, viewport, { width: 32, height: 32 })];

		layers[0].mount(application.stage);
	});

	function handleWheel(event: WheelEvent) {
		if (event.metaKey || event.ctrlKey) {
			viewport.zoom -= event.deltaY / 50;
		} else {
			viewport.offset.x -= event.deltaX;
			viewport.offset.y -= event.deltaY;
		}

		layers[0].navigate();

		event.preventDefault();
	}
</script>

<div>
	<div bind:this={wrapper} on:wheel={handleWheel}></div>

	Scroll to pan<br />
	Ctrl/Cmd + Scroll to zoom
</div>
