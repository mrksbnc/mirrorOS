import { defineAsyncComponent } from 'vue';

export declare type MirrorUiModule = {
	name: string;
	version: string;
	filleModuleGrid?: boolean;
	position: 'left' | 'right' | 'top' | 'bottom';
	subPosition?: 'inner-left' | 'inner-right' | 'inner-top' | 'inner-bottom';
	component: ReturnType<typeof defineAsyncComponent>;
};
