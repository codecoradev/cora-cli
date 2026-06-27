
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/private';
 * 
 * console.log(ENVIRONMENT); // => "production"
 * console.log(PUBLIC_BASE_URL); // => throws error during build
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/private' {
	export const HERMES_GID: string;
	export const HERMES_MODEL: string;
	export const OPENAI_FALLBACK_API_KEY: string;
	export const timezone: string;
	export const PYTHONIOENCODING: string;
	export const HERMES_RESTART_DRAIN_TIMEOUT: string;
	export const _HERMES_GATEWAY: string;
	export const HERMES_SESSION_KEY: string;
	export const TG_AJIANAZBRIEF_CHAT_ID: string;
	export const PLANE_AZFIRAZKA_WORKSPACE: string;
	export const TG_BOT_TOKEN: string;
	export const npm_config_user_agent: string;
	export const TERMINAL_CONTAINER_CPU: string;
	export const CARGO_TARGET_DIR: string;
	export const AGENT_BROWSER_EXECUTABLE_PATH: string;
	export const MATRIX_ALLOWED_ROOMS: string;
	export const DISCORD_HISTORY_BACKFILL: string;
	export const HERMES_SESSION_USER_NAME: string;
	export const TERMINAL_DOCKER_ENV: string;
	export const TERMINAL_CWD: string;
	export const DISCORD_REACTIONS: string;
	export const STALWART_APIKEY: string;
	export const npm_node_execpath: string;
	export const GHOST_WEBHOOK_SECRET: string;
	export const SHLVL: string;
	export const npm_config_noproxy: string;
	export const HOME: string;
	export const GOWA_CHANNEL_ADK_INFO: string;
	export const GLM_PROXY_ADMIN_API_KEY: string;
	export const OLDPWD: string;
	export const TERMINAL_DOCKER_FORWARD_ENV: string;
	export const X_CODECORA_USER: string;
	export const THREADS_ANALYZE_PARENT_ID: string;
	export const PEXELS_API_KEY: string;
	export const HTTP_API_KEY: string;
	export const HERMES_HOME: string;
	export const npm_package_json: string;
	export const MINIO_BUCKET: string;
	export const TG_AJIANAZBRIEF_USERNAME: string;
	export const GHOST_ADMIN_KEY: string;
	export const TERMINAL_DOCKER_IMAGE: string;
	export const HERMES_EXEC_ASK: string;
	export const ASSEMBLYAI_API_KEY: string;
	export const SLACK_FREE_RESPONSE_CHANNELS: string;
	export const JINA_API_KEY: string;
	export const TERMINAL_DOCKER_MOUNT_CWD_TO_WORKSPACE: string;
	export const TERMINAL_CONTAINER_MEMORY: string;
	export const LC_CTYPE: string;
	export const SSL_CERT_FILE: string;
	export const TERMINAL_CONTAINER_PERSISTENT: string;
	export const npm_config_userconfig: string;
	export const npm_config_local_prefix: string;
	export const HERMES_SESSION_USER_ID: string;
	export const OPENSSL_INCLUDE_DIR: string;
	export const HERMES_SESSION_CHAT_NAME: string;
	export const TERMINAL_MODAL_IMAGE: string;
	export const CORA_PROVIDER: string;
	export const ZAI_CUSTOMER_ID: string;
	export const file_read_max_chars: string;
	export const MINIO_ACCESS_KEY: string;
	export const COLOR: string;
	export const DISCORD_HISTORY_BACKFILL_LIMIT: string;
	export const CLOUDFLARE_ACCOUNT_ID: string;
	export const TERMINAL_DOCKER_VOLUMES: string;
	export const npm_config_metrics_registry: string;
	export const THREADS_KARIRDEV_USER_ID: string;
	export const MINIO_REGION: string;
	export const TERMINAL_ENV: string;
	export const HERMES_UID: string;
	export const AUXILIARY_VISION_PROVIDER: string;
	export const CORA_BASE_URL: string;
	export const TERMINAL_VERCEL_RUNTIME: string;
	export const MODAL_API_KEY: string;
	export const MINIO_ENDPOINT: string;
	export const SESSION_IDLE_MINUTES: string;
	export const TELEGRAM_ALLOWED_CHATS: string;
	export const PYTHONDONTWRITEBYTECODE: string;
	export const HERMES_SESSION_THREAD_ID: string;
	export const GLM_PROXY_BASE_URL: string;
	export const LYNK_MERCHANT_KEY: string;
	export const LLM_BASE_URL: string;
	export const ADK_COMPANY_LOGO: string;
	export const AUXILIARY_VISION_BASE_URL: string;
	export const QDRANT_API_KEY: string;
	export const _: string;
	export const npm_config_prefix: string;
	export const PKG_CONFIG_PATH: string;
	export const group_sessions_per_user: string;
	export const LITELLM_MASTER_KEY: string;
	export const CORA_API_KEY: string;
	export const HERMES_PROVIDER: string;
	export const GHOST_ADMIN_URL: string;
	export const TERMINAL_DAYTONA_IMAGE: string;
	export const GLM_PROXY_API_KEY: string;
	export const npm_config_cache: string;
	export const AUXILIARY_VISION_API_KEY: string;
	export const HERMES_SESSION_CHAT_ID: string;
	export const DISCORD_THREAD_REQUIRE_MENTION: string;
	export const ZAI_JWT: string;
	export const RUSTUP_HOME: string;
	export const THREADS_CODECORA_PASS: string;
	export const TERMINAL_LIFETIME_SECONDS: string;
	export const DISCORD_ALLOWED_CHANNELS: string;
	export const TERMINAL_TIMEOUT: string;
	export const HERMES_GATEWAY_BUSY_INPUT_MODE: string;
	export const npm_config_node_gyp: string;
	export const PATH: string;
	export const MEMPALACE_PALACE_PATH: string;
	export const TELEGRAM_REACTIONS: string;
	export const MAYAR_WEBHOOK_TOKEN: string;
	export const HERMES_AGENT_TIMEOUT: string;
	export const NODE: string;
	export const npm_package_name: string;
	export const THREADS_KARIRDEV_ACCESS_TOKEN: string;
	export const OPENSSL_LIB_DIR: string;
	export const DISCORD_ACTIVITY: string;
	export const HERMES_WEB_DIST: string;
	export const HERMES_RPC_SOCKET: string;
	export const TERMINAL_PERSISTENT_SHELL: string;
	export const THREADS_ANALYZE_COLLECTION_ID: string;
	export const PLANE_BASE_URL: string;
	export const HERMES_QUIET: string;
	export const PLANE_AKALA_WORKSPACE: string;
	export const VIRTUAL_ENV_PROMPT: string;
	export const MQTT_URL: string;
	export const THREADS_GUYONLELUCON_PASS: string;
	export const QDRANT_URL: string;
	export const HERMES_MAX_ITERATIONS: string;
	export const npm_lifecycle_script: string;
	export const HERMES_SESSION_ID: string;
	export const GOWA_BASIC_AUTH: string;
	export const HERMES_AGENT_TIMEOUT_WARNING: string;
	export const prefill_messages_file: string;
	export const HERMES_AGENT_NOTIFY_INTERVAL: string;
	export const OPENAI_MODEL: string;
	export const PLANE_API_KEY: string;
	export const THREADS_CODECORA_USER: string;
	export const CLOUDFLARE_TOKEN: string;
	export const npm_package_version: string;
	export const npm_lifecycle_event: string;
	export const SLACK_ALLOWED_CHANNELS: string;
	export const OUTLINE_BASE_URL: string;
	export const MAYAR_API_KEY: string;
	export const HERMES_DAEMON: string;
	export const AGENTBOARD_API_KEY: string;
	export const PIXABAY_API_KEY: string;
	export const PYTHONUTF8: string;
	export const THREADS_CODECORA_USER_ID: string;
	export const GOWA_CHANNEL_AZFEED: string;
	export const PLANE_AKALA_PROJECT_ID: string;
	export const NINE_ROUTER_BASE_URL: string;
	export const HERMES_SESSION_PLATFORM: string;
	export const TERMINAL_CONTAINER_DISK: string;
	export const OUTLINE_API_KEY: string;
	export const MINIO_SECRET_KEY: string;
	export const THREADS_GUYONLELUCON_USER: string;
	export const VIRTUAL_ENV: string;
	export const TERMINAL_DOCKER_RUN_AS_HOST_USER: string;
	export const npm_config_globalconfig: string;
	export const npm_config_init_module: string;
	export const NINE_ROUTER_API_KEY: string;
	export const PWD: string;
	export const _config_version: string;
	export const HERMES_AUTO_CONTINUE_FRESHNESS: string;
	export const npm_config_globalignorefile: string;
	export const npm_execpath: string;
	export const TG_CRYPTOTECHTALKID_CHAT_ID: string;
	export const CARGO_HOME: string;
	export const npm_config_global_prefix: string;
	export const HERMES_CRON_SESSION: string;
	export const GOWA_WEBHOOK_SECRET: string;
	export const PYTHONPATH: string;
	export const hooks_auto_accept: string;
	export const npm_command: string;
	export const BROWSER_INACTIVITY_TIMEOUT: string;
	export const ALLOW_SCRIPT_EDIT: string;
	export const HERMES_REDACT_SECRETS: string;
	export const OPENAI_FALLBACK_BASE_URL: string;
	export const THREADS_CODECORA_ACCESS_TOKEN: string;
	export const AUXILIARY_VISION_MODEL: string;
	export const MEMPALACE_CONFIG_DIR: string;
	export const TERMINAL_SINGULARITY_IMAGE: string;
	export const X_CODECORA_PASS: string;
	export const MATTERMOST_ALLOWED_CHANNELS: string;
	export const SLACK_REQUIRE_MENTION: string;
	export const AGENTBOARD_URL: string;
	export const INIT_CWD: string;
	export const EDITOR: string;
	export const NODE_ENV: string;
}

/**
 * This module provides access to environment variables that are injected _statically_ into your bundle at build time and are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Static environment variables are [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env` at build time and then statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * For example, given the following build time environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { ENVIRONMENT, PUBLIC_BASE_URL } from '$env/static/public';
 * 
 * console.log(ENVIRONMENT); // => throws error during build
 * console.log(PUBLIC_BASE_URL); // => "http://site.com"
 * ```
 * 
 * The above values will be the same _even if_ different values for `ENVIRONMENT` or `PUBLIC_BASE_URL` are set at runtime, as they are statically replaced in your code with their build time values.
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are limited to _private_ access.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Private_ access:**
 * 
 * - This module cannot be imported into client-side code
 * - This module includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured)
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://site.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * 
 * console.log(env.ENVIRONMENT); // => "production"
 * console.log(env.PUBLIC_BASE_URL); // => undefined
 * ```
 */
declare module '$env/dynamic/private' {
	export const env: {
		HERMES_GID: string;
		HERMES_MODEL: string;
		OPENAI_FALLBACK_API_KEY: string;
		timezone: string;
		PYTHONIOENCODING: string;
		HERMES_RESTART_DRAIN_TIMEOUT: string;
		_HERMES_GATEWAY: string;
		HERMES_SESSION_KEY: string;
		TG_AJIANAZBRIEF_CHAT_ID: string;
		PLANE_AZFIRAZKA_WORKSPACE: string;
		TG_BOT_TOKEN: string;
		npm_config_user_agent: string;
		TERMINAL_CONTAINER_CPU: string;
		CARGO_TARGET_DIR: string;
		AGENT_BROWSER_EXECUTABLE_PATH: string;
		MATRIX_ALLOWED_ROOMS: string;
		DISCORD_HISTORY_BACKFILL: string;
		HERMES_SESSION_USER_NAME: string;
		TERMINAL_DOCKER_ENV: string;
		TERMINAL_CWD: string;
		DISCORD_REACTIONS: string;
		STALWART_APIKEY: string;
		npm_node_execpath: string;
		GHOST_WEBHOOK_SECRET: string;
		SHLVL: string;
		npm_config_noproxy: string;
		HOME: string;
		GOWA_CHANNEL_ADK_INFO: string;
		GLM_PROXY_ADMIN_API_KEY: string;
		OLDPWD: string;
		TERMINAL_DOCKER_FORWARD_ENV: string;
		X_CODECORA_USER: string;
		THREADS_ANALYZE_PARENT_ID: string;
		PEXELS_API_KEY: string;
		HTTP_API_KEY: string;
		HERMES_HOME: string;
		npm_package_json: string;
		MINIO_BUCKET: string;
		TG_AJIANAZBRIEF_USERNAME: string;
		GHOST_ADMIN_KEY: string;
		TERMINAL_DOCKER_IMAGE: string;
		HERMES_EXEC_ASK: string;
		ASSEMBLYAI_API_KEY: string;
		SLACK_FREE_RESPONSE_CHANNELS: string;
		JINA_API_KEY: string;
		TERMINAL_DOCKER_MOUNT_CWD_TO_WORKSPACE: string;
		TERMINAL_CONTAINER_MEMORY: string;
		LC_CTYPE: string;
		SSL_CERT_FILE: string;
		TERMINAL_CONTAINER_PERSISTENT: string;
		npm_config_userconfig: string;
		npm_config_local_prefix: string;
		HERMES_SESSION_USER_ID: string;
		OPENSSL_INCLUDE_DIR: string;
		HERMES_SESSION_CHAT_NAME: string;
		TERMINAL_MODAL_IMAGE: string;
		CORA_PROVIDER: string;
		ZAI_CUSTOMER_ID: string;
		file_read_max_chars: string;
		MINIO_ACCESS_KEY: string;
		COLOR: string;
		DISCORD_HISTORY_BACKFILL_LIMIT: string;
		CLOUDFLARE_ACCOUNT_ID: string;
		TERMINAL_DOCKER_VOLUMES: string;
		npm_config_metrics_registry: string;
		THREADS_KARIRDEV_USER_ID: string;
		MINIO_REGION: string;
		TERMINAL_ENV: string;
		HERMES_UID: string;
		AUXILIARY_VISION_PROVIDER: string;
		CORA_BASE_URL: string;
		TERMINAL_VERCEL_RUNTIME: string;
		MODAL_API_KEY: string;
		MINIO_ENDPOINT: string;
		SESSION_IDLE_MINUTES: string;
		TELEGRAM_ALLOWED_CHATS: string;
		PYTHONDONTWRITEBYTECODE: string;
		HERMES_SESSION_THREAD_ID: string;
		GLM_PROXY_BASE_URL: string;
		LYNK_MERCHANT_KEY: string;
		LLM_BASE_URL: string;
		ADK_COMPANY_LOGO: string;
		AUXILIARY_VISION_BASE_URL: string;
		QDRANT_API_KEY: string;
		_: string;
		npm_config_prefix: string;
		PKG_CONFIG_PATH: string;
		group_sessions_per_user: string;
		LITELLM_MASTER_KEY: string;
		CORA_API_KEY: string;
		HERMES_PROVIDER: string;
		GHOST_ADMIN_URL: string;
		TERMINAL_DAYTONA_IMAGE: string;
		GLM_PROXY_API_KEY: string;
		npm_config_cache: string;
		AUXILIARY_VISION_API_KEY: string;
		HERMES_SESSION_CHAT_ID: string;
		DISCORD_THREAD_REQUIRE_MENTION: string;
		ZAI_JWT: string;
		RUSTUP_HOME: string;
		THREADS_CODECORA_PASS: string;
		TERMINAL_LIFETIME_SECONDS: string;
		DISCORD_ALLOWED_CHANNELS: string;
		TERMINAL_TIMEOUT: string;
		HERMES_GATEWAY_BUSY_INPUT_MODE: string;
		npm_config_node_gyp: string;
		PATH: string;
		MEMPALACE_PALACE_PATH: string;
		TELEGRAM_REACTIONS: string;
		MAYAR_WEBHOOK_TOKEN: string;
		HERMES_AGENT_TIMEOUT: string;
		NODE: string;
		npm_package_name: string;
		THREADS_KARIRDEV_ACCESS_TOKEN: string;
		OPENSSL_LIB_DIR: string;
		DISCORD_ACTIVITY: string;
		HERMES_WEB_DIST: string;
		HERMES_RPC_SOCKET: string;
		TERMINAL_PERSISTENT_SHELL: string;
		THREADS_ANALYZE_COLLECTION_ID: string;
		PLANE_BASE_URL: string;
		HERMES_QUIET: string;
		PLANE_AKALA_WORKSPACE: string;
		VIRTUAL_ENV_PROMPT: string;
		MQTT_URL: string;
		THREADS_GUYONLELUCON_PASS: string;
		QDRANT_URL: string;
		HERMES_MAX_ITERATIONS: string;
		npm_lifecycle_script: string;
		HERMES_SESSION_ID: string;
		GOWA_BASIC_AUTH: string;
		HERMES_AGENT_TIMEOUT_WARNING: string;
		prefill_messages_file: string;
		HERMES_AGENT_NOTIFY_INTERVAL: string;
		OPENAI_MODEL: string;
		PLANE_API_KEY: string;
		THREADS_CODECORA_USER: string;
		CLOUDFLARE_TOKEN: string;
		npm_package_version: string;
		npm_lifecycle_event: string;
		SLACK_ALLOWED_CHANNELS: string;
		OUTLINE_BASE_URL: string;
		MAYAR_API_KEY: string;
		HERMES_DAEMON: string;
		AGENTBOARD_API_KEY: string;
		PIXABAY_API_KEY: string;
		PYTHONUTF8: string;
		THREADS_CODECORA_USER_ID: string;
		GOWA_CHANNEL_AZFEED: string;
		PLANE_AKALA_PROJECT_ID: string;
		NINE_ROUTER_BASE_URL: string;
		HERMES_SESSION_PLATFORM: string;
		TERMINAL_CONTAINER_DISK: string;
		OUTLINE_API_KEY: string;
		MINIO_SECRET_KEY: string;
		THREADS_GUYONLELUCON_USER: string;
		VIRTUAL_ENV: string;
		TERMINAL_DOCKER_RUN_AS_HOST_USER: string;
		npm_config_globalconfig: string;
		npm_config_init_module: string;
		NINE_ROUTER_API_KEY: string;
		PWD: string;
		_config_version: string;
		HERMES_AUTO_CONTINUE_FRESHNESS: string;
		npm_config_globalignorefile: string;
		npm_execpath: string;
		TG_CRYPTOTECHTALKID_CHAT_ID: string;
		CARGO_HOME: string;
		npm_config_global_prefix: string;
		HERMES_CRON_SESSION: string;
		GOWA_WEBHOOK_SECRET: string;
		PYTHONPATH: string;
		hooks_auto_accept: string;
		npm_command: string;
		BROWSER_INACTIVITY_TIMEOUT: string;
		ALLOW_SCRIPT_EDIT: string;
		HERMES_REDACT_SECRETS: string;
		OPENAI_FALLBACK_BASE_URL: string;
		THREADS_CODECORA_ACCESS_TOKEN: string;
		AUXILIARY_VISION_MODEL: string;
		MEMPALACE_CONFIG_DIR: string;
		TERMINAL_SINGULARITY_IMAGE: string;
		X_CODECORA_PASS: string;
		MATTERMOST_ALLOWED_CHANNELS: string;
		SLACK_REQUIRE_MENTION: string;
		AGENTBOARD_URL: string;
		INIT_CWD: string;
		EDITOR: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * This module provides access to environment variables set _dynamically_ at runtime and that are _publicly_ accessible.
 * 
 * |         | Runtime                                                                    | Build time                                                               |
 * | ------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
 * | Private | [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private) | [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private) |
 * | Public  | [`$env/dynamic/public`](https://svelte.dev/docs/kit/$env-dynamic-public)   | [`$env/static/public`](https://svelte.dev/docs/kit/$env-static-public)   |
 * 
 * Dynamic environment variables are defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`.
 * 
 * **_Public_ access:**
 * 
 * - This module _can_ be imported into client-side code
 * - **Only** variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`) are included
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 * 
 * > [!NOTE] To get correct types, environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * >
 * > ```env
 * > MY_FEATURE_FLAG=
 * > ```
 * >
 * > You can override `.env` values from the command line like so:
 * >
 * > ```sh
 * > MY_FEATURE_FLAG="enabled" npm run dev
 * > ```
 * 
 * For example, given the following runtime environment:
 * 
 * ```env
 * ENVIRONMENT=production
 * PUBLIC_BASE_URL=http://example.com
 * ```
 * 
 * With the default `publicPrefix` and `privatePrefix`:
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.ENVIRONMENT); // => undefined, not public
 * console.log(env.PUBLIC_BASE_URL); // => "http://example.com"
 * ```
 * 
 * ```
 * 
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
