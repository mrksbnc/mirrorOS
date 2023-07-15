<template>
	<div class="w-full h-full flex justify-center items-center">
		<h1 className="text-3xl font-bold underline">Hello world!</h1>
	</div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import MirrorOSConfigImpl from './config/config';
import { GetMailsChannel } from './modules/email/channels';

onMounted(async () => {
	const result = await new GetMailsChannel().use({
		port: MirrorOSConfigImpl.sharedInstance.email.outlook.port,
		host: MirrorOSConfigImpl.sharedInstance.email.outlook.host,
		auth: MirrorOSConfigImpl.sharedInstance.email.outlook.auth,
	});

	console.log(result);
});
</script>
