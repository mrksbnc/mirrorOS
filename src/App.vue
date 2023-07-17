<template>
	<div class="w-full h-full flex justify-center items-center">
		<h1 className="text-3xl font-bold underline">Hello world!</h1>
		<span
			class="flex flex-row justify-center items-center w-1/2 h-1/2 bg-gray-200 rounded-lg shadow-lg"
			v-for="(item, index) in result"
			:key="index"
		>
			{{ item }}
		</span>
	</div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { GetUnseenMailsChannel } from './modules/email/channels';

const result = ref();
const result2 = ref();

onMounted(async () => {
	result.value = ref(await new GetUnseenMailsChannel().use());
	result2.value = ref(await new GetUnseenMailsChannel().use());

	console.log(result);
	console.log(result2);
});
</script>
