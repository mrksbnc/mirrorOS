<template>
	<div class="w-full h-full">
		<component :is="module.component" />
	</div>
</template>

<script setup lang="ts">
import { PropType } from 'vue';
import type { MirrorUiModule } from '@/types/mirror_ui_module.d';

const props = defineProps({
	module: {
		type: Object as PropType<MirrorUiModule>,
		required: true,
	},
});
</script>
