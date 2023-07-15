import './assets/styles/index.scss';

import App from './App.vue';
import { createApp } from 'vue';

const mirrorOS = createApp(App);

mirrorOS.mount('#app');
