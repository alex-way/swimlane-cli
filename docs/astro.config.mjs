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
							label: "task",
							link: "/commands/task/"
						},
						{
							label: "migrate",
							items: [
								{
									label: "all",
									link: "/commands/migrate/"
								},
								{
									label: "user",
									link: "/commands/migrate/user/"
								},
								{
									label: "users",
									link: "/commands/migrate/users/"
								},
								{
									label: "group",
									link: "/commands/migrate/group/"
								},
								{
									label: "groups",
									link: "/commands/migrate/groups/"
								},
								{
									label: "role",
									link: "/commands/migrate/role/"
								},
								{
									label: "roles",
									link: "/commands/migrate/roles/"
								},
								{
									label: "app",
									link: "/commands/migrate/app/"
								}
								,
								{
									label: "apps",
									link: "/commands/migrate/apps/"
								}
							]
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
