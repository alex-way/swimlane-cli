import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://alex-way.github.io',
  	base: '/swimlane-cli',
	integrations: [
		starlight({
			title: 'swimlane CLI',
			logo: {
				src: "./src/assets/swimlane.svg",
			},
			social: {
				github: 'https://github.com/alex-way/swimlane-cli',
			},
			sidebar: [
				{
					label: "CLI Commands",
					autogenerate: { directory: 'commands'},
					items: [
						{
							label: "download-python-tasks",
							link: "/commands/download-python-tasks/"
						},
						{
							label: "migrate",
							link: "/commands/migrate/"
						},
						{
							label: "pip",
							link: "/commands/pip/"
						}
					]
				},
			],
		}),
	],

	// Process images with sharp: https://docs.astro.build/en/guides/assets/#using-sharp
	image: { service: { entrypoint: 'astro/assets/services/sharp' } },
});
