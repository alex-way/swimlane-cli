import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://alex-way.github.io',
	base: '/swimlane-cli/',
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
					autogenerate: { directory: 'commands' },
					items: [
						{
							label: "task",
							items: [
								{
									label: "task save",
									link: "/commands/task/save/"
								}
							]
						},
						{
							label: "migrate",
							items: [
								{
									label: "migrate",
									link: "/commands/migrate/"
								},
								{
									label: "migrate user",
									link: "/commands/migrate/user/"
								},
								{
									label: "migrate users",
									link: "/commands/migrate/users/"
								},
								{
									label: "migrate group",
									link: "/commands/migrate/group/"
								},
								{
									label: "migrate groups",
									link: "/commands/migrate/groups/"
								},
								{
									label: "migrate role",
									link: "/commands/migrate/role/"
								},
								{
									label: "migrate roles",
									link: "/commands/migrate/roles/"
								},
								{
									label: "migrate app",
									link: "/commands/migrate/app/"
								}
								,
								{
									label: "migrate apps",
									link: "/commands/migrate/apps/"
								}
							]
						},
						{
							label: "pip",
							items: [
								{
									label: "pip install",
									link: "/commands/pip/install/"
								},
								{
									label: "pip remove",
									link: "/commands/pip/remove/"
								},
								{
									label: "pip freeze",
									link: "/commands/pip/freeze/"
								}
							]
						}
					]
				},
			],
		}),
	],

	// Process images with sharp: https://docs.astro.build/en/guides/assets/#using-sharp
	image: { service: { entrypoint: 'astro/assets/services/sharp' } },

});
