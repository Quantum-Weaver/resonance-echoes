<script lang="ts">
	import { echoStore } from '$lib/stores/echo.svelte';
	import { SENSES } from '$lib/data/senses';

	let displayCount = $state(50);

	const echoes = $derived(echoStore.echoes);
	const visible = $derived(echoes.slice(0, displayCount));
	const hasMore = $derived(echoes.length > displayCount);
	const total = $derived(echoStore.totalCount);

	function getSense(senseId: string) {
		return SENSES.find((s) => s.id === senseId) ?? { name: senseId, emoji: '✨' };
	}

	function relativeTime(timestamp: number): string {
		const diff = Date.now() - timestamp;
		if (diff < 60_000) return 'just now';
		if (diff < 3_600_000) return `${Math.floor(diff / 60_000)}m ago`;
		if (diff < 86_400_000) return `${Math.floor(diff / 3_600_000)}h ago`;
		const days = Math.floor(diff / 86_400_000);
		if (days === 1) return 'yesterday';
		if (days < 7) return new Date(timestamp).toLocaleDateString('en', { weekday: 'long' });
		return new Date(timestamp).toLocaleDateString('en', { month: 'short', day: 'numeric' });
	}
</script>

<div class="home" style="padding-top: env(safe-area-inset-top, 0px);">
	<header class="home-header">
		<h1 class="home-title">Echoes</h1>
		{#if total > 0}
			<span class="count-badge">{total}</span>
		{/if}
	</header>

	{#if echoes.length === 0}
		<div class="empty-state">
			<div class="empty-icon">✨</div>
			<p class="empty-heading">No echoes yet.</p>
			<p class="empty-sub">Tap + to log your first felt moment.</p>
		</div>
	{:else}
		<div class="echo-list">
			{#each visible as echo (echo.id)}
				{@const sense = getSense(echo.sense)}
				<div class="echo-card">
					<div class="echo-emoji">{echo.emoji}</div>
					<div class="echo-body">
						<div class="echo-header">
							<span class="echo-name">{echo.name}</span>
							<span class="echo-time">{relativeTime(echo.timestamp)}</span>
						</div>
						<div class="echo-meta">
							<span class="sense-badge">{sense.emoji} {sense.name} · {echo.subcategory}</span>
						</div>
						<div class="echo-intensity">
							{#each [1, 2, 3, 4, 5] as n}
								<div class="dot" class:filled={n <= echo.intensity}></div>
							{/each}
						</div>
						{#if echo.note}
							<p class="echo-note">{echo.note}</p>
						{/if}
					</div>
				</div>
			{/each}

			{#if hasMore}
				<button class="load-more" onclick={() => (displayCount += 50)}>
					Load more
				</button>
			{/if}
		</div>
	{/if}
</div>

<style>
	.home {
		min-height: 100%;
	}

	.home-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		padding: 1rem 1.25rem 0.75rem;
		border-bottom: 1px solid var(--border-color);
	}

	.home-title {
		font-size: 1.25rem;
		font-weight: 700;
		color: var(--text);
		margin: 0;
	}

	.count-badge {
		background: color-mix(in srgb, var(--accent) 20%, transparent);
		color: var(--accent);
		border-radius: 20px;
		padding: 0.1rem 0.6rem;
		font-size: 0.75rem;
		font-weight: 600;
	}

	/* Empty state */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 4rem 2rem;
		gap: 0.5rem;
		text-align: center;
	}
	.empty-icon { font-size: 3rem; margin-bottom: 0.5rem; }
	.empty-heading { font-size: 1.1rem; font-weight: 600; color: var(--text); margin: 0; }
	.empty-sub { font-size: 0.9rem; color: var(--text-muted); margin: 0; }

	/* Echo list */
	.echo-list {
		padding: 0.75rem 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	/* Echo card */
	.echo-card {
		display: flex;
		gap: 0.875rem;
		padding: 0.875rem 1rem;
		background: var(--bg-surface);
		border: 1px solid var(--border-color);
		border-radius: 12px;
		transition: border-color 0.15s;
	}
	.echo-card:hover { border-color: color-mix(in srgb, var(--accent) 40%, var(--border-color)); }

	.echo-emoji {
		font-size: 2rem;
		line-height: 1;
		flex-shrink: 0;
		width: 2.25rem;
		text-align: center;
	}

	.echo-body {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.echo-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		gap: 0.5rem;
	}

	.echo-name {
		font-size: 0.95rem;
		font-weight: 600;
		color: var(--text);
		flex: 1;
		min-width: 0;
		word-break: break-word;
	}

	.echo-time {
		font-size: 0.7rem;
		color: var(--text-muted);
		white-space: nowrap;
		flex-shrink: 0;
		padding-top: 0.1rem;
	}

	.echo-meta {
		display: flex;
		align-items: center;
	}

	.sense-badge {
		font-size: 0.72rem;
		color: var(--text-muted);
	}

	.echo-intensity {
		display: flex;
		gap: 0.25rem;
		margin-top: 0.1rem;
	}

	.dot {
		width: 7px;
		height: 7px;
		border-radius: 50%;
		border: 1.5px solid var(--border-color);
		background: transparent;
		transition: background 0.15s, border-color 0.15s;
	}
	.dot.filled {
		background: var(--accent);
		border-color: var(--accent);
	}

	.echo-note {
		font-size: 0.8rem;
		color: var(--text-secondary);
		margin: 0.25rem 0 0;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		line-clamp: 2;
		overflow: hidden;
	}

	/* Load more */
	.load-more {
		width: 100%;
		margin-top: 0.5rem;
		padding: 0.75rem;
		background: var(--bg-surface);
		border: 1px solid var(--border-color);
		border-radius: 10px;
		color: var(--text-muted);
		font-size: 0.85rem;
		cursor: pointer;
		transition: color 0.15s, border-color 0.15s;
	}
	.load-more:hover { color: var(--accent); border-color: var(--accent); }
</style>
