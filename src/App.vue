<template>
	<div class="w-full h-full flex justify-center items-center">
		<h1 className="text-3xl font-bold underline">Hello world!</h1>
	</div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import MirrorOSConfig from './config/config';
import { GetMailsChannel } from './modules/email/channels';

onMounted(async () => {
	const result = await new GetMailsChannel().use(MirrorOSConfig.sharedInstance.email.iCloud);
	const result2 = await new GetMailsChannel().use(MirrorOSConfig.sharedInstance.email.outlook);

	console.log(result);
	console.log(result2);
});
</script>
